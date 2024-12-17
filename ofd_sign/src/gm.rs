use der::pem::LineEnding;
use sm2::elliptic_curve::rand_core::OsRng;
use sm2::pkcs8::{EncodePrivateKey, EncodePublicKey};
use sm2::SecretKey;

pub fn gen_keypair_pem() {
    let mut rng = OsRng;
    let key = SecretKey::random(&mut rng);
    key.write_pkcs8_pem_file("", LineEnding::LF)
        .expect("TODO: panic message");
    let pub_key = key.public_key();
    pub_key
        .write_public_key_pem_file("", LineEnding::LF)
        .expect("");
}
