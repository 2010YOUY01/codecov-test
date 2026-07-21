/// Returns a simple label for a score.
pub fn score_label(score: u8) -> &'static str {
    if score >= 60 { "pass" } else { "retry" }
}
