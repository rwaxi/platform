//! The spike scenario from ADR-007, on the real contracts.
//!
//! Three verified investors hold wallets: Alice and Bob live in Colombia, Carol in
//! the United States. While the Reg S period runs, Alice -> Bob works and
//! Alice -> Carol is rejected by the compliance module. After the period, Carol can
//! receive.

extern crate std;

use claim_issuer::{ClaimIssuerContract, ClaimIssuerContractClient};
use claim_topics_and_issuers::ClaimTopicsAndIssuersContract;
use compliance::{ComplianceContract, ComplianceContractClient};
use ed25519_dalek::Signer;
use identity_registry::{IdentityRegistryContract, IdentityRegistryContractClient};
use identity_verifier::IdentityVerifierContract;
use investor_identity::{IdentityContract, IdentityContractClient};
use reg_s_distribution_period::{RegSDistributionPeriod, RegSDistributionPeriodClient};
use rwa_token::{RWATokenContract, RWATokenContractClient};
use soroban_sdk::{
    testutils::{Address as _, Ledger as _},
    vec, Address, Bytes, BytesN, Env, Error, IntoVal, String, Val, Vec,
};
use stellar_tokens::rwa::{
    claim_issuer::{encode_claim_data_expiration, Ed25519Verifier, SignatureVerifier},
    claim_topics_and_issuers::ClaimTopicsAndIssuersClient,
    compliance::ComplianceHook,
    identity_registry_storage::{CountryData, CountryRelation, IndividualCountryRelation},
};

const KYC: u32 = 1;
const ED25519: u32 = 101;
const COLOMBIA: u32 = 170;
const USA: u32 = 840;
const PERIOD_END: u32 = 5_000;
const UNIT: i128 = 10_000_000;
/// `RWAError::TransferNotCompliant` / `MintNotCompliant` in OZ stellar-tokens 0.7.2.
const TRANSFER_NOT_COMPLIANT: u32 = 305;
const MINT_NOT_COMPLIANT: u32 = 306;

struct World<'a> {
    e: Env,
    admin: Address,
    key: ed25519_dalek::SigningKey,
    issuer: Address,
    registry: IdentityRegistryContractClient<'a>,
    token: RWATokenContractClient<'a>,
    module: RegSDistributionPeriodClient<'a>,
}

impl<'a> World<'a> {
    fn new() -> Self {
        let e = Env::default();
        e.mock_all_auths();
        e.ledger().set_sequence_number(100);
        e.ledger().set_timestamp(1_800_000_000);
        let admin = Address::generate(&e);

        // Claim topics, issuer and its signing key.
        let cti_id = e.register(ClaimTopicsAndIssuersContract, (&admin, &admin));
        let cti = ClaimTopicsAndIssuersClient::new(&e, &cti_id);
        cti.add_claim_topic(&KYC, &admin);
        let issuer = e.register(ClaimIssuerContract, (&admin,));
        cti.add_trusted_issuer(&issuer, &vec![&e, KYC], &admin);
        let key = ed25519_dalek::SigningKey::from_bytes(&[7u8; 32]);
        let pk = Bytes::from_array(&e, &key.verifying_key().to_bytes());
        ClaimIssuerContractClient::new(&e, &issuer).allow_key(&pk, &cti_id, &KYC);

        // Identity registry and verifier.
        let registry_id = e.register(IdentityRegistryContract, (&admin, &admin));
        let registry = IdentityRegistryContractClient::new(&e, &registry_id);
        let verifier = e.register(
            IdentityVerifierContract,
            (&admin, &admin, &registry_id, &cti_id),
        );

        // Compliance dispatcher and the Reg S module.
        let compliance_id = e.register(ComplianceContract, (&admin, &admin));
        let compliance = ComplianceContractClient::new(&e, &compliance_id);
        let module_id = e.register(RegSDistributionPeriod, (&admin, &compliance_id));
        let module = RegSDistributionPeriodClient::new(&e, &module_id);
        compliance.add_module_to(&ComplianceHook::CanTransfer, &module_id, &admin);
        compliance.add_module_to(&ComplianceHook::CanCreate, &module_id, &admin);

        // Token, bound to the registry and compliance.
        let token_id = e.register(
            RWATokenContract,
            (
                String::from_str(&e, "Avista Beach Fund I"),
                String::from_str(&e, "ABF1"),
                &admin,
                &admin,
                &compliance_id,
                &verifier,
            ),
        );
        let token = RWATokenContractClient::new(&e, &token_id);
        registry.bind_token(&token_id, &admin);
        compliance.bind_token(&token_id, &admin);

        module.set_identity_registry_storage(&token_id, &registry_id);
        module.set_period_end(&token_id, &PERIOD_END);

        World {
            e,
            admin,
            key,
            issuer,
            registry,
            token,
            module,
        }
    }

    /// A verified investor: identity contract with a signed KYC claim, plus a registry
    /// entry with the country of residence.
    fn investor(&self, country: u32) -> Address {
        let e = &self.e;
        let wallet = Address::generate(e);
        let identity_id = e.register(IdentityContract, (&wallet,));
        let identity = IdentityContractClient::new(e, &identity_id);

        let raw = Bytes::from_array(e, &[1u8, 2, 3]);
        let now = e.ledger().timestamp();
        let data = encode_claim_data_expiration(e, now, now + 365 * 86_400, &raw);
        let message = e.as_contract(&self.issuer, || {
            Ed25519Verifier::build_message(e, &identity_id, KYC, &data)
        });
        let buf = message.to_buffer::<256>();
        let sig = self
            .key
            .sign(&buf.as_slice()[..message.len() as usize])
            .to_bytes();
        let mut sig_data = Bytes::from_array(e, &self.key.verifying_key().to_bytes());
        sig_data.append(&Bytes::from_array(e, &sig));
        let _id: BytesN<32> = identity.add_claim(
            &KYC,
            &ED25519,
            &self.issuer,
            &sig_data,
            &data,
            &String::from_str(e, "https://rwaxi.com/kyc"),
        );

        let profile = CountryData {
            country: CountryRelation::Individual(IndividualCountryRelation::Residence(country)),
            metadata: None,
        };
        let profiles: Vec<Val> = vec![e, profile.into_val(e)];
        self.registry
            .add_identity(&wallet, &identity_id, &profiles, &self.admin);
        wallet
    }
}

#[test]
fn alice_to_bob_allowed_and_alice_to_carol_blocked_during_period() {
    let w = World::new();
    let alice = w.investor(COLOMBIA);
    let bob = w.investor(COLOMBIA);
    let carol = w.investor(USA);

    w.token.mint(&alice, &(100 * UNIT), &w.admin);
    w.token.transfer(&alice, &bob, &(10 * UNIT));
    assert_eq!(w.token.balance(&bob), 10 * UNIT);

    assert_eq!(
        w.token
            .try_transfer(&alice, &carol, &(10 * UNIT))
            .unwrap_err(),
        Ok(Error::from_contract_error(TRANSFER_NOT_COMPLIANT))
    );
    assert_eq!(w.token.balance(&carol), 0);
    assert_eq!(w.token.balance(&alice), 90 * UNIT);
}

#[test]
fn minting_to_a_us_investor_is_blocked_during_period() {
    let w = World::new();
    let carol = w.investor(USA);
    assert_eq!(
        w.token.try_mint(&carol, &UNIT, &w.admin).unwrap_err(),
        Ok(Error::from_contract_error(MINT_NOT_COMPLIANT))
    );
    assert_eq!(w.token.balance(&carol), 0);
}

#[test]
fn us_investor_can_receive_after_the_period_ends() {
    let w = World::new();
    let alice = w.investor(COLOMBIA);
    let carol = w.investor(USA);
    w.token.mint(&alice, &(100 * UNIT), &w.admin);

    w.e.ledger().set_sequence_number(PERIOD_END - 1);
    assert_eq!(
        w.token.try_transfer(&alice, &carol, &UNIT).unwrap_err(),
        Ok(Error::from_contract_error(TRANSFER_NOT_COMPLIANT))
    );

    w.e.ledger().set_sequence_number(PERIOD_END);
    assert!(!w.module.is_active(&w.token.address));
    w.token.transfer(&alice, &carol, &UNIT);
    assert_eq!(w.token.balance(&carol), UNIT);
}

#[test]
fn forced_transfer_is_exempt_from_the_period() {
    let w = World::new();
    let alice = w.investor(COLOMBIA);
    let carol = w.investor(USA);
    w.token.mint(&alice, &(100 * UNIT), &w.admin);
    w.token.forced_transfer(&alice, &carol, &UNIT, &w.admin);
    assert_eq!(w.token.balance(&carol), UNIT);
}

#[test]
fn a_wallet_without_identity_cannot_receive() {
    let w = World::new();
    let alice = w.investor(COLOMBIA);
    let stranger = Address::generate(&w.e);
    w.token.mint(&alice, &(100 * UNIT), &w.admin);
    // Rejected by the identity stack, not by the Reg S module.
    assert!(w.token.try_transfer(&alice, &stranger, &UNIT).is_err());
}
