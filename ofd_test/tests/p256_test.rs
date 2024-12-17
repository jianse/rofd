use der::pem::LineEnding;
use der::EncodePem;
use ecdsa::elliptic_curve::rand_core::OsRng;
use eyre::Result;
use p256::ecdsa::DerSignature;
use p256::{NistP256, PublicKey};
use pkcs8::{EncodePrivateKey, EncodePublicKey};
use std::fs::File;
use std::io::Write;
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;
use x509_cert::builder::{Builder, CertificateBuilder, Profile, RequestBuilder};
use x509_cert::ext::pkix::name::GeneralName;
use x509_cert::ext::pkix::SubjectAltName;
use x509_cert::name::Name;
use x509_cert::serial_number::SerialNumber;
use x509_cert::spki::{DecodePublicKey, SubjectPublicKeyInfoOwned};
use x509_cert::time::Validity;

fn ecdsa_signer() -> ecdsa::SigningKey<NistP256> {
    let mut rng = OsRng;

    let p256 = p256::SecretKey::random(&mut rng);
    ecdsa::SigningKey::from(p256)
}

#[test]
#[ignore = "should do this manually."]
fn gen_p256() -> Result<()> {
    let mut rng = OsRng;

    let p256 = p256::SecretKey::random(&mut rng);
    p256.write_pkcs8_pem_file("resources/test.p256.key", LineEnding::LF)?;
    p256.public_key()
        .write_public_key_pem_file("resources/test.p256.pub", LineEnding::LF)?;
    Ok(())
}
#[test]
fn test_gen_p256() -> eyre::Result<()> {
    // let mut rng = OsRng;
    // let p256 = p256::SecretKey::random(&mut rng);
    let p = PublicKey::read_public_key_pem_file("resources/test.p256.pub")?;
    let pub_key = SubjectPublicKeyInfoOwned::from_key(p)?;

    let subject = Name::from_str("CN=service.domination.world")?;

    let signer = ecdsa_signer();

    let mut builder = RequestBuilder::new(subject, &signer)?;
    builder.add_extension(&SubjectAltName(vec![GeneralName::from(IpAddr::V4(
        Ipv4Addr::new(192, 0, 2, 0),
    ))]))?;
    let csr = builder.build::<DerSignature>()?;
    // println!("{}", csr.to_pem(pkcs8::LineEnding::LF)?);
    File::create("resources/test.p256.csr")?.write_all(csr.to_pem(LineEnding::LF)?.as_bytes())?;

    let cert = CertificateBuilder::new(
        Profile::Root,
        SerialNumber::from(1_u32),
        Validity::from_now(chrono::Duration::days(3650).to_std()?)?,
        Name::from_str("CN=test")?,
        pub_key,
        &signer,
    )?;
    let cert = cert.build::<DerSignature>()?;
    let s = cert.to_pem(LineEnding::LF)?;
    File::create("resources/test.p256.crt")?.write_all(s.as_bytes())?;
    Ok(())
}
