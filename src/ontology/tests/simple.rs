#![allow(missing_docs)]

use ontology::Node;
use ontology::node::Name;
use ontology::node::{self};
use test_infra::read_fixture;

#[test]
fn parse() {
    let mut chrs = read_fixture("simple").unwrap().into_iter();

    let first: Node = chrs.next().unwrap();
    let expected = node::Builder::default()
        .name(
            "B-cell Acute Lymphoblastic Leukemia, PAX5 P80R"
                .parse::<Name>()
                .unwrap(),
        )
        .parent("B-cell Lymphoblastic Leukemia".parse::<Name>().unwrap())
        .try_build()
        .unwrap();
    assert_eq!(first, expected);
}
