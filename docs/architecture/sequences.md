# Critical Flows

## 1. Compliance-checked transfer (the core claim — spike evidence)

Investor A (Colombia, Reg S purchaser) tries to send tokens to B (US resident) during the Reg S distribution compliance period, then to C (Colombia).

```mermaid
sequenceDiagram
  autonumber
  participant A as A · smart account (CO)
  participant T as rwa-token
  participant V as identity-verifier
  participant C as compliance dispatcher
  participant R as reg-s-distribution-period
  participant S as identity-registry-storage

  A->>T: transfer(A, B, 100)
  T->>T: require_auth(A) · not paused · not frozen · free balance ≥ 100
  T->>V: verify_identity(A), verify_identity(B)
  V-->>T: ok (both hold valid KYC claims)
  T->>T: move balances (state change)
  T->>C: on_transfer(snapA, snapB, 100, Standard, token)
  C->>R: on_transfer(...)
  R->>S: country data of B
  S-->>R: Residence(840 · US)
  R-->>C: panic RegSRecipientUSPerson
  C-->>T: revert
  T-->>A: transaction FAILED · whole operation reverted
  Note over A,T: Public hash #1 on Stellar Expert (blocked)

  A->>T: transfer(A, C, 100)
  T->>V: verify_identity(A), verify_identity(C)
  V-->>T: ok
  T->>C: on_transfer(...)
  C->>R: on_transfer(...)
  R->>S: country data of C
  S-->>R: Residence(170 · CO)
  R-->>C: ok
  T-->>A: success
  Note over A,T: Public hash #2 on Stellar Expert (allowed)
```

Why it matters: the rule cannot be bypassed by the operator's database, the API or a third-party wallet — any path to the token hits the same hook.

## 2. Onboarding: passkey → KYC → on-chain claim

```mermaid
sequenceDiagram
  autonumber
  actor I as Investor
  participant App as Investor app
  participant API as Platform API
  participant K as KYC service
  participant P as KYC provider
  participant KMS as KMS
  participant Rel as OZ Relayer
  participant ID as Identity stack

  I->>App: Create account
  App->>App: WebAuthn create (passkey in secure enclave)
  App->>API: public key
  API->>Rel: deploy smart account (sponsored)
  Rel-->>API: account address
  I->>App: Start verification
  App->>K: create session
  K->>P: session
  I->>P: documents + liveness
  P-->>K: webhook: approved, country, investor type
  K->>KMS: sign claim(topic, identity, expiry)
  K->>Rel: add_identity(wallet, identity, country data) + add_claim
  Rel->>ID: invoke
  ID-->>K: events
  K-->>App: verified — wallet ready
```

No seed phrase is ever created or shown.

## 3. Subscription (T2)

```mermaid
sequenceDiagram
  autonumber
  participant SA as Investor smart account
  participant SUB as subscription
  participant V as identity-verifier
  participant U as USDC SAC
  participant T as rwa-token
  participant Op as Agent (backoffice)

  SA->>SUB: subscribe(1000 USDC)
  SUB->>V: verify_identity(SA)
  SUB->>U: transfer SA → escrow
  Op->>SUB: close()
  alt soft cap reached
    SUB->>T: mint(SA, units) — compliance on_created runs per investor
    SUB->>U: escrow → vehicle treasury
  else soft cap missed
    SA->>SUB: refund()
    SUB->>U: escrow → SA
  end
```

## 4. Distribution (T2)

```mermaid
sequenceDiagram
  autonumber
  participant Op as Agent
  participant D as distribution
  participant K as balance-checkpoint
  participant U as USDC SAC
  participant SA as Holder smart account

  Op->>D: deposit(50,000 USDC)
  D->>K: freeze epoch n
  D->>U: agent → distribution
  SA->>D: claim(n)
  D->>K: balance_at(SA, n), supply_at(n)
  D->>U: pro-rata amount → SA
  SA->>D: claim(n) again
  D-->>SA: error AlreadyClaimed
```

## 5. Lost device recovery

```mermaid
sequenceDiagram
  autonumber
  actor I as Investor
  participant K as KYC service
  participant Op as Agent (multisig on mainnet)
  participant T as rwa-token
  participant ID as Identity stack

  I->>K: re-verify identity on new device (new passkey, new smart account)
  K->>ID: link new wallet to same identity
  Op->>T: recover_balance(old, new)
  T->>T: move balance · TransferKind::Recovery
  Note over T: lockup state moves with the investor, policy modules exempt
```
