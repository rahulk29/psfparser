use crate::analysis::transient::TransientData;
use crate::ascii::parse as ascii_parse;
use crate::ascii::tests::VDIV_SIN_PSF;
use crate::binary::parse as bin_parse;

use crate::binary::tests::{self, VDIV_SIN_PSFBIN};

#[test]
fn parses_vdiv_sin() {
    let ast = ascii_parse(VDIV_SIN_PSF).expect("Failed to parse transient PSF file");
    let ascii_data = TransientData::from_ascii(&ast);
    let ast = bin_parse(VDIV_SIN_PSFBIN).expect("Failed to parse transient PSF file");
    let bin_data = TransientData::from_binary(ast);

    assert!(ascii_data.approx_eq(&bin_data, 1e-12));
}