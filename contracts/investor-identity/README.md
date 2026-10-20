# investor-identity

## What it does

One contract per investor. It stores the claims signed about that investor, such as KYC passed. The identity registry links the investor's wallet to this contract, and the identity verifier reads the claims from here. The same WASM is deployed once per investor.

## Origin

Adapted from OpenZeppelin stellar-contracts v0.7.2, `examples/rwa/identity` (MIT, see [NOTICE](../../NOTICE)). We changed nothing in the logic.

## Public functions

| Function | Who can call | What it does |
|---|---|---|
| `add_claim(topic, scheme, issuer, signature, data, uri)` | owner | Adds a claim. It first calls the issuer's `is_claim_valid`, so an invalid claim is not stored and the call fails with the issuer's error (350–357). Returns the claim ID |
| `remove_claim(claim_id)` | owner | Removes a claim |
| `get_claim(claim_id)`, `get_claim_ids_by_topic(topic)` | anyone | Read claims |
| `get_owner`, `transfer_ownership`, `accept_ownership`, `renounce_ownership` | owner for changes, anyone for reads | OpenZeppelin ownable, two-step transfer |

Each identity is owned by the investor's own account, so only the investor can add or remove claims on it.

## Errors

| Code | Name | When |
|---|---|---|
| 340 | ClaimNotFound | No claim with that ID |
| 2100–2102 | OwnableError | Caller is not the owner |

## Testnet

| Investor | Contract |
|---|---|
| Alice (Colombia) | [`CAHT32AY6CJYJVNC3QE7ZPZPHPLEKSOD5WX3F3X63PYKPMWFEP2U5ZRF`](https://stellar.expert/explorer/testnet/contract/CAHT32AY6CJYJVNC3QE7ZPZPHPLEKSOD5WX3F3X63PYKPMWFEP2U5ZRF) |
| Bob (Colombia) | [`CANNATC7RCZEXXEDWOUTWXF4FPQN5MSZV6XPBL3324UXJZODHYUIEKGX`](https://stellar.expert/explorer/testnet/contract/CANNATC7RCZEXXEDWOUTWXF4FPQN5MSZV6XPBL3324UXJZODHYUIEKGX) |
| Carol (United States) | [`CBGJ6URIPLC3H2IBMGVXMOR5PI7M4XV4XOAPJ4K46GNIAXDYGIUN4R4C`](https://stellar.expert/explorer/testnet/contract/CBGJ6URIPLC3H2IBMGVXMOR5PI7M4XV4XOAPJ4K46GNIAXDYGIUN4R4C) |
