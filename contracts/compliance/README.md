# compliance

## What it does

Keeps the list of rule modules for each hook and runs them. The token asks `can_transfer` before a transfer and `can_create` before a mint; if any module registered on that hook returns false, the token refuses. After the operation, the token calls `transferred`, `created` or `destroyed` so modules can update their own state. One compliance contract can serve several tokens.

## Origin

Adapted from OpenZeppelin stellar-contracts v0.7.2, `examples/rwa/compliance` (MIT, see [NOTICE](../../NOTICE)). We changed nothing in the logic.

## Public functions

| Function | Who can call | What it does |
|---|---|---|
| `add_module_to(hook, module, operator)` | manager | Registers a module on a hook (`CanTransfer`, `CanCreate`, `Transferred`, `Created`, `Destroyed`) |
| `remove_module_from(hook, module, operator)` | manager | Removes it |
| `bind_token`, `bind_tokens`, `unbind_token` | manager | Says which tokens may use this contract |
| `can_transfer(from, to, amount, token)`, `can_create(to, amount, token)` | anyone (read-only) | Asks every module on the hook; true only if all agree |
| `transferred`, `created`, `destroyed` | a bound token only | Notifies the modules after the operation |
| `get_modules_for_hook`, `is_module_registered`, `linked_tokens`, `get_token_index` | anyone | Read state |
| Access control functions (`grant_role`, `revoke_role`, `get_admin`, ...) | admin for changes, anyone for reads | OpenZeppelin access control |

In our deployment the Reg S module is registered on `CanTransfer` and `CanCreate`.

## Errors

| Code | Name | When |
|---|---|---|
| 360 | ModuleAlreadyRegistered | The module is already on that hook |
| 361 | ModuleNotRegistered | Removing a module that is not there |
| 362 | ModuleBoundExceeded | Too many modules on one hook |
| 363 | TokenNotBound | A notification came from a token that is not bound |
| 330–334 | TokenBinderError | Binding errors (not found, already bound, batch too large or with duplicates) |
| 2000–2010 | AccessControlError | Caller lacks the role |

## Testnet

[`CAXPI7GCY7ZZJ3CNIVQA5A76I6TYC6NBNS44M4USMBNG32Y4Z6QZWNEM`](https://stellar.expert/explorer/testnet/contract/CAXPI7GCY7ZZJ3CNIVQA5A76I6TYC6NBNS44M4USMBNG32Y4Z6QZWNEM)
