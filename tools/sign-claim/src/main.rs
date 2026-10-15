//! Usage:
//!   sign-claim pubkey <secret-key-file>
//!   sign-claim sign <secret-key-file> <claim-issuer C...> <identity C...> <topic> <valid-days> <now-unix> [testnet|mainnet]
//!
//! `sign` prints `data=<hex>` and `signature=<hex>` (public key || signature, 96 bytes),
//! the two values `add_claim` takes on the investor identity contract.

use ed25519_dalek::{Signer, SigningKey};
use soroban_sdk::{contract, contractimpl, testutils::Ledger as _, Address, Bytes, Env};
use stellar_tokens::rwa::claim_issuer::{
    encode_claim_data_expiration, Ed25519Verifier, SignatureVerifier,
};

#[contract]
struct Stub;

#[contractimpl]
impl Stub {}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn load_key(path: &str) -> SigningKey {
    let text = std::fs::read_to_string(path).expect("cannot read the secret key file");
    let text = text.trim();
    assert_eq!(text.len(), 64, "secret key must be 64 hex characters");
    let mut sk = [0u8; 32];
    for (i, b) in sk.iter_mut().enumerate() {
        *b = u8::from_str_radix(&text[2 * i..2 * i + 2], 16).expect("invalid hex");
    }
    SigningKey::from_bytes(&sk)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("pubkey") if args.len() == 2 => {
            println!("{}", hex(&load_key(&args[1]).verifying_key().to_bytes()));
        }
        Some("sign") if args.len() == 7 || args.len() == 8 => {
            let key = load_key(&args[1]);
            let topic: u32 = args[4].parse().expect("topic");
            let days: u64 = args[5].parse().expect("valid-days");
            let now: u64 = args[6].parse().expect("now-unix");

            // The signed message includes the network id, so it must match the target network.
            let passphrase = match args.get(7).map(String::as_str) {
                None | Some("testnet") => "Test SDF Network ; September 2015",
                Some("mainnet") => "Public Global Stellar Network ; September 2015",
                Some(other) => panic!("unknown network {other}"),
            };
            let e = Env::default();
            let network_id = e
                .crypto()
                .sha256(&Bytes::from_slice(&e, passphrase.as_bytes()))
                .to_array();
            e.ledger().with_mut(|li| li.network_id = network_id);
            let issuer = Address::from_str(&e, &args[2]);
            let identity = Address::from_str(&e, &args[3]);
            e.register_at(&issuer, Stub, ());

            let raw = Bytes::from_slice(&e, b"rwaxi-spike-kyc");
            let data = encode_claim_data_expiration(&e, now, now + days * 86_400, &raw);
            let message = e.as_contract(&issuer, || {
                Ed25519Verifier::build_message(&e, &identity, topic, &data)
            });
            let buf = message.to_buffer::<256>();
            let sig = key
                .sign(&buf.as_slice()[..message.len() as usize])
                .to_bytes();

            let mut data_bytes = std::vec![0u8; data.len() as usize];
            data.copy_into_slice(&mut data_bytes);
            println!("data={}", hex(&data_bytes));
            println!(
                "signature={}{}",
                hex(&key.verifying_key().to_bytes()),
                hex(&sig)
            );
        }
        _ => {
            eprintln!("usage: sign-claim pubkey <key-file> | sign <key-file> <issuer> <identity> <topic> <days> <now> [network]");
            std::process::exit(2);
        }
    }
}
