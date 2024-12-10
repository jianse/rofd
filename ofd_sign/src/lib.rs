mod v4;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::v4::SesSignature;
    use eyre::Result;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn it_works() -> Result<()> {
        let f = ofd_rw::from_path("../samples/test.ofd")?;
        // let sigs = f.signatures_for_doc(0)?;
        // dbg!(sigs);
        // let anno = f.annotations_for_page(0,0)?;
        // dbg!(anno);
        let sig_vec = f.signature_for_doc(0)?.unwrap();
        let sig0 = &sig_vec[0];

        let path = sig0.resolve(&sig0.signed_value);
        dbg!(&path);
        let sign = f.bytes(path)?;
        // File::create("../samples/SignedValue.dat")?.write_all(&sign)?;
        Ok(())
    }

    #[test]
    fn read_sign() -> Result<()> {
        let mut f = File::open("../samples/SignedValue.dat")?;
        let mut data = Vec::new();
        let _ = f.read_to_end(&mut data)?;
        let seq = asn1::parse_single::<SesSignature>(&data);
        dbg!(&seq);
        // seq?;
        Ok(())
    }
}
