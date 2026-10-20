# claim-topics-and-issuers

## What it does

Holds two lists: the claim topics every investor must have (for example, KYC passed) and the issuers we trust to sign each topic. The identity verifier reads both when it checks a wallet.

## Origin

Adapted from OpenZeppelin stellar-contracts v0.7.2, `examples/rwa/claim-topics-and-issuers` (MIT, see [NOTICE](../../NOTICE)). We changed nothing in the logic.

## Public functions

| Function | Who can call | What it does |
|---|---|---|
| `add_claim_topic(claim_topic, operator)`, `remove_claim_topic` | manager | Adds or removes a required topic |
| `add_trusted_issuer(trusted_issuer, claim_topics, operator)`, `remove_trusted_issuer` | manager | Adds or removes an issuer and the topics it may sign |
| `update_issuer_claim_topics(trusted_issuer, claim_topics, operator)` | manager | Changes an issuer's topics |
| `get_claim_topics`, `has_claim_topic`, `get_trusted_issuers`, `is_trusted_issuer`, `get_claim_topic_issuers`, `get_trusted_issuer_claim_topics`, `get_claim_topics_and_issuers` | anyone | Read state |
| Access control functions (`grant_role`, `revoke_role`, `get_admin`, ...) | admin for changes, anyone for reads | OpenZeppelin access control |

## Errors

| Code | Name | When |
|---|---|---|
| 370 | ClaimTopicDoesNotExist | Removing or using a topic that is not there |
| 371 | IssuerDoesNotExist | Same for an issuer |
| 372, 373 | ...AlreadyExists | Adding a topic or issuer twice |
| 374, 375 | Max...LimitReached | Too many topics or issuers |
| 376 | ClaimTopicsSetCannotBeEmpty | An issuer with no topics |
| 2000–2010 | AccessControlError | Caller lacks the role |

## Testnet

[`CCBXT2VD4ZN5HNCTXRBJWFAPWQFSDQEJXGMZFD3DE375ZU6D76GIW43Q`](https://stellar.expert/explorer/testnet/contract/CCBXT2VD4ZN5HNCTXRBJWFAPWQFSDQEJXGMZFD3DE375ZU6D76GIW43Q)
