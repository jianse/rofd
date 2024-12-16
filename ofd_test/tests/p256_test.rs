use der::EncodePem;
use ecdsa::elliptic_curve::rand_core::OsRng;
use p256::ecdsa::DerSignature;
use p256::NistP256;
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;
use x509_cert::builder::{Builder, RequestBuilder};
use x509_cert::ext::pkix::name::GeneralName;
use x509_cert::ext::pkix::SubjectAltName;
use x509_cert::name::Name;

fn ecdsa_signer() -> ecdsa::SigningKey<NistP256> {
    let mut rng = OsRng;

    let p256 = p256::SecretKey::random(&mut rng);
    ecdsa::SigningKey::from(p256)
}
#[test]
fn test_gen_p256() -> eyre::Result<()> {
    // let mut rng = OsRng;
    // let p256 = p256::SecretKey::random(&mut rng);
    // let pub_key = SubjectPublicKeyInfoOwned::from_key(p256.public_key())?;

    let subject = Name::from_str("CN=service.domination.world")?;

    let signer = ecdsa_signer();

    let mut builder = RequestBuilder::new(subject, &signer)?;
    builder.add_extension(&SubjectAltName(vec![GeneralName::from(IpAddr::V4(
        Ipv4Addr::new(192, 0, 2, 0),
    ))]))?;
    let csr = builder.build::<DerSignature>()?;
    println!("{}", csr.to_pem(pkcs8::LineEnding::LF)?);
    Ok(())
}
