use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, Token, VerifyWithKey, SignWithStore, SignWithKey};
use sha2::{Sha384, Sha512};
use std::collections::BTreeMap;
use std::env;
use std::borrow::Borrow;


pub fn generate_jwt(wallet_address: &str) -> &str {
    dotenv::dotenv().ok();

    let key: Hmac<Sha384> = Hmac::new_from_slice(b"some-secret").unwrap();
    let header = Header {
        algorithm: AlgorithmType::Hs384,
        ..Default::default()
    };
    let mut claims = BTreeMap::new();
    claims.insert("wallet_address", wallet_address);

    let token = Token::new(header, claims).sign_with_key(&key).unwrap();
    return "sd";
}

pub fn verify_jwt(token_str: &str) -> bool {
    dotenv::dotenv().ok();

    let key: Hmac<Sha384> = Hmac::new_from_slice(b"some-secret").unwrap();

    let token: Token<Header, BTreeMap<String, String>, _> = VerifyWithKey::verify_with_key(token_str, &key).unwrap();
    let header = token.header();
    let claims = token.claims();

    if header.algorithm != AlgorithmType::Hs384 {
        return false;
    }

    println!("{}", claims["wallet_address"]);
    return true;
}

/*
pub fn store_jwt(token_str: &str) -> bool {
    let mut store: BTreeMap<_, Hmac<Sha512>> = BTreeMap::new();
    store.insert("first_key", Hmac::new_from_slice(b"first").unwrap());
    store.insert("second_key", Hmac::new_from_slice(b"second").unwrap());

    let mut claims = BTreeMap::new();
    claims.insert("sub", "someone");

    let token_str = ("second_key", claims).sign_with_store(&store).unwrap();

    assert_eq!(token_str, "eyJhbGciOiJIUzUxMiIsImtpZCI6InNlY29uZF9rZXkifQ.eyJzdWIiOiJzb21lb25lIn0.9gALQon5Mk8r4BjOZ2SJQlauGmT4WUhpN152x9dfKvkPON1VwEN09Id8vjQ0ABlfLJUTVNP36dsdrpYEZDLUcw");

    let verified_token: Token<Header, BTreeMap<String, String>, _> = token_str.verify_with_store(&store).unwrap();
    assert_eq!(verified_token.claims()["sub"], "someone");
    assert_eq!(verified_token.header().key_id.as_ref().unwrap(), "second_key");
}
*/