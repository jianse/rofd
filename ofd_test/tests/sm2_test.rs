#![cfg(feature = "sm2")]

// use sm2::dsa::SigningKey;
// use sm2::dsa::SigningKey;
// use std::time::Duration;
use chrono::Duration;
use p256::NistP256;
use pkcs8::DecodePublicKey;
use sm2::elliptic_curve::rand_core::OsRng;
use sm2::pkcs8::{DecodePrivateKey, EncodePrivateKey, EncodePublicKey, LineEnding};
use sm2::Sm2;
use std::str::FromStr;
use x509_cert::builder::{Profile, RequestBuilder};
use x509_cert::name::Name;
use x509_cert::serial_number::SerialNumber;
use x509_cert::spki::SubjectPublicKeyInfoOwned;
use x509_cert::time::Validity;

#[test]
fn test_gen_sm2() -> eyre::Result<()> {
    let mut rng = OsRng;
    let sm2 = sm2::SecretKey::random(&mut rng);
    sm2.write_pkcs8_pem_file("../output/test.sm2_p8.key", LineEnding::LF)?;
    sm2.public_key()
        .write_public_key_pem_file("../output/test.sm2_p8.pub", LineEnding::LF)?;
    Ok(())
}

// fn sm2_signer() -> ecdsa::SigningKey<Sm2> {
//     let secret_key = sm2::SecretKey::read_pkcs8_pem_file("../output/test.sm2_p8.key").unwrap();
//     // SubjectPublicKeyInfo::from_key(secret_key.public_key())
//     // ecdsa::SigningKey::from(&secret_key.public_key())
//     // SigningKey::
//     // // SigningKey::new("",&secret_key)
//     // SubjectPublicKeyInfo::read_pkcs8_pem_file("../output/test.sm2_p8.key")
//     todo!()
// }

// impl

#[test]
fn test_gen_cert() -> eyre::Result<()> {
    // sm2::
    Ok(())
}

// use p521::{pkcs8::DecodePrivateKey, NistP521, ecdsa::DerSignature};

mod p521 {}
