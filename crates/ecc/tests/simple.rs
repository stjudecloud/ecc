#![allow(missing_docs)]

use std::num::NonZeroU64;

use chrono::DateTime;
use ecc::Characteristic;
use ecc::Identifier;
use test_infra::read_fixture;

#[test]
fn parse() {
    let mut chrs = read_fixture("simple").unwrap().into_iter();

    ////////////////////////////////////////////////////////////////////////////
    // First characteristic
    ////////////////////////////////////////////////////////////////////////////

    let first = chrs.next().unwrap();
    assert!(matches!(first, Characteristic::Adopted { .. }));

    let expected = Identifier::Morphological(NonZeroU64::try_from(1).unwrap());
    let actual = first.identifier().unwrap();
    assert_eq!(actual, &expected);
    assert_eq!(actual.to_string(), "ECC-MORPH-000001");

    assert_eq!(
        first.rfc().unwrap().as_str(),
        "https://github.com/stjudecloud/ecc/issues/1"
    );

    assert_eq!(
        first.adoption_date().unwrap(),
        &DateTime::from_timestamp(0, 0).unwrap()
    );

    ////////////////////////////////////////////////////////////////////////////
    // Second characteristic
    ////////////////////////////////////////////////////////////////////////////

    let second = chrs.next().unwrap();
    assert!(matches!(second, Characteristic::Provisional { .. }));

    let expected = Identifier::Molecular(NonZeroU64::try_from(1).unwrap());
    let actual = second.identifier().unwrap();
    assert_eq!(actual, &expected);
    assert_eq!(actual.to_string(), "ECC-MOLEC-000001");

    assert_eq!(
        second.rfc().unwrap().as_str(),
        "https://github.com/stjudecloud/ecc/issues/2"
    );

    ////////////////////////////////////////////////////////////////////////////
    // Third characteristic
    ////////////////////////////////////////////////////////////////////////////

    let third = chrs.next().unwrap();
    assert!(matches!(third, Characteristic::Proposed { .. }));

    assert_eq!(
        third.rfc().unwrap().as_str(),
        "https://github.com/stjudecloud/ecc/issues/3"
    );
}
