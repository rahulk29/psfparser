use super::*;

static TRAN_EXAMPLE_PSFBIN: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/tranbin.tran.tran"
));

#[test]
fn test_header() {
    let toc = parse_toc(TRAN_EXAMPLE_PSFBIN);
    let header = parse_header(TRAN_EXAMPLE_PSFBIN, &toc.data[&SectionKind::Header]);
    println!("Header: {:?}", header);
}
