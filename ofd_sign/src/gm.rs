use crate::v4::{
    SesCertList, SesEsPictureInfo, SesEsPropertyInfo, SesHeader, SesSeal, SesSealInfo,
};
use crate::{Error, SignClass};
use der::asn1::{GeneralizedTime, Ia5String, ObjectIdentifier, OctetString};
use der::pem::LineEnding;
use der::zeroize::Zeroizing;
use der::{DecodePem, Encode, EncodePem};
use pkcs8::spki::SubjectPublicKeyInfoOwned;
use pkcs8::DecodePrivateKey;
use signature::Signer;
use sm2::dsa::SigningKey;
use sm2::elliptic_curve::rand_core::OsRng;
use sm2::pkcs8::{EncodePrivateKey, EncodePublicKey};
use sm2::SecretKey;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use x509_cert::builder::{Builder, CertificateBuilder, Profile};
use x509_cert::name::Name;
use x509_cert::serial_number::SerialNumber;
use x509_cert::spki::SignatureBitStringEncoding;
use x509_cert::time::Validity;
use x509_cert::Certificate;

fn sm2_signer(path: impl AsRef<Path>) -> eyre::Result<crate::ext::SigningKey> {
    let key = SecretKey::read_pkcs8_pem_file(path)?;
    let signing_key = SigningKey::new("1234", &key)?;
    Ok(crate::ext::SigningKey::from(signing_key))
}

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
    mkdir: bool,
    signing_key_path: PathBuf,
    subject: String,
    duration: std::time::Duration,
    tbs_key_path: PathBuf,
    output: PathBuf,
}

impl GenCertReq {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mkdir(&mut self, mkdir: bool) -> &mut Self {
        self.mkdir = mkdir;
        self
    }
    pub fn signing_key_path(&mut self, signing_key_path: impl AsRef<Path>) -> &mut Self {
        self.signing_key_path = signing_key_path.as_ref().to_path_buf();
        self
    }
    pub fn subject(&mut self, subject: impl AsRef<str>) -> &mut Self {
        self.subject = subject.as_ref().to_owned();
        self
    }
    pub fn duration(&mut self, duration: std::time::Duration) -> &mut Self {
        self.duration = duration;
        self
    }
    pub fn tbs_key_path(&mut self, tbs_key_path: impl AsRef<Path>) -> &mut Self {
        self.tbs_key_path = tbs_key_path.as_ref().to_path_buf();
        self
    }

    pub fn output(&mut self, output: impl AsRef<Path>) -> &mut Self {
        self.output = output.as_ref().to_path_buf();
        self
    }

    fn read_pem_from_path(path: impl AsRef<Path>) -> eyre::Result<Zeroizing<Vec<u8>>> {
        let buf = std::fs::read(path)?;
        Ok(Zeroizing::new(buf))
    }

    fn generate_sm2(&self) -> eyre::Result<()> {
        let bytes = GenCertReq::read_pem_from_path(&self.tbs_key_path)?;
        let pub_key = SubjectPublicKeyInfoOwned::from_pem(bytes)?;
        let signer = sm2_signer(&self.signing_key_path)?;
        // signer.verifying_key();
        let cert_builder = CertificateBuilder::new(
            Profile::Root,
            SerialNumber::from(1_u32),
            Validity::from_now(self.duration)?,
            Name::from_str(&self.subject)?,
            pub_key,
            &signer,
        )?;

        // cert_builder.add_extension();

        let cert = cert_builder.build()?;
        let cert_pem = cert.to_pem(LineEnding::LF)?;
        if self.mkdir {
            if let Some(dirs) = self.signing_key_path.parent() {
                create_dir_all(dirs)?;
            }
            // fail if file already exist
            let mut file = File::create_new(&self.output)?;
            file.write_all(cert_pem.as_bytes())?;
        }
        Ok(())
    }

    pub fn generate(&self) -> eyre::Result<()> {
        // TODO: SUPPORT GENERATE OTHER CERT
        self.generate_sm2()
    }
}

pub struct ESealReq {
    /// 当输出路径不存在时，是否新建路径
    mkdir: bool,
    /// 签名用的私钥
    signing_key_path: PathBuf,
    /// 签名用的证书
    signing_cert_path: PathBuf,
    /// 输出路径
    output: PathBuf,

    /// 签章外观图片
    picture_path: PathBuf,
    /// 图片类型
    picture_type: String,
    /// 图片尺寸
    picture_size: (u64, u64),

    /// 内嵌的证书
    embed_cert_path: PathBuf,
    /// 签章版本
    version: SignClass,
    /// 有效时长
    duration: std::time::Duration,
    /// 章名称
    stamp_name: String,
}
impl Default for ESealReq {
    fn default() -> Self {
        Self {
            mkdir: Default::default(),
            signing_key_path: Default::default(),
            signing_cert_path: Default::default(),
            output: Default::default(),
            picture_path: Default::default(),
            picture_type: Default::default(),
            picture_size: Default::default(),
            embed_cert_path: Default::default(),
            version: SignClass::SesV4,
            duration: Default::default(),
            stamp_name: Default::default(),
        }
    }
}

impl ESealReq {
    pub fn new() -> Self {
        Self::default()
    }

    fn read_picture_v4(&self) -> eyre::Result<SesEsPictureInfo> {
        let img = std::fs::read(&self.picture_path)?;

        Ok(SesEsPictureInfo {
            r#type: Ia5String::new(&self.picture_type)?,
            data: OctetString::new(img)?,
            width: self.picture_size.0,
            height: self.picture_size.1,
        })
    }

    fn es_info_header(&self) -> SesHeader {
        SesHeader {
            id: Ia5String::new("ES").unwrap(),
            version: 4,
            vid: Ia5String::new("rofd").unwrap(),
        }
    }

    fn es_info_property(&self) -> eyre::Result<SesEsPropertyInfo> {
        let validity = Validity::from_now(self.duration)?;

        let valid_start = GeneralizedTime::from_date_time(validity.not_before.to_date_time());
        let valid_end = GeneralizedTime::from_date_time(validity.not_after.to_date_time());

        let c = std::fs::read(&self.embed_cert_path)?;
        let cert = Certificate::from_pem(c.as_slice())?;

        // let cert =
        let property = SesEsPropertyInfo {
            r#type: 0,
            name: self.stamp_name.clone(),
            cert_list_type: 1,
            cert_list: SesCertList::Certs(vec![OctetString::new(cert.to_der()?)?]),
            create_date: valid_start,
            valid_start,
            valid_end,
        };
        Ok(property)
    }

    pub fn assemble_e_seal_info_v4(&self) -> eyre::Result<SesSealInfo> {
        let picture = self.read_picture_v4()?;

        let seal_info = SesSealInfo {
            header: self.es_info_header(),
            es_id: Ia5String::new("1")?,
            property: self.es_info_property()?,
            picture,
            ext_data: None,
        };
        Ok(seal_info)
    }
    fn read_signing_cert(&self) -> eyre::Result<Certificate> {
        let cert = std::fs::read(self.signing_cert_path.as_path())?;
        let c = Certificate::from_pem(cert.as_slice())?;
        Ok(c)
    }

    pub fn assemble_e_seal_v4(&self) -> eyre::Result<SesSeal> {
        let signer: crate::ext::SigningKey = sm2_signer(&self.signing_key_path)?;

        let esi = self.assemble_e_seal_info_v4()?;
        let signed_value = signer.try_sign(&esi.to_der()?)?;
        let cert = self.read_signing_cert()?;

        let es = SesSeal {
            e_seal_info: esi,
            cert: OctetString::new(cert.to_der()?)?,
            signature_alg_id: ObjectIdentifier::new_unwrap("1.2.156.10197.1.501"),
            signed_value: signed_value.to_bitstring()?,
        };
        Ok(es)
    }

    pub fn generate(&self) -> eyre::Result<()> {
        let der = match self.version {
            SignClass::Unknown | SignClass::SesV4 => {
                let es = self.assemble_e_seal_v4()?;
                es.to_der()?
            }
            SignClass::SesV1 => todo!(),
        };
        if self.mkdir {
            if let Some(dirs) = self.picture_path.parent() {
                create_dir_all(dirs)?;
            }
        }
        File::create_new(&self.output)?.write_all(&der)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eyre::Result;

    #[test]
    #[ignore = "should run manually"]
    fn test_gen_e_seal() -> Result<()> {
        let mut esr = ESealReq::new();
        esr.signing_key_path = "../ofd_test/resources/test.sm2_p8.key".into();
        esr.signing_cert_path = "../ofd_test/resources/test.sm2_p8.crt".into();
        esr.mkdir = true;

        // a year
        esr.duration = std::time::Duration::from_secs(60 * 60 * 24 * 366);
        esr.embed_cert_path = "../ofd_test/resources/test.sm2_p8.crt".into();
        esr.stamp_name = "测试用章".into();
        esr.picture_size = (80, 50);
        esr.picture_type = "PNG".into();
        esr.picture_path = "../ofd_test/resources/test.png".into();

        esr.output = "../output/test_stamp.es".into();
        esr.generate()?;
        Ok(())
    }
}
