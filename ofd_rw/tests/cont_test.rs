use eyre::Result;
use ofd_rw::from_path;
// use
#[test]
fn test() -> Result<()> {
    let ofd = from_path("../samples/002.ofd")?;
    let e = ofd.entry()?;

    dbg!(e);
    Ok(())
}
