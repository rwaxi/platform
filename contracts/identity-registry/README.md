# identity-registry

## What it does

Links each investor wallet to its identity contract and stores the investor's country data (residence, citizenship, tax residency). The identity verifier reads it to check a wallet, and the Reg S module reads the country data to decide whether the recipient is a U.S. person. It also records wallet recoveries.

## Origin

Adapted from OpenZeppelin stellar-contracts v0.7.2, `examples/rwa/identity-registry` (MIT, see [NOTICE](../../NOTICE)). One change: we export `get_country_data_entries` and `get_country_data`. The example does not, and without them any compliance module that reads countries fails with "unknown contract function".

## Public functions

| Function | Who can call | What it does |
|---|---|---|
| `add_identity(account, identity, initial_profiles, operator)` | manager | Registers a wallet with its identity contract and country data |
| `modify_identity`, `remove_identity` | manager | Changes or removes that link |
| `recover_identity(old_account, new_account, operator)` | manager | Moves the identity to a new wallet (lost device). The token's `recover_balance` needs this first |
| `add_country_data_entries`, `modify_country_data`, `delete_country_data` | manager | Changes an investor's country data |
| `bind_token`, `bind_tokens`, `unbind_token` | manager | Says which tokens use this registry |
| `stored_identity`, `get_recovered_to`, `get_country_data_entries`, `get_country_data`, `linked_tokens`, `get_token_index` | anyone | Read state |

This contract does not export the access control functions, same as the OpenZeppelin example. The admin and manager are fixed at deployment. Being able to rotate the manager is a T1 item.

## Errors

| Code | Name | When |
|---|---|---|
| 320 | IdentityOverwrite | The wallet already has an identity |
| 321 | IdentityNotFound | The wallet has none |
| 322 | CountryDataNotFound | Bad index |
| 323 | EmptyCountryList | Registering with no country data |
| 324 | MaxCountryEntriesReached | Too many entries |
| 325 | AccountRecovered | The wallet was already recovered to another one |
| 326, 327 | Metadata... | Metadata too large |
| 330–334 | TokenBinderError | Binding errors |
| 2000–2010 | AccessControlError | Caller lacks the role |

## Testnet

[`CAEK3YJSWNYRNJ7GJRJAQOWZPARJXRKP4KCW7TTP6KTIUH7GYO7ZL2HD`](https://stellar.expert/explorer/testnet/contract/CAEK3YJSWNYRNJ7GJRJAQOWZPARJXRKP4KCW7TTP6KTIUH7GYO7ZL2HD)
