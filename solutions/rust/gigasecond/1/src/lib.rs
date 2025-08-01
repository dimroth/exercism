use time::Duration;
use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start
        .checked_add(Duration::seconds(1_000_000_000))
        .expect("Date overflow when adding 1 billion seconds")
}
