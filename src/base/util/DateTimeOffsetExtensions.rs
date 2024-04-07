extern crate chrono;
use chrono::{DateTime, TimeZone, Utc, FixedOffset, Local};

pub struct DateTimeOffsetExtensions;

impl DateTimeOffsetExtensions {
    /// Creates a DateTime<Utc> from a UTC Unix time in milliseconds.
    pub fn from_utc_unix_time_milliseconds(milliseconds: i64) -> DateTime<Utc> {
        Utc.timestamp_millis(milliseconds)
    }

    /// Creates a DateTime<Utc> from a UTC Unix time in seconds.
    pub fn from_utc_unix_time_seconds(seconds: i64) -> DateTime<Utc> {
        Utc.timestamp(seconds, 0)
    }

    /// Creates a DateTime<FixedOffset> from an ISO 8601 string.
    pub fn from_utc_iso8601_string(date_time: &str) -> Result<DateTime<FixedOffset>, chrono::ParseError> {
        DateTime::parse_from_rfc3339(date_time)
    }

    /// Converts a DateTime to an ISO 8601 string.
    pub fn to_utc_iso8601_string(date_time: &DateTime<Utc>) -> String {
        date_time.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
    }

    /// Converts a DateTime to an RFC 3339 string.
    pub fn to_rfc3339_string(date_time: &DateTime<Utc>) -> String {
        format!("{}Z", Self::to_utc_iso8601_string(date_time))
    }
}

fn main() {
    let millis = DateTimeOffsetExtensions::from_utc_unix_time_milliseconds(1609459200000);
    println!("DateTime from millis: {}", millis);

    let seconds = DateTimeOffsetExtensions::from_utc_unix_time_seconds(1609459200);
    println!("DateTime from seconds: {}", seconds);

    let iso_string = "2021-01-01T00:00:00+00:00";
    match DateTimeOffsetExtensions::from_utc_iso8601_string(iso_string) {
        Ok(date) => println!("DateTime from ISO 8601 string: {}", date),
        Err(e) => println!("Error parsing date: {:?}", e),
    };

    let rfc3339 = DateTimeOffsetExtensions::to_rfc3339_string(&seconds);
    println!("RFC 3339 formatted date: {}", rfc3339);
}
