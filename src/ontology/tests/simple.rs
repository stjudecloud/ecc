#![allow(missing_docs)]

use ontology::Node;
use ontology::node::Name;
use test_infra::read_fixture;

#[test]
fn parse() {
    let mut chrs = read_fixture("simple").unwrap().into_iter();

    let first: Node = chrs.next().unwrap();
    assert_eq!(
        first,
        Node::new(
            "B-cell Acute Lymphoblastic Leukemia, PAX5 P80R"
                .parse::<Name>()
                .unwrap(),
            "B-cell Lymphoblastic Leukemia".parse::<Name>().unwrap()
        )
    );
}
