# identity-verifier

## What it does

Answers one question for the token: does this wallet have a valid identity? It looks up the wallet's identity contract in the identity registry, then checks that the identity holds a valid claim for every required topic, signed by a trusted issuer listed in claim-topics-and-issuers. If not, the call fails with 304 and the token refuses the operation.

## Origin

Adapted from OpenZeppelin stellar-contracts v0.7.2, `examples/rwa/identity-verifier` (MIT, see [NOTICE](../../NOTICE)). We changed nothing in the logic.

## Public functions

| Function | Who can call | What it does |
|---|---|---|
| `verify_identity(account)` | anyone (read-only) | Fails with 304 if the wallet has no identity or no valid claim |
| `recovery_target(old_account)` | anyone | The wallet an account was recovered to, if any |
| `set_identity_registry_storage(identity_registry_storage, operator)` | admin | Points to another registry |
| `set_claim_topics_and_issuers(claim_topics_and_issuers, operator)` | admin | Points to another list of topics and issuers |
| `identity_registry_storage`, `claim_topics_and_issuers` | anyone | Read state |
| Access control functions (`grant_role`, `revoke_role`, `get_admin`, ...) | admin for changes, anyone for reads | OpenZeppelin access control |

## Errors

| Code | Name | When |
|---|---|---|
| 304 | IdentityVerificationFailed | No identity, missing claim, issuer not trusted or claim not valid |
| 310, 311 | ClaimTopicsAndIssuersNotSet, IdentityRegistryStorageNotSet | Configuration missing |
| 2000–2010 | AccessControlError | Caller lacks the role |

## Testnet

[`CDVOLAVFM7ZD4BRFJKI3YJY5PJ2JNW4T6MH7ALMBJR3ZNUMIGG45S5NF`](https://stellar.expert/explorer/testnet/contract/CDVOLAVFM7ZD4BRFJKI3YJY5PJ2JNW4T6MH7ALMBJR3ZNUMIGG45S5NF)
