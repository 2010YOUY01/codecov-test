/// Returns a short description of an integer.
pub fn describe_number(number: i32) -> String {
    let kind = if number > 0 {
        "positive"
    } else if number < 0 {
        "negative"
    } else {
        "zero"
    };

    format!("{number} is {kind}")
}

#[cfg(test)]
mod tests {
    use super::describe_number;

    #[test]
    fn describes_a_positive_number() {
        assert_eq!(describe_number(7), "7 is positive");
    }

    #[test]
    fn describes_zero() {
        assert_eq!(describe_number(0), "0 is zero");
    }
}
