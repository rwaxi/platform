extern crate std;

use soroban_sdk::{
    contract, contractimpl, contracttype,
    testutils::{Address as _, Ledger, MockAuth, MockAuthInvoke},
    vec, Address, Env, IntoVal, Val, Vec,
};
use stellar_tokens::rwa::identity_registry_storage::{
    CountryData, CountryRelation, IndividualCountryRelation, OrganizationCountryRelation,
};

use crate::{RegSDistributionPeriod, RegSDistributionPeriodClient, RelationKind};

const US: u32 = 840;
const CO: u32 = 170;
const END: u32 = 1_000;

/// Minimal stand-in for the OZ identity registry: only the call the module makes.
#[contract]
struct MockIrs;

#[contracttype]
#[derive(Clone)]
enum IrsKey {
    Entries(Address),
}

#[contractimpl]
impl MockIrs {
    pub fn set_entries(e: &Env, account: Address, entries: Vec<CountryData>) {
        let raw: Vec<Val> = Vec::from_iter(e, entries.iter().map(|c| c.into_val(e)));
        e.storage()
            .persistent()
            .set(&IrsKey::Entries(account), &raw);
    }

    pub fn get_country_data_entries(e: &Env, account: Address) -> Vec<Val> {
        e.storage()
            .persistent()
            .get(&IrsKey::Entries(account))
            .unwrap_or_else(|| Vec::new(e))
    }
}

fn individual(_e: &Env, rel: IndividualCountryRelation) -> CountryData {
    CountryData {
        country: CountryRelation::Individual(rel),
        metadata: None,
    }
}

struct Fixture<'a> {
    e: Env,
    owner: Address,
    token: Address,
    irs: Address,
    module: RegSDistributionPeriodClient<'a>,
}

impl<'a> Fixture<'a> {
    fn new() -> Self {
        let e = Env::default();
        e.mock_all_auths();
        let owner = Address::generate(&e);
        let compliance = Address::generate(&e);
        let token = Address::generate(&e);
        let irs = e.register(MockIrs, ());
        let id = e.register(RegSDistributionPeriod, (&owner, &compliance));
        let module = RegSDistributionPeriodClient::new(&e, &id);
        module.set_identity_registry_storage(&token, &irs);
        module.set_period_end(&token, &END);
        Fixture {
            e,
            owner,
            token,
            irs,
            module,
        }
    }

    fn account(&self, entries: Vec<CountryData>) -> Address {
        let a = Address::generate(&self.e);
        MockIrsClient::new(&self.e, &self.irs).set_entries(&a, &entries);
        a
    }

    fn residing_in(&self, code: u32) -> Address {
        self.account(vec![
            &self.e,
            individual(&self.e, IndividualCountryRelation::Residence(code)),
        ])
    }

    fn at(&self, ledger: u32) {
        self.e.ledger().set_sequence_number(ledger);
    }

    fn can_transfer(&self, to: &Address) -> bool {
        let from = Address::generate(&self.e);
        self.module.can_transfer(&from, to, &1, &self.token)
    }
}

#[test]
fn us_recipient_is_blocked_during_period() {
    let f = Fixture::new();
    f.at(10);
    let us = f.residing_in(US);
    assert!(f.module.is_active(&f.token));
    assert!(!f.can_transfer(&us));
    assert!(!f.module.can_create(&us, &1, &f.token));
}

#[test]
fn non_us_recipient_is_allowed_during_period() {
    let f = Fixture::new();
    f.at(10);
    let co = f.residing_in(CO);
    assert!(f.can_transfer(&co));
    assert!(f.module.can_create(&co, &1, &f.token));
}

#[test]
fn us_recipient_is_allowed_after_period() {
    let f = Fixture::new();
    f.at(END + 500);
    let us = f.residing_in(US);
    assert!(!f.module.is_active(&f.token));
    assert!(f.can_transfer(&us));
    assert!(f.module.can_create(&us, &1, &f.token));
}

#[test]
fn boundary_ledger_is_exact() {
    let f = Fixture::new();
    let us = f.residing_in(US);
    f.at(END - 1);
    assert!(!f.can_transfer(&us), "last blocked ledger");
    f.at(END);
    assert!(f.can_transfer(&us), "period_end_ledger itself is free");
}

#[test]
fn nothing_is_blocked_until_a_period_is_set() {
    let f = Fixture::new();
    let other = Address::generate(&f.e);
    let module = RegSDistributionPeriodClient::new(
        &f.e,
        &f.e.register(RegSDistributionPeriod, (&f.owner, &other)),
    );
    module.set_identity_registry_storage(&other, &f.irs);
    let us = f.residing_in(US);
    assert_eq!(module.period_end(&other), 0);
    assert!(!module.is_active(&other));
    assert!(module.can_transfer(&us, &us, &1, &other));
}

#[test]
fn recipient_without_country_data_fails_closed() {
    let f = Fixture::new();
    f.at(10);
    let unknown = f.account(Vec::new(&f.e));
    assert!(!f.can_transfer(&unknown));
}

#[test]
fn only_configured_relations_count() {
    let f = Fixture::new();
    f.at(10);
    // Lives in Colombia, US citizen: allowed by default (residence only)...
    let dual = f.account(vec![
        &f.e,
        individual(&f.e, IndividualCountryRelation::Residence(CO)),
        individual(&f.e, IndividualCountryRelation::Citizenship(US)),
    ]);
    assert!(f.can_transfer(&dual));
    // ...blocked once counsel adds citizenship.
    f.module.set_restricted_relations(
        &f.token,
        &vec![&f.e, RelationKind::Residence, RelationKind::Citizenship],
    );
    assert!(!f.can_transfer(&dual));
}

#[test]
fn us_organization_is_blocked_by_incorporation() {
    let f = Fixture::new();
    f.at(10);
    let org = f.account(vec![
        &f.e,
        CountryData {
            country: CountryRelation::Organization(OrganizationCountryRelation::Incorporation(US)),
            metadata: None,
        },
    ]);
    assert!(!f.can_transfer(&org));
}

#[test]
fn empty_relation_list_is_rejected() {
    let f = Fixture::new();
    assert!(f
        .module
        .try_set_restricted_relations(&f.token, &Vec::new(&f.e))
        .is_err());
}

#[test]
fn config_requires_the_owner() {
    let f = Fixture::new();
    let stranger = Address::generate(&f.e);
    f.e.set_auths(&[]);
    // No auth at all: rejected.
    assert!(f.module.try_set_period_end(&f.token, &1).is_err());
    // Auth from someone else: rejected.
    f.e.mock_auths(&[MockAuth {
        address: &stranger,
        invoke: &MockAuthInvoke {
            contract: &f.module.address,
            fn_name: "set_period_end",
            args: (&f.token, 1u32).into_val(&f.e),
            sub_invokes: &[],
        },
    }]);
    assert!(f.module.try_set_period_end(&f.token, &1).is_err());
    assert_eq!(f.module.period_end(&f.token), END);
}

#[test]
fn compliance_address_is_one_time() {
    let f = Fixture::new();
    let other = Address::generate(&f.e);
    assert!(f.module.try_set_compliance_address(&other).is_err());
}
