use codecov_test::describe_number;

#[test]
fn repeats_an_existing_positive_case() {
    assert_eq!(describe_number(7), "7 is positive");
}
