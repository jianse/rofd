use der::asn1::{BitString, ObjectIdentifier};
use der::Document;
use ecdsa::Error;
use sm2::dsa::signature::Keypair;
use sm2::pkcs8;
use x509_cert::spki::{
    AlgorithmIdentifier, EncodePublicKey, SignatureAlgorithmIdentifier, SignatureBitStringEncoding,
};

/// SM2DSA secret key used for signing messages and producing signatures.
pub struct SigningKey(sm2::dsa::SigningKey);

impl From<sm2::dsa::SigningKey> for SigningKey {
    fn from(value: sm2::dsa::SigningKey) -> Self {
        Self(value)
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
    const SIGNATURE_ALGORITHM_IDENTIFIER: AlgorithmIdentifier<Self::Params> = AlgorithmIdentifier {
        oid: ObjectIdentifier::new_unwrap("1.2.156.10197.1.501"),
        parameters: None,
    };
}

impl ecdsa::signature::Signer<Signature> for SigningKey {
    fn try_sign(&self, msg: &[u8]) -> Result<Signature, Error> {
        self.0.try_sign(msg).map(Into::into)
    }
}

/// SM2DSA public key used for verifying signatures are valid for a given message.
#[derive(Clone)]
pub struct VerifyingKey(sm2::dsa::VerifyingKey);
impl EncodePublicKey for VerifyingKey {
    fn to_public_key_der(&self) -> pkcs8::spki::Result<Document> {
        sm2::PublicKey::from(&self.0).to_public_key_der()
    }
}

/// SM2DSA signature.
pub struct Signature(sm2::dsa::Signature);

impl From<sm2::dsa::Signature> for Signature {
    fn from(value: sm2::dsa::Signature) -> Self {
        Self(value)
    }
}

impl SignatureBitStringEncoding for Signature {
    fn to_bitstring(&self) -> der::Result<BitString> {
        BitString::new(0, self.0.to_bytes())
    }
}
