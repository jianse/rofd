pub mod der;
mod error;
mod v1;
mod v4;

pub use error::Error;
use std::fmt::Debug;

use crate::der::parse_single;
use ::der::Decode;
use ofd_rw::Ofd;
use std::io::Cursor;

#[derive(Debug, PartialEq)]
pub enum SignClass {
    Unknown,
    SesV1,
    SesV4,
}

pub fn detect_sign_class(data: &[u8]) -> SignClass {
    let tlv = parse_single(&mut Cursor::new(data));
    match tlv {
        Ok(v) => {
            if v.item_count() == 2 {
                SignClass::SesV1
            } else if v.item_count() == 4 || v.item_count() == 5 {
                SignClass::SesV4
            } else {
                SignClass::Unknown
            }
        }
        Err(_) => SignClass::Unknown,
    }
}

pub trait Sign: Debug {}

pub fn decode_sign(data: &[u8]) -> Result<Box<dyn Sign>, Error> {
    let class = detect_sign_class(data);
    match class {
        SignClass::Unknown => Err(Error::UnSupportedSignClass),
        SignClass::SesV1 => {
            let sign = v1::SesSignature::from_der(data)?;
            Ok(Box::new(sign))
        }
        SignClass::SesV4 => {
            let sign = v4::SesSignature::from_der(data)?;
            Ok(Box::new(sign))
        }
    }
}

/// an extension trait for Ofd
/// make it can handle signature stuff
pub trait SignInfoExt {}

impl SignInfoExt for Ofd {}

#[cfg(test)]
mod tests {
    use crate::decode_sign;
    use eyre::Result;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn it_works() -> Result<()> {
        let f = ofd_rw::from_path("../samples/000.ofd")?;
        let sig_vec = f.signature_for_doc(0)?.unwrap();
        let sig0 = &sig_vec[0];

        let path = sig0.resolve(&sig0.signed_value);
        dbg!(&path);
        let _sign = f.bytes(path)?;
        // File::create("../samples/SignedValue.dat")?.write_all(&_sign)?;
        Ok(())
    }

    #[test]
    fn test_decode_v4() -> Result<()> {
        let mut buf = Vec::new();
        let mut file = File::open("../samples/SignedValue.dat")?;
        file.read_to_end(&mut buf)?;

        let sign = decode_sign(&buf)?;
        dbg!(&sign);

        Ok(())
    }
    #[test]
    fn test_decode_v1() -> Result<()> {
        let mut buf = Vec::new();
        let mut file = File::open("../samples/SignedValueV1.dat")?;
        file.read_to_end(&mut buf)?;

        let sign = decode_sign(&buf)?;
        dbg!(&sign);

        Ok(())
    }
}
