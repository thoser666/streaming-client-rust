extern crate chrono;
use chrono::{DateTime, TimeZone, Utc, FixedOffset, Local};

mod date_time_offset_extensions {
    use chrono::{DateTime, FixedOffset, TimeZone, Utc};

    pub struct DateTimeOffsetExtensions;

    impl DateTimeOffsetExtensions {
        pub fn from_utc_unix_time_milliseconds(milliseconds: i64) -> DateTime<Utc> {
            Utc.timestamp_millis(milliseconds)
        }

        pub fn from_utc_unix_time_seconds(seconds: i64) -> DateTime<Utc> {
            Utc.timestamp(seconds, 0)
        }

        pub fn from_utc_iso8601_string(date_time: &str) -> Result<DateTime<FixedOffset>, chrono::ParseError> {
            DateTime::parse_from_rfc3339(date_time)
        }

        pub fn to_utc_iso8601_string(date_time: &DateTime<Utc>) -> String {
            date_time.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
        }

        pub fn to_rfc3339_string(date_time: &DateTime<Utc>) -> String {
            format!("{}Z", Self::to_utc_iso8601_string(date_time))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use chrono::prelude::*;

        #[test]
        fn test_from_utc_unix_time_milliseconds() {
            let millis = 1609459200000i64; // Corresponds to 2021-01-01 00:00:00 UTC
            let date_time = DateTimeOffsetExtensions::from_utc_unix_time_milliseconds(millis);
            assert_eq!(date_time, Utc.ymd(2021, 1, 1).and_hms(0, 0, 0));
        }

        #[test]
        fn test_from_utc_unix_time_seconds() {
            let seconds = 1609459200i64; // Corresponds to 2021-01-01 00:00:00 UTC
            let date_time = DateTimeOffsetExtensions::from_utc_unix_time_seconds(seconds);
            assert_eq!(date_time, Utc.ymd(2021, 1, 1).and_hms(0, 0, 0));
        }

        #[test]
        fn test_from_utc_iso8601_string() {
            let iso_string = "2021-01-01T00:00:00+00:00";
            let date_time = DateTimeOffsetExtensions::from_utc_iso8601_string(iso_string).unwrap();
            assert_eq!(date_time, FixedOffset::east(0).ymd(2021, 1, 1).and_hms(0, 0, 0));
        }

        #[test]
        fn test_to_utc_iso8601_string() {
            let date_time = Utc.ymd(2021, 1, 1).and_hms(0, 0, 0);
            let iso_string = DateTimeOffsetExtensions::to_utc_iso8601_string(&date_time);
            assert_eq!(iso_string, "2021-01-01T00:00:00Z");
        }

        #[test]
        fn test_to_rfc3339_string() {
            let date_time = Utc.ymd(2021, 1, 1).and_hms(0, 0, 0);
            let rfc3339_string = DateTimeOffsetExtensions::to_rfc3339_string(&date_time);
            assert_eq!(rfc3339_string, "2021-01-01T00:00:00Z");
        }
    }
}
fn main() {
    // The main function remains empty, as we use this file mainly for the library functionality and tests.
}
