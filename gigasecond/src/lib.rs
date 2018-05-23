extern crate chrono;
use chrono::{NaiveDateTime, DateTime, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let billion = 1_000_000_000;
    let seconds = start.timestamp();
    let new_time = billion + seconds;
    let end = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(new_time, 0), Utc);
    return end
}
