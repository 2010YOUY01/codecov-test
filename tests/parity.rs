use codecov_test::parity_label;

#[test]
fn labels_an_odd_number() {
    assert_eq!(parity_label(5), "odd");
}
