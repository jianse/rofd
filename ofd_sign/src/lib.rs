mod v4;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use eyre::Result;

    #[test]
    fn it_works() -> Result<()> {
        let f = ofd_rw::from_path("../samples/000.ofd")?;
        // let sigs = f.signatures_for_doc(0)?;
        // dbg!(sigs);
        // let anno = f.annotations_for_page(0,0)?;
        // dbg!(anno);
        let sig_vec = f.signature_for_doc(0)?.unwrap();
        let sig0 = &sig_vec[0];

        let path = sig0.resolve(&sig0.signed_value);
        dbg!(&path);
        let _sign = f.bytes(path)?;
        // File::create("../samples/SignedValue.dat")?.write_all(&_sign)?;
        Ok(())
    }
}
