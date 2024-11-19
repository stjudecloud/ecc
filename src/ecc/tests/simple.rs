#![allow(missing_docs)]

use std::num::NonZeroU64;

use ecc::Characteristic;
use ecc::Identifier;

mod common;

#[test]
fn parse() {
    let chr: Characteristic = common::read_fixture("simple").unwrap();

    let expected = Identifier::Morphological(NonZeroU64::try_from(1).unwrap());
    let actual = chr.identifier().unwrap();
    assert_eq!(actual, &expected);
    assert_eq!(actual.to_string(), "ECC-MORPH-000001");
}
