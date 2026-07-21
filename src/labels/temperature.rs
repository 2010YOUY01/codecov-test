/// Returns a simple label for a temperature in Celsius.
pub fn temperature_label(celsius: i32) -> &'static str {
    if celsius <= 0 { "cold" } else { "warm" }
}
