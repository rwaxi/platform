# rwa-token

## What it does

The fund's security token. Every transfer and mint first asks the identity verifier whether both wallets have a valid identity, then asks the compliance contract whether the rules allow it. If either says no, nothing moves. Units have 7 decimals.

## Origin

Adapted from OpenZeppelin stellar-contracts v0.7.2, `examples/rwa/token` (MIT, see [NOTICE](../../NOTICE)). We changed nothing in the logic.

## Public functions

Roles: **admin** and **manager** are set in the constructor. The manager role is checked through the `operator` argument.

| Function | Who can call | What it does |
|---|---|---|
| `transfer(from, to, amount)` | holder (`from`) | Moves units. Fails if paused, if either address is frozen, if either has no valid identity (304) or if compliance refuses (305) |
| `approve`, `transfer_from` | holder / spender | Standard allowance. `transfer_from` runs the same checks as `transfer` |
| `mint(to, amount, operator)` | manager | Issues units. Checks the recipient's identity and asks compliance `can_create` (306 if refused) |
| `burn(user_address, amount, operator)` | manager | Destroys units |
| `forced_transfer(from, to, amount, operator)` | manager | Moves units without the holder's signature, for court orders or corrections. Skips pause, freeze and the `can_transfer` check, so the Reg S module does not run |
| `recover_balance(old_account, new_account, operator)` | manager | Moves the whole balance to a new wallet after the identity registry recorded the recovery. Keeps frozen status |
| `set_address_frozen`, `freeze_partial_tokens`, `unfreeze_partial_tokens` | manager | Blocks a wallet, or part of its balance |
| `set_compliance`, `set_identity_verifier` | manager | Points the token to another compliance or verifier contract |
| `pause`, `unpause` | admin | Stops or resumes all transfers |
| `balance`, `total_supply`, `allowance`, `name`, `symbol`, `decimals`, `paused`, `is_frozen`, `get_frozen_tokens`, `compliance`, `identity_verifier` | anyone | Read state |
| `onchain_id`, `version` | anyone | Not set in this deployment; they return 308 and 309 |
| `grant_role`, `revoke_role`, `set_role_admin`, `transfer_admin_role`, `accept_admin_transfer`, `renounce_admin`, `renounce_role`, `has_role`, `get_admin`, `get_role_member`, `get_role_member_count`, `get_role_admin`, `get_existing_roles` | admin for changes, anyone for reads | OpenZeppelin access control |

## Errors

| Code | Name | When |
|---|---|---|
| 300 | InsufficientBalance | Not enough units |
| 302 | AddressFrozen | Sender or recipient is frozen |
| 303 | InsufficientFreeTokens | Part of the balance is frozen |
| 304 | IdentityVerificationFailed | A wallet has no identity or no valid claim |
| 305 | TransferNotCompliant | A compliance module refused the transfer (Reg S during the period) |
| 306 | MintNotCompliant | A compliance module refused the mint |
| 307–313 | ...NotSet, IdentityMismatch | Configuration missing or wrong |
| 1000 | EnforcedPause | Token is paused |
| 100–114 | FungibleTokenError | Base token errors (allowance, overflow) |
| 2000–2010 | AccessControlError | Caller lacks the role |

## Testnet

[`CDKWYFYWDFH43BJ3SZBHQ3MCHZHUZFLUA2UE3KI3HN5R6UYKC7FLUIFH`](https://stellar.expert/explorer/testnet/contract/CDKWYFYWDFH43BJ3SZBHQ3MCHZHUZFLUA2UE3KI3HN5R6UYKC7FLUIFH)
