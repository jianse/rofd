// #![cfg(feature = "sm2")]

use chrono::Duration;
use der::EncodePem;
use hex::ToHex;
use pkcs8::{DecodePublicKey, SubjectPublicKeyInfo};
use sm2::dsa::SigningKey;
use sm2::elliptic_curve::rand_core::OsRng;
use sm2::elliptic_curve::sec1::ToEncodedPoint;
use sm2::pkcs8::{DecodePrivateKey, EncodePrivateKey, EncodePublicKey, LineEnding};
use sm2::{PublicKey, SecretKey};
use std::str::FromStr;
use x509_cert::builder::{Builder, CertificateBuilder, Profile};
use x509_cert::name::Name;
use x509_cert::serial_number::SerialNumber;
use x509_cert::time::Validity;

#[test]
#[ignore = "should run manually."]
fn test_gen_sm2() -> eyre::Result<()> {
    let mut rng = OsRng;
    let sm2 = SecretKey::random(&mut rng);
    sm2.write_pkcs8_pem_file("resources/test.sm2_p8.key", LineEnding::LF)?;
    sm2.public_key()
        .write_public_key_pem_file("resources/test.sm2_p8.pub", LineEnding::LF)?;
    Ok(())
}

pub mod mysm2 {
    use der::asn1::{BitString, ObjectIdentifier};
    use der::Document;
    use ecdsa::Error;
    use pkcs8::spki::AlgorithmIdentifier;
    use sm2::dsa::signature::Keypair;
    use x509_cert::spki::{
        EncodePublicKey, SignatureAlgorithmIdentifier, SignatureBitStringEncoding,
    };

    /// SM2DSA
    pub struct SigningKey(sm2::dsa::SigningKey);

    #[derive(Clone)]
    pub struct VerifyingKey(sm2::dsa::VerifyingKey);
    impl From<sm2::dsa::SigningKey> for SigningKey {
        fn from(value: sm2::dsa::SigningKey) -> Self {
            Self(value)
        }
    }
    impl EncodePublicKey for VerifyingKey {
        fn to_public_key_der(&self) -> pkcs8::spki::Result<Document> {
            sm2::PublicKey::from(&self.0).to_public_key_der()
        }
    }

    impl Keypair for SigningKey {
        type VerifyingKey = VerifyingKey;

        fn verifying_key(&self) -> Self::VerifyingKey {
            VerifyingKey(self.0.verifying_key().clone())
        }
    }

    impl SignatureAlgorithmIdentifier for SigningKey {
        type Params = ();
        const SIGNATURE_ALGORITHM_IDENTIFIER: AlgorithmIdentifier<Self::Params> =
            AlgorithmIdentifier {
                oid: ObjectIdentifier::new_unwrap("1.2.156.10197.1.501"),
                parameters: None,
            };
    }

    pub struct Signature(sm2::dsa::Signature);

    impl From<sm2::dsa::Signature> for Signature {
        fn from(value: sm2::dsa::Signature) -> Self {
            Self(value)
        }
    }

    impl ecdsa::signature::Signer<Signature> for SigningKey {
        fn try_sign(&self, msg: &[u8]) -> Result<Signature, Error> {
            self.0.try_sign(msg).map(Into::into)
        }
    }

    impl SignatureBitStringEncoding for Signature {
        fn to_bitstring(&self) -> der::Result<BitString> {
            BitString::new(0, self.0.to_bytes())
        }
    }
}

fn mysm2_signer() -> eyre::Result<mysm2::SigningKey> {
    let key = SecretKey::read_pkcs8_pem_file("resources/test.sm2_p8.key")?;
    let signing_key = SigningKey::new("1234", &key)?;
    Ok(mysm2::SigningKey::from(signing_key))
}

#[test]
fn test_mysm2() -> eyre::Result<()> {
    let pub_key = PublicKey::read_public_key_pem_file("resources/test.sm2_p8.pub")?;
    let signer = mysm2_signer()?;
    let cb = CertificateBuilder::new(
        Profile::Root,
        SerialNumber::from(1_u32),
        Validity::from_now(Duration::days(3650).to_std()?)?,
        Name::from_str("CN=test")?,
        SubjectPublicKeyInfo::from_key(pub_key)?,
        &signer,
    )?;
    let cert = cb.build()?;

    println!("{}", cert.to_pem(LineEnding::LF)?);
    Ok(())
}

#[test]
fn test_compressed_key() -> eyre::Result<()> {
    let hex = hex::decode("024B0FA601977C659C9DF6E1DD4BD55243BF42B7FC0AB92F2984539D4824FCB9C2")?;
    let pk = PublicKey::from_sec1_bytes(&hex)?;
    // let pk
    let enc_point = pk.to_encoded_point(true);
    let h = enc_point.as_bytes().encode_hex::<String>();
    println!("{}", &h);
    let s = pk.to_public_key_pem(LineEnding::LF)?;
    println!("{}", s);
    Ok(())
}
