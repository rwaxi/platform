# Error codes

> Status: **v0.1** · 2026-10-20 · covers the contracts in `contracts/` at OpenZeppelin `stellar-tokens` 0.7.2.

When a call to one of our contracts fails, the transaction returns `Error(Contract, #N)`. This page lists every `N` a user, an operator or the API can get from the contracts in [`contracts/`](../contracts), ordered by code. Each contract's README lists its own errors; this page puts them together.

An error can come from a contract you did not call. The token calls the compliance dispatcher, the identity verifier and the identity registry, so a transfer can fail with an error of any of them. The **Contract** column names the contract that raises the error, and **Seen on** the calls where it reaches the user.

A caller that lacks the right signature does not get a code from this list. The host rejects the call with `Error(Auth, InvalidAction)` before the contract runs.

Every number was checked in the source. **Source** paths are relative to `src/` of the crate named in the section header: `stellar-tokens-0.7.2`, `stellar-access-0.7.2` or `stellar-contract-utils-0.7.2` in `~/.cargo/registry/src/*/`, or our own module in [`contracts/reg-s-distribution-period/src/lib.rs`](../contracts/reg-s-distribution-period/src/lib.rs).

## Token: transfers, mint and supply

| Code | Name | Contract | Seen on | When | What to do | Source |
|---|---|---|---|---|---|---|
| 100 | InsufficientBalance | `rwa-token` | almost never | Last check when balances are written. The token checks first and returns 303 (transfers) or 300 (forced transfer, burn), so this one is a safety net | Lower the amount to the balance | tokens `fungible/mod.rs:284` |
| 101 | InsufficientAllowance | `rwa-token` | `transfer_from` | The spender's allowance is below the amount | Ask the holder for a larger `approve` | tokens `fungible/mod.rs:287` |
| 102 | InvalidLiveUntilLedger | `rwa-token` | `approve` | The allowance expiry ledger is not valid for that amount | Use a ledger after the current one | tokens `fungible/mod.rs:290` |
| 103 | LessThanZero | `rwa-token` | `approve`, `transfer_from`, `transfer`, `mint` | The amount is negative | Send a positive amount | tokens `fungible/mod.rs:292` |
| 104 | MathOverflow | `rwa-token` | `mint` | The total supply would overflow `i128` | Check the amount; it is far too large | tokens `fungible/mod.rs:294` |
| 300 | InsufficientBalance | `rwa-token` | `forced_transfer`, `burn`, `freeze_partial_tokens` | The wallet holds fewer units than the amount | Lower the amount to the balance | tokens `rwa/mod.rs:472` |
| 301 | LessThanZero | `rwa-token` | `freeze_partial_tokens`, `unfreeze_partial_tokens` | The amount is negative | Send a positive amount | tokens `rwa/mod.rs:474` |
| 302 | AddressFrozen | `rwa-token` | `transfer`, `transfer_from` | The sender or the recipient is frozen | An operator must call `set_address_frozen(wallet, false)` if the freeze no longer applies | tokens `rwa/mod.rs:476` |
| 303 | InsufficientFreeTokens | `rwa-token` | `transfer`, `transfer_from`, `unfreeze_partial_tokens` | The amount is above the balance that is not frozen. This is also what a plain transfer with too little balance returns | Lower the amount, or an operator unfreezes part of the balance | tokens `rwa/mod.rs:478` |
| 304 | IdentityVerificationFailed | `identity-verifier` | `transfer`, `transfer_from`, `mint` | The wallet has an identity, but no valid KYC claim from a trusted issuer: missing, expired, revoked or from an issuer that is no longer trusted | Renew the investor's KYC claim with the claim issuer | tokens `rwa/mod.rs:480` |
| 305 | TransferNotCompliant | `rwa-token` | `transfer`, `transfer_from` | A compliance module said no. Today the only one is Reg S: a U.S. recipient while the period is active | Wait for the period to end, or choose a non-U.S. recipient. `forced_transfer` and `recover_balance` do not go through the modules | tokens `rwa/mod.rs:482` |
| 306 | MintNotCompliant | `rwa-token` | `mint` | A compliance module said no. Reg S: minting to a U.S. investor while the period is active | Same as 305 | tokens `rwa/mod.rs:484` |
| 307 | ComplianceNotSet | `rwa-token` | any transfer or mint | The token has no compliance contract | Deployment error: call `set_compliance` | tokens `rwa/mod.rs:486` |
| 308 | OnchainIdNotSet | `rwa-token` | `onchain_id` | The token has no on-chain identity | Deployment error | tokens `rwa/mod.rs:488` |
| 309 | VersionNotSet | `rwa-token` | `version` | The token has no version | Deployment error | tokens `rwa/mod.rs:490` |
| 310 | ClaimTopicsAndIssuersNotSet | `identity-verifier` | any transfer or mint | The verifier has no claim topics and issuers contract | Deployment error: call `set_claim_topics_and_issuers` | tokens `rwa/mod.rs:492` |
| 311 | IdentityRegistryStorageNotSet | `identity-verifier` | any transfer or mint | The verifier has no identity registry | Deployment error: call `set_identity_registry_storage` | tokens `rwa/mod.rs:494` |
| 312 | IdentityVerifierNotSet | `rwa-token` | any transfer or mint | The token has no identity verifier | Deployment error: call `set_identity_verifier` | tokens `rwa/mod.rs:496` |
| 313 | IdentityMismatch | `rwa-token` | `recover_balance` | The new wallet was not set as the recovery target of the old one in the identity registry | Call `recover_identity(old, new)` in the registry first | tokens `rwa/mod.rs:498` |

## Identity registry

| Code | Name | Contract | Seen on | When | What to do | Source |
|---|---|---|---|---|---|---|
| 320 | IdentityOverwrite | `identity-registry` | `add_identity`, `recover_identity` | The wallet already has an identity | Use `modify_identity`, or pick a wallet without one | tokens `rwa/identity_verification/identity_registry_storage/mod.rs:574` |
| 321 | IdentityNotFound | `identity-registry` | `transfer`, `transfer_from`, `mint`, and every registry call on that wallet | The wallet has no identity. A transfer to an unregistered wallet fails with 321, not 304 | Onboard the investor: create the identity and call `add_identity` | `…/identity_registry_storage/mod.rs:576` |
| 322 | CountryDataNotFound | `identity-registry` | `modify_country_data`, `delete_country_data`, `get_country_data` | No country entry at that index | Read the entries with `get_country_data_entries` and use a valid index | `…/identity_registry_storage/mod.rs:578` |
| 323 | EmptyCountryList | `identity-registry` | `add_identity`, `add_country_data_entries`, `delete_country_data` | The identity would have no country data | Send at least one entry; to replace the last one, add first and delete after | `…/identity_registry_storage/mod.rs:580` |
| 324 | MaxCountryEntriesReached | `identity-registry` | `add_identity`, `add_country_data_entries` | Too many country entries for one identity | Remove entries that no longer apply | `…/identity_registry_storage/mod.rs:582` |
| 325 | AccountRecovered | `identity-registry` | `add_identity`, `recover_identity` | The wallet was already recovered to another one, so it cannot be registered or recovered again | Use the new wallet | `…/identity_registry_storage/mod.rs:584` |
| 326 | MetadataTooManyEntries | `identity-registry` | `add_identity`, `add_country_data_entries`, `modify_country_data` | Too many metadata entries in a country entry | Send fewer entries | `…/identity_registry_storage/mod.rs:586` |
| 327 | MetadataStringTooLong | `identity-registry` | same as 326 | A metadata value is too long | Shorten it | `…/identity_registry_storage/mod.rs:588` |

## Token binding

Raised by `identity-registry` and `compliance` on `bind_token`, `bind_tokens` and `unbind_token`.

| Code | Name | When | What to do | Source |
|---|---|---|---|---|
| 330 | TokenNotFound | Unbinding or looking up a token that is not bound | Check the token address | tokens `rwa/utils/token_binder/mod.rs:94` |
| 331 | TokenAlreadyBound | Binding a token twice | Nothing to do; it is already bound | `…/token_binder/mod.rs:96` |
| 332 | MaxTokensReached | The contract already has the maximum number of bound tokens | Unbind a token that is no longer used | `…/token_binder/mod.rs:98` |
| 333 | BindBatchTooLarge | `bind_tokens` with too many tokens at once | Split the batch | `…/token_binder/mod.rs:100` |
| 334 | BindBatchDuplicates | `bind_tokens` with the same token twice | Remove the duplicate | `…/token_binder/mod.rs:102` |

## Investor identity and claims

| Code | Name | Contract | Seen on | When | What to do | Source |
|---|---|---|---|---|---|---|
| 340 | ClaimNotFound | `investor-identity` | `get_claim`, `remove_claim` | No claim with that ID | Read the IDs with `get_claim_ids_by_topic` | tokens `rwa/identity_verification/identity_claims/mod.rs:90` |
| 350 | SigDataMismatch | `claim-issuer` | `add_claim` on the identity | The scheme is not Ed25519 (101), or the signature data is malformed | Re-sign the claim with `tools/sign-claim` | tokens `rwa/identity_verification/claim_issuer/mod.rs:402` |
| 351 | KeyIsEmpty | `claim-issuer` | `allow_key` | The public key is empty | Send the 32-byte Ed25519 key | `…/claim_issuer/mod.rs:404` |
| 352 | KeyAlreadyAllowed | `claim-issuer` | `allow_key` | The key is already allowed for that topic and registry | Nothing to do | `…/claim_issuer/mod.rs:406` |
| 353 | KeyNotFound | `claim-issuer` | `remove_key` | The key is not allowed for that topic and registry | Check the key and topic | `…/claim_issuer/mod.rs:408` |
| 354 | NotAllowed | `claim-issuer` | `allow_key`, `add_claim` on the identity | On `allow_key`: the issuer is not trusted for that topic. On `add_claim`: the signing key is not allowed, or the claim expired or was revoked. At transfer time this error is caught and the user sees 304 | Trust the issuer for the topic in `claim-topics-and-issuers`, or issue a new claim | `…/claim_issuer/mod.rs:411` |
| 355 | LimitExceeded | `claim-issuer` | `allow_key` | Too many keys for one topic, or too many registries for one key | Remove a key that is no longer used | `…/claim_issuer/mod.rs:413` |
| 357 | InvalidClaimDataExpiration | `claim-issuer` | `add_claim` on the identity | The claim data has no expiry header (shorter than 16 bytes) | Encode the data with the expiry, as `tools/sign-claim` does | `…/claim_issuer/mod.rs:417` |

## Compliance dispatcher

| Code | Name | Contract | Seen on | When | What to do | Source |
|---|---|---|---|---|---|---|
| 360 | ModuleAlreadyRegistered | `compliance` | `add_module_to` | The module is already on that hook | Nothing to do | tokens `rwa/compliance/mod.rs:231` |
| 361 | ModuleNotRegistered | `compliance` | `remove_module_from` | The module is not on that hook | Check the hook and module address | `…/compliance/mod.rs:233` |
| 362 | ModuleBoundExceeded | `compliance` | `add_module_to` | Too many modules on one hook | Remove a module that is no longer used | `…/compliance/mod.rs:235` |
| 363 | TokenNotBound | `compliance` | hook calls from a token | The calling token is not bound to the dispatcher | Call `bind_token` on `compliance` | `…/compliance/mod.rs:237` |

## Claim topics and trusted issuers

| Code | Name | Contract | Seen on | When | What to do | Source |
|---|---|---|---|---|---|---|
| 370 | ClaimTopicDoesNotExist | `claim-topics-and-issuers` | `remove_claim_topic`, `add_trusted_issuer`, `update_issuer_claim_topics` | The topic is not registered | Add it with `add_claim_topic` | tokens `rwa/identity_verification/claim_topics_and_issuers/mod.rs:295` |
| 371 | IssuerDoesNotExist | `claim-topics-and-issuers` | `remove_trusted_issuer`, `update_issuer_claim_topics` | The issuer is not trusted | Add it with `add_trusted_issuer` | `…/claim_topics_and_issuers/mod.rs:297` |
| 372 | ClaimTopicAlreadyExists | `claim-topics-and-issuers` | `add_claim_topic`, `add_trusted_issuer`, `update_issuer_claim_topics` | The topic is already registered, or the issuer's topic list repeats one | Nothing to do | `…/claim_topics_and_issuers/mod.rs:299` |
| 373 | IssuerAlreadyExists | `claim-topics-and-issuers` | `add_trusted_issuer` | The issuer is already trusted | Use `update_issuer_claim_topics` | `…/claim_topics_and_issuers/mod.rs:301` |
| 374 | MaxClaimTopicsLimitReached | `claim-topics-and-issuers` | `add_claim_topic`, `add_trusted_issuer`, `update_issuer_claim_topics` | Too many topics, in the contract or for one issuer | Remove a topic that is no longer used | `…/claim_topics_and_issuers/mod.rs:303` |
| 375 | MaxIssuersLimitReached | `claim-topics-and-issuers` | `add_trusted_issuer` | Too many trusted issuers | Remove an issuer that is no longer used | `…/claim_topics_and_issuers/mod.rs:305` |
| 376 | ClaimTopicsSetCannotBeEmpty | `claim-topics-and-issuers` | `add_trusted_issuer`, `update_issuer_claim_topics` | An issuer with no topics | Send at least one topic | `…/claim_topics_and_issuers/mod.rs:307` |

## Reg S module

| Code | Name | Contract | Seen on | When | What to do | Source |
|---|---|---|---|---|---|---|
| 397 | IdentityRegistryNotSet | `reg-s-distribution-period` | `transfer`, `mint` of the token | The module has no identity registry for that token, so it cannot read the recipient's country | Call `set_identity_registry_storage(token, registry)` on the module | tokens `rwa/compliance/modules/mod.rs:235` |
| 399 | ComplianceAlreadySet | `reg-s-distribution-period` | `set_compliance_address` | The dispatcher is set in the constructor and cannot be changed | Deploy a new module if the dispatcher changes | tokens `rwa/compliance/modules/mod.rs:239` |
| 410 | EmptyRelations | `reg-s-distribution-period` | `set_restricted_relations` | The list of relations is empty | Send at least `Residence` | [`lib.rs:32`](../contracts/reg-s-distribution-period/src/lib.rs) |

The module never returns 305 or 306 itself. It answers `false` and the token raises the error.

## Pause

| Code | Name | Contract | Seen on | When | What to do | Source |
|---|---|---|---|---|---|---|
| 1000 | EnforcedPause | `rwa-token` | `transfer`, `transfer_from`, `pause` | The token is paused, or `pause` was called on a paused token | The admin calls `unpause` when the incident is closed | contract-utils `pausable/mod.rs:134` |
| 1001 | ExpectedPause | `rwa-token` | `unpause` | The token is not paused | Nothing to do | contract-utils `pausable/mod.rs:136` |

## Roles (`rwa-token`, `compliance`, `identity-registry`, `identity-verifier`, `claim-topics-and-issuers`)

| Code | Name | When | What to do | Source |
|---|---|---|---|---|
| 2000 | Unauthorized | The `operator` passed to the call does not hold the role (`manager` for token and registry operations) | Grant the role with `grant_role`, or send the call from a wallet that has it | access `access_control/mod.rs:384` |
| 2001 | AdminNotSet | An admin call, `set_role_admin` or `accept_admin_transfer` on a contract with no admin | Deployment error | `…/access_control/mod.rs:385` |
| 2002 | IndexOutOfBounds | `get_role_member` with an index past the end | Read `get_role_member_count` first | `…/access_control/mod.rs:386` |
| 2003 | AdminRoleNotFound | Removing the admin role of a role that has none | Check the role name | `…/access_control/mod.rs:387` |
| 2004 | RoleCountIsNotZero | Internal check when the last member of a role is removed | Report it; it should not happen | `…/access_control/mod.rs:388` |
| 2005 | RoleNotFound | Internal check when the last member of a role that was never counted is removed | Report it; it should not happen | `…/access_control/mod.rs:389` |
| 2006 | AdminAlreadySet | Setting the admin a second time | Use the two-step admin transfer | `…/access_control/mod.rs:390` |
| 2007 | RoleNotHeld | Revoking or renouncing a role the account does not hold | Check the account and role | `…/access_control/mod.rs:391` |
| 2008 | RoleIsEmpty | Revoking a member of a role that has no members | Check the account and role | `…/access_control/mod.rs:392` |
| 2009 | TransferInProgress | Renouncing the admin role while an admin transfer is pending | Finish or cancel the transfer first | `…/access_control/mod.rs:393` |
| 2010 | MaxRolesExceeded | `grant_role` with a new role when the contract already has the maximum | Use fewer roles | `…/access_control/mod.rs:394` |

## Owner (`claim-issuer`, `investor-identity`)

These contracts check that the caller is the owner with the owner's signature, so a wrong caller gets the auth error from the host, not a code.

| Code | Name | When | What to do | Source |
|---|---|---|---|---|
| 2100 | OwnerNotSet | An owner-only call on a contract without an owner (after `renounce_ownership`) | The contract cannot be administered any more; deploy a new one | access `ownable/mod.rs:139` |
| 2101 | TransferInProgress | `renounce_ownership` while an ownership transfer is pending | Finish or cancel the transfer first | `…/ownable/mod.rs:140` |
| 2102 | OwnerAlreadySet | Setting the owner a second time | Use `transfer_ownership` | `…/ownable/mod.rs:141` |

## Two-step transfers (owner and admin)

| Code | Name | When | What to do | Source |
|---|---|---|---|---|
| 2200 | NoPendingTransfer | Accepting, or cancelling (expiry ledger 0), when no transfer is pending | Start the transfer first | access `role_transfer/mod.rs:14` |
| 2201 | InvalidLiveUntilLedger | The transfer's expiry ledger is before the current ledger or after the maximum the network allows | Use a ledger within that window | `…/role_transfer/mod.rs:15` |
| 2202 | InvalidPendingAccount | Cancelling a transfer with an address that is not the pending one | Cancel with the address named in the transfer | `…/role_transfer/mod.rs:16` |
| 2203 | TransferExpired | Accepting a transfer after its expiry ledger | Start a new transfer | `…/role_transfer/mod.rs:17` |

## Codes left out

Some codes exist in the OpenZeppelin source but our contracts cannot return them: `ClaimNotValid` (341) is declared and never raised in 0.7.2; `NoKeysForTopic` (356) and `MathOverflow` (359) come from claim issuer functions our `claim-issuer` does not expose; the other Reg S module codes (390 to 400) belong to OpenZeppelin's example modules; `FungibleTokenError` 105 to 114 cover metadata, caps, the Stellar Asset Contract and allow/block lists, none of which our token uses.
