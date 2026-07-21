use codecov_test::access::access_level;

#[test]
fn grants_admin_access_for_a_high_score() {
    assert_eq!(access_level(95), "admin");
}
