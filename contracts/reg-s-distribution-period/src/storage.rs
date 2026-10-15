use soroban_sdk::{contractevent, contracttype, Address, Env, Vec};
use stellar_tokens::rwa::compliance::modules::{
    storage as base, MODULE_EXTEND_AMOUNT, MODULE_TTL_THRESHOLD,
};
use stellar_tokens::rwa::identity_registry_storage::{
    CountryRelation, IndividualCountryRelation, OrganizationCountryRelation,
};

/// ISO 3166-1 numeric code for the United States.
pub const US_COUNTRY_CODE: u32 = 840;

/// Which identity country relation makes a recipient a "U.S. person".
/// The mapping to Reg S definitions is counsel's call (ADR-007).
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelationKind {
    /// Individual residence, or organization incorporation / operating jurisdiction.
    Residence,
    Citizenship,
    /// Individual tax residency, or organization tax jurisdiction.
    TaxResidency,
}

#[contracttype]
#[derive(Clone)]
enum Key {
    Owner,
    PeriodEnd(Address),
    Restricted(Address),
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PeriodEndSet {
    #[topic]
    pub token: Address,
    pub period_end_ledger: u32,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RestrictedRelationsSet {
    #[topic]
    pub token: Address,
    pub relations: Vec<RelationKind>,
}

pub fn set_owner(e: &Env, owner: &Address) {
    e.storage().instance().set(&Key::Owner, owner);
}

pub fn get_owner(e: &Env) -> Address {
    e.storage()
        .instance()
        .get(&Key::Owner)
        .expect("owner not set")
}

pub fn get_period_end(e: &Env, token: &Address) -> Option<u32> {
    let key = Key::PeriodEnd(token.clone());
    let v = e.storage().persistent().get::<_, u32>(&key);
    if v.is_some() {
        e.storage()
            .persistent()
            .extend_ttl(&key, MODULE_TTL_THRESHOLD, MODULE_EXTEND_AMOUNT);
    }
    v
}

pub fn set_period_end(e: &Env, token: &Address, ledger: u32) {
    let key = Key::PeriodEnd(token.clone());
    e.storage().persistent().set(&key, &ledger);
    e.storage()
        .persistent()
        .extend_ttl(&key, MODULE_TTL_THRESHOLD, MODULE_EXTEND_AMOUNT);
    PeriodEndSet {
        token: token.clone(),
        period_end_ledger: ledger,
    }
    .publish(e);
}

pub fn get_restricted(e: &Env, token: &Address) -> Vec<RelationKind> {
    let key = Key::Restricted(token.clone());
    match e.storage().persistent().get::<_, Vec<RelationKind>>(&key) {
        Some(v) => {
            e.storage()
                .persistent()
                .extend_ttl(&key, MODULE_TTL_THRESHOLD, MODULE_EXTEND_AMOUNT);
            v
        }
        None => Vec::from_array(e, [RelationKind::Residence]),
    }
}

pub fn set_restricted(e: &Env, token: &Address, relations: &Vec<RelationKind>) {
    let key = Key::Restricted(token.clone());
    e.storage().persistent().set(&key, relations);
    e.storage()
        .persistent()
        .extend_ttl(&key, MODULE_TTL_THRESHOLD, MODULE_EXTEND_AMOUNT);
    RestrictedRelationsSet {
        token: token.clone(),
        relations: relations.clone(),
    }
    .publish(e);
}

/// The restriction applies only once a period end is configured and not yet reached.
pub fn active(e: &Env, token: &Address) -> bool {
    match get_period_end(e, token) {
        Some(end) => e.ledger().sequence() < end,
        None => false,
    }
}

fn kind_and_code(rel: &CountryRelation) -> Option<(RelationKind, u32)> {
    match rel {
        CountryRelation::Individual(r) => match r {
            IndividualCountryRelation::Residence(c) => Some((RelationKind::Residence, *c)),
            IndividualCountryRelation::Citizenship(c) => Some((RelationKind::Citizenship, *c)),
            IndividualCountryRelation::TaxResidency(c) => Some((RelationKind::TaxResidency, *c)),
            _ => None,
        },
        CountryRelation::Organization(r) => match r {
            OrganizationCountryRelation::Incorporation(c)
            | OrganizationCountryRelation::OperatingJurisdiction(c) => {
                Some((RelationKind::Residence, *c))
            }
            OrganizationCountryRelation::TaxJurisdiction(c) => {
                Some((RelationKind::TaxResidency, *c))
            }
            _ => None,
        },
    }
}

/// True if any restricted relation of `account` points to the U.S.
/// Fails closed: an account with no country data is treated as restricted,
/// because the module cannot show the recipient is not a U.S. person.
pub fn is_us_person(e: &Env, token: &Address, account: &Address) -> bool {
    let entries = base::get_irs_country_data_entries(e, token, account);
    if entries.is_empty() {
        return true;
    }
    let restricted = get_restricted(e, token);
    for entry in entries.iter() {
        if let Some((kind, code)) = kind_and_code(&entry.country) {
            if code == US_COUNTRY_CODE && restricted.contains(kind) {
                return true;
            }
        }
    }
    false
}
