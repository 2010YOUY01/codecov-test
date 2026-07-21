use codecov_test::labels::{score_label, temperature_label};

#[test]
fn labels_scores() {
    assert_eq!(score_label(80), "pass");
    assert_eq!(score_label(40), "retry");
}

#[test]
fn labels_temperatures() {
    assert_eq!(temperature_label(-5), "cold");
    assert_eq!(temperature_label(20), "warm");
}
