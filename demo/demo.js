// Read-only demo of the spike on Stellar testnet. Nothing is signed or sent:
// every call is a simulation through the public RPC, so no wallet is needed.
// Works in the browser (window.StellarSdk) and in Node (globalThis.StellarSdk).
(function (root) {
  "use strict";
  const RPC_URL = "https://soroban-testnet.stellar.org";
  const UNIT = 10_000_000n; // the token has 7 decimals

  function sdk() {
    if (!root.StellarSdk) throw new Error("StellarSdk is not loaded");
    return root.StellarSdk;
  }

  function server() {
    return new (sdk().rpc.Server)(RPC_URL);
  }

  // Simulates contractId.method(args) with `source` as the transaction source.
  async function simulate(contractId, method, args, source) {
    const S = sdk();
    const rpc = server();
    const account = await rpc.getAccount(source);
    const tx = new S.TransactionBuilder(account, {
      fee: "100",
      networkPassphrase: S.Networks.TESTNET,
    })
      .addOperation(new S.Contract(contractId).call(method, ...args))
      .setTimeout(30)
      .build();
    const sim = await rpc.simulateTransaction(tx);
    if (S.rpc.Api.isSimulationError(sim)) {
      const m = /Error\(Contract, #(\d+)\)/.exec(sim.error || "");
      return { ok: false, code: m ? Number(m[1]) : null, error: sim.error };
    }
    return { ok: true, value: sim.result ? S.scValToNative(sim.result.retval) : null };
  }

  const addr = (a) => sdk().nativeToScVal(a, { type: "address" });
  const i128 = (n) => sdk().nativeToScVal(BigInt(n), { type: "i128" });

  async function latestLedger() {
    return (await server().getLatestLedger()).sequence;
  }

  async function balance(d, who) {
    const r = await simulate(d.contracts.rwa_token, "balance", [addr(who)], d.accounts.alice_colombia);
    return r.ok ? BigInt(r.value) : null;
  }

  async function periodStatus(d) {
    const src = d.accounts.alice_colombia;
    const m = d.contracts.reg_s_distribution_period;
    const [active, end] = await Promise.all([
      simulate(m, "is_active", [addr(d.contracts.rwa_token)], src),
      simulate(m, "period_end", [addr(d.contracts.rwa_token)], src),
    ]);
    return { active: active.ok ? active.value : null, end: end.ok ? Number(end.value) : null };
  }

  // Alice sends `units` whole tokens to `to`. Simulated with Alice as source,
  // so the token runs identity and compliance checks exactly as on a real send.
  async function simulateTransfer(d, to, units) {
    return simulate(
      d.contracts.rwa_token,
      "transfer",
      [addr(d.accounts.alice_colombia), addr(to), i128(BigInt(units) * UNIT)],
      d.accounts.alice_colombia
    );
  }

  const ERRORS = {
    304: "IdentityVerificationFailed: the recipient has no valid identity",
    305: "TransferNotCompliant: a compliance module said no (here, the Reg S module)",
    306: "MintNotCompliant",
  };

  root.RwaxiDemo = { simulate, latestLedger, balance, periodStatus, simulateTransfer, ERRORS, UNIT };
})(typeof window !== "undefined" ? window : globalThis);
