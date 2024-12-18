use crate::Error;
use chrono::Duration;
use der::pem::LineEnding;
use der::{DecodePem, EncodePem};
use pkcs8::spki::SubjectPublicKeyInfoOwned;
use pkcs8::DecodePrivateKey;
use sm2::dsa::SigningKey;
use sm2::elliptic_curve::rand_core::OsRng;
use sm2::pkcs8::{EncodePrivateKey, EncodePublicKey};
use sm2::SecretKey;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use x509_cert::builder::{Builder, CertificateBuilder, Profile};
use x509_cert::name::Name;
use x509_cert::serial_number::SerialNumber;
use x509_cert::time::Validity;

/// Generate keypair request
/// builder for generate keypair action
#[derive(Default)]
pub struct GenKeyPairReq {
    /// create parent dirs or not
    mkdir: bool,
    /// secret key output path
    sk_path: PathBuf,
    /// extract and save public key or not
    extract_pk: bool,
    /// public key output path
    pk_path: PathBuf,
}
impl GenKeyPairReq {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn mkdir(&mut self, mkdir: bool) -> &mut Self {
        self.mkdir = mkdir;
        self
    }
    pub fn sk_path(&mut self, sk_path: impl AsRef<Path>) -> &mut Self {
        self.sk_path = sk_path.as_ref().to_path_buf();
        self
    }

    pub fn extract_pk(&mut self, extract_pk: bool) -> &mut Self {
        self.extract_pk = extract_pk;
        self
    }

    pub fn pk_path(&mut self, pk_path: impl AsRef<Path>) -> &mut Self {
        self.pk_path = pk_path.as_ref().to_path_buf();
        self
    }

    pub fn generate(&self) -> Result<(), Error> {
        let mut rng = OsRng;
        let key = SecretKey::random(&mut rng);
        if self.mkdir {
            if let Some(dirs) = self.sk_path.parent() {
                create_dir_all(dirs)?;
            }
        }
        key.write_pkcs8_pem_file(&self.sk_path, LineEnding::LF)?;
        if self.extract_pk {
            let pub_key = key.public_key();
            if self.mkdir {
                if let Some(dirs) = self.pk_path.parent() {
                    create_dir_all(dirs)?;
                }
            }
            pub_key.write_public_key_pem_file(&self.pk_path, LineEnding::LF)?;
        }
        Ok(())
    }
}

/// Generate certification request
#[derive(Default)]
pub struct GenCertReq {
    _mkdir: bool,
    signing_key_path: PathBuf,
    subject: String,
}

impl GenCertReq {
    fn sm2_signer(&self) -> eyre::Result<crate::ext::SigningKey> {
        let key = SecretKey::read_pkcs8_pem_file(&self.signing_key_path)?;
        let signing_key = SigningKey::new("1234", &key)?;
        Ok(crate::ext::SigningKey::from(signing_key))
    }

    pub fn generate(&self) -> eyre::Result<()> {
        let pub_key = SubjectPublicKeyInfoOwned::from_pem("")?;
        let signer = self.sm2_signer()?;
        // signer.verifying_key();
        let cb = CertificateBuilder::new(
            Profile::Root,
            SerialNumber::from(1_u32),
            Validity::from_now(Duration::days(3650).to_std()?)?,
            Name::from_str(&self.subject)?,
            pub_key,
            &signer,
        )?;
        let cert = cb.build()?;

        println!("{}", cert.to_pem(LineEnding::LF)?);
        // let k = pkcs8::DecodePrivateKey::read_pkcs8_pem_file(Path::new("pkcs8.pem"))?;
        Ok(())
    }
}
