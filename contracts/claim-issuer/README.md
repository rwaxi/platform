# claim-issuer

## What it does

Represents the party that certifies investors (the KYC provider, in production). It keeps the Ed25519 public keys allowed to sign claims, per topic and registry, and checks a claim when the identity verifier asks. A claim is valid if the scheme is Ed25519 (101), the key is allowed for the topic, the claim has not expired and has not been revoked, and the signature matches.

In the spike the signing key is ours and claims are signed with [`tools/sign-claim`](../../tools/sign-claim). There is no real KYC provider yet.

## Origin

Adapted from OpenZeppelin stellar-contracts v0.7.2, `examples/rwa/claim-issuer` (MIT, see [NOTICE](../../NOTICE)). We changed nothing in the logic.

## Public functions

| Function | Who can call | What it does |
|---|---|---|
| `allow_key(public_key, registry, claim_topic)` | owner | Allows a signing key for a topic and registry |
| `remove_key(public_key, registry, claim_topic)` | owner | Removes it |
| `is_claim_valid(identity, claim_topic, scheme, sig_data, claim_data)` | anyone (read-only) | Fails if the claim is not valid |
| `is_key_allowed(public_key, registry, claim_topic)` | anyone | Read state |
| `get_owner`, `transfer_ownership`, `accept_ownership`, `renounce_ownership` | owner for changes, anyone for reads | OpenZeppelin ownable, two-step transfer |

## Errors

| Code | Name | When |
|---|---|---|
| 350 | SigDataMismatch | Scheme is not Ed25519 or signature data is malformed |
| 351–353 | KeyIsEmpty, KeyAlreadyAllowed, KeyNotFound | Key management errors |
| 354 | NotAllowed | Key not allowed for the topic, claim expired or revoked |
| 355, 356 | LimitExceeded, NoKeysForTopic | Limits |
| 357 | InvalidClaimDataExpiration | Bad expiry data in the claim |
| 2100–2102 | OwnableError | Caller is not the owner |

## Testnet

[`CDHJDOHRHNPF4HGZR4YFB5RE7W7MBTNDNYYVMVIRHJHPKGYAECYDXO3Q`](https://stellar.expert/explorer/testnet/contract/CDHJDOHRHNPF4HGZR4YFB5RE7W7MBTNDNYYVMVIRHJHPKGYAECYDXO3Q)
