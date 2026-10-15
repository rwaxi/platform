#![no_std]
//! Regulation S distribution compliance period module (ADR-007).
//!
//! While `ledger sequence < period_end_ledger`, rejects any transfer or mint
//! whose recipient has a restricted U.S. country relation in the token's
//! identity registry storage. After the period ends, it allows everything.
//!
//! It blocks through the read-only `can_transfer` / `can_create` hooks, so it
//! must be registered on `CanTransfer` and `CanCreate`. Forced and recovery
//! transfers do not go through those hooks and are therefore exempt.

mod storage;

#[cfg(test)]
mod test;

use soroban_sdk::{contract, contracterror, contractimpl, Address, Env, String, Vec};
use stellar_tokens::rwa::compliance::modules::storage as base;
use stellar_tokens::rwa::compliance::modules::ComplianceModule;

pub use storage::RelationKind;
use storage::{
    active, get_owner, get_period_end, get_restricted, is_us_person, set_owner, set_period_end,
    set_restricted, US_COUNTRY_CODE,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum RegSError {
    /// `set_restricted_relations` was called with an empty list.
    EmptyRelations = 410,
}

#[contract]
pub struct RegSDistributionPeriod;

#[contractimpl]
impl RegSDistributionPeriod {
    pub fn __constructor(e: &Env, owner: Address, compliance: Address) {
        set_owner(e, &owner);
        base::set_compliance_address(e, &compliance);
    }

    /// Sets the identity registry storage the module reads country data from.
    pub fn set_identity_registry_storage(e: &Env, token: Address, irs: Address) {
        get_owner(e).require_auth();
        base::set_irs_address(e, &token, &irs);
    }

    /// Sets the first ledger sequence at which the restriction no longer applies.
    pub fn set_period_end(e: &Env, token: Address, period_end_ledger: u32) {
        get_owner(e).require_auth();
        set_period_end(e, &token, period_end_ledger);
    }

    /// Sets which country relations count as "U.S. person". Default: residence only.
    pub fn set_restricted_relations(e: &Env, token: Address, relations: Vec<RelationKind>) {
        get_owner(e).require_auth();
        if relations.is_empty() {
            soroban_sdk::panic_with_error!(e, RegSError::EmptyRelations);
        }
        set_restricted(e, &token, &relations);
    }

    /// Ledger sequence at which the period ends, or 0 if it was never set.
    pub fn period_end(e: &Env, token: Address) -> u32 {
        get_period_end(e, &token).unwrap_or(0)
    }

    /// True while the restriction applies for `token`.
    pub fn is_active(e: &Env, token: Address) -> bool {
        active(e, &token)
    }

    pub fn restricted_relations(e: &Env, token: Address) -> Vec<RelationKind> {
        get_restricted(e, &token)
    }

    pub fn restricted_country(_e: &Env) -> u32 {
        US_COUNTRY_CODE
    }
}

fn allowed(e: &Env, to: &Address, token: &Address) -> bool {
    !active(e, token) || !is_us_person(e, token, to)
}

#[contractimpl(contracttrait)]
impl ComplianceModule for RegSDistributionPeriod {
    fn on_transfer(_e: &Env, _from: Address, _to: Address, _amount: i128, _token: Address) {}

    fn on_created(_e: &Env, _to: Address, _amount: i128, _token: Address) {}

    fn on_destroyed(_e: &Env, _from: Address, _amount: i128, _token: Address) {}

    fn can_transfer(e: &Env, _from: Address, to: Address, _amount: i128, token: Address) -> bool {
        allowed(e, &to, &token)
    }

    fn can_create(e: &Env, to: Address, _amount: i128, token: Address) -> bool {
        allowed(e, &to, &token)
    }

    fn name(e: &Env) -> String {
        base::module_name(e, "RegSDistributionPeriodModule")
    }

    fn get_compliance_address(e: &Env) -> Address {
        base::get_compliance_address(e)
    }

    fn set_compliance_address(e: &Env, compliance: Address) {
        get_owner(e).require_auth();
        base::set_compliance_address(e, &compliance);
    }
}
