/// Returns an access level based on a numeric score.
pub fn access_level(score: u8) -> &'static str {
    if score >= 90 {
        return "admin";
    }

    if score >= 60 {
        return "member";
    }

    "guest"
}
