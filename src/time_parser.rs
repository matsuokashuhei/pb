//! Time parsing functionality for the pb CLI tool
//!
//! This module provides functions to parse various time formats into
//! `NaiveDateTime` objects for use in progress bar calculations.

use chrono::{NaiveDate, NaiveDateTime, Duration};
use regex::Regex;
use crate::error::PbError;

/// Parse a date string in YYYY-MM-DD format
///
/// This function parses date strings in the ISO 8601 date format (YYYY-MM-DD)
/// and converts them to `NaiveDateTime` with time set to 00:00:00.
///
/// The function enforces strict formatting requirements:
/// - Year must be exactly 4 digits
/// - Month and day must be 1-2 digits (zero-padding is optional)
/// - Uses hyphens as separators
///
/// # Arguments
///
/// * `input` - A string slice containing the date in YYYY-MM-DD format
///
/// # Returns
///
/// * `Ok(NaiveDateTime)` - Successfully parsed date with time 00:00:00
/// * `Err(PbError)` - Invalid date format or invalid date
///
/// # Examples
///
/// ```
/// use pb::time_parser::parse_date;
///
/// // Valid date
/// let result = parse_date("2025-07-21");
/// assert!(result.is_ok());
///
/// // Invalid format
/// let result = parse_date("25-07-21");
/// assert!(result.is_err());
///
/// // Invalid date
/// let result = parse_date("2025-02-30");
/// assert!(result.is_err());
/// ```
pub fn parse_date(input: &str) -> Result<NaiveDateTime, PbError> {
    // First, validate the basic format using regex to ensure 4-digit year
    if !input.chars().all(|c| c.is_ascii_digit() || c == '-') {
        return Err(PbError::InvalidTimeFormat {
            input: input.to_string()
        });
    }

    // Split by hyphens and validate format
    let parts: Vec<&str> = input.split('-').collect();
    if parts.len() != 3 {
        return Err(PbError::InvalidTimeFormat {
            input: input.to_string()
        });
    }

    // Validate year is exactly 4 digits
    if parts[0].len() != 4 {
        return Err(PbError::InvalidTimeFormat {
            input: input.to_string()
        });
    }

    // Validate month and day are 1-2 digits
    if parts[1].is_empty() || parts[1].len() > 2 || parts[2].is_empty() || parts[2].len() > 2 {
        return Err(PbError::InvalidTimeFormat {
            input: input.to_string()
        });
    }

    // Parse the date string using chrono's built-in parser
    NaiveDate::parse_from_str(input, "%Y-%m-%d")
        .map(|date| {
            // Convert to NaiveDateTime with time 00:00:00
            // Using and_hms_opt(0, 0, 0).unwrap() is safe here because
            // 00:00:00 is always a valid time
            date.and_hms_opt(0, 0, 0).unwrap()
        })
        .map_err(|_| PbError::InvalidTimeFormat {
            input: input.to_string()
        })
}

/// Parse a relative time string and convert to absolute timestamp
///
/// This function parses relative time strings in formats like `30m`, `2h`, `1d`
/// and converts them to absolute timestamps by adding the duration to a base time.
///
/// Supported formats:
/// - `30m` - 30 minutes
/// - `2h` - 2 hours
/// - `1d` - 1 day
///
/// The function enforces strict formatting requirements:
/// - Must match pattern `^(\d+)([mhd])$` exactly
/// - Amount must be between 1 and 999 (inclusive)
/// - Only supports units: m (minutes), h (hours), d (days)
///
/// # Arguments
///
/// * `input` - A string slice containing the relative time (e.g., "30m", "2h", "1d")
/// * `base_time` - The base time to add the relative duration to
///
/// # Returns
///
/// * `Ok(NaiveDateTime)` - Successfully parsed relative time added to base_time
/// * `Err(PbError)` - Invalid relative time format or calculation overflow
///
/// # Examples
///
/// ```
/// use pb::time_parser::parse_relative_time;
/// use chrono::NaiveDateTime;
///
/// let base = NaiveDateTime::parse_from_str("2025-07-21 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
///
/// // Valid relative times
/// let result = parse_relative_time("30m", base);
/// assert!(result.is_ok());
///
/// let result = parse_relative_time("2h", base);
/// assert!(result.is_ok());
///
/// let result = parse_relative_time("1d", base);
/// assert!(result.is_ok());
///
/// // Invalid format
/// let result = parse_relative_time("30", base);
/// assert!(result.is_err());
///
/// // Invalid unit
/// let result = parse_relative_time("30x", base);
/// assert!(result.is_err());
/// ```
pub fn parse_relative_time(input: &str, base_time: NaiveDateTime) -> Result<NaiveDateTime, PbError> {
    // Create regex pattern for relative time formats: ^(\d+)([mhd])$
    let re = Regex::new(r"^(\d+)([mhd])$").unwrap();

    if let Some(captures) = re.captures(input) {
        // Parse the numeric amount
        let amount: i64 = captures[1].parse()
            .map_err(|_| PbError::InvalidRelativeTimeFormat {
                input: input.to_string()
            })?;

        let unit = &captures[2];

        // Validate range (1-999)
        if !(1..=999).contains(&amount) {
            return Err(PbError::InvalidRelativeTimeFormat {
                input: input.to_string()
            });
        }

        // Convert to seconds based on unit
        let seconds = match unit {
            "m" => amount * 60,        // minutes to seconds
            "h" => amount * 3600,      // hours to seconds
            "d" => amount * 86400,     // days to seconds
            _ => return Err(PbError::InvalidRelativeTimeFormat {
                input: input.to_string()
            }),
        };

        // Add duration to base time with overflow checking
        base_time.checked_add_signed(Duration::seconds(seconds))
            .ok_or_else(|| PbError::InvalidRelativeTimeFormat {
                input: input.to_string()
            })
    } else {
        Err(PbError::InvalidRelativeTimeFormat {
            input: input.to_string()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Timelike};

    #[test]
    fn test_parse_valid_dates() {
        // Test basic valid date
        let result = parse_date("2025-07-21");
        assert!(result.is_ok());
        let datetime = result.unwrap();
        assert_eq!(datetime.date().year(), 2025);
        assert_eq!(datetime.date().month(), 7);
        assert_eq!(datetime.date().day(), 21);
        assert_eq!(datetime.time().hour(), 0);
        assert_eq!(datetime.time().minute(), 0);
        assert_eq!(datetime.time().second(), 0);

        // Test leap year date
        let result = parse_date("2024-02-29");
        assert!(result.is_ok());
        let datetime = result.unwrap();
        assert_eq!(datetime.date().year(), 2024);
        assert_eq!(datetime.date().month(), 2);
        assert_eq!(datetime.date().day(), 29);

        // Test year boundaries
        let result = parse_date("2020-12-31");
        assert!(result.is_ok());

        let result = parse_date("2025-01-01");
        assert!(result.is_ok());

        // Test different months
        let result = parse_date("2025-06-15");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_invalid_formats() {
        // Wrong year format
        let result = parse_date("25-07-21");
        assert!(result.is_err());
        if let Err(PbError::InvalidTimeFormat { input }) = result {
            assert_eq!(input, "25-07-21");
        } else {
            panic!("Expected InvalidTimeFormat error");
        }

        // Wrong format entirely
        let result = parse_date("July 21, 2025");
        assert!(result.is_err());

        // Wrong order
        let result = parse_date("21-07-2025");
        assert!(result.is_err());

        // Empty string
        let result = parse_date("");
        assert!(result.is_err());

        // Incomplete date
        let result = parse_date("2025-07");
        assert!(result.is_err());

        let result = parse_date("2025");
        assert!(result.is_err());

        // Too many components
        let result = parse_date("2025-07-21-extra");
        assert!(result.is_err());

        // Non-numeric components
        let result = parse_date("abcd-07-21");
        assert!(result.is_err());

        let result = parse_date("2025-ab-21");
        assert!(result.is_err());

        let result = parse_date("2025-07-cd");
        assert!(result.is_err());

        // Zero values (not allowed)
        let result = parse_date("2025-0-21");
        assert!(result.is_err());

        let result = parse_date("2025-07-0");
        assert!(result.is_err());
    }

    #[test]
    fn test_flexible_date_formats() {
        // chrono accepts these formats, which is fine for our use case
        let result = parse_date("2025-7-21");
        assert!(result.is_ok());
        let datetime = result.unwrap();
        assert_eq!(datetime.date().month(), 7);
        assert_eq!(datetime.date().day(), 21);

        let result = parse_date("2025-07-1");
        assert!(result.is_ok());
        let datetime = result.unwrap();
        assert_eq!(datetime.date().month(), 7);
        assert_eq!(datetime.date().day(), 1);

        let result = parse_date("2025-1-1");
        assert!(result.is_ok());
        let datetime = result.unwrap();
        assert_eq!(datetime.date().month(), 1);
        assert_eq!(datetime.date().day(), 1);
    }

    #[test]
    fn test_parse_invalid_dates() {
        // Invalid month
        let result = parse_date("2025-13-01");
        assert!(result.is_err());

        let result = parse_date("2025-00-15");
        assert!(result.is_err());

        // Invalid day for February
        let result = parse_date("2025-02-30");
        assert!(result.is_err());

        // Non-leap year February 29th
        let result = parse_date("2023-02-29");
        assert!(result.is_err());

        // Invalid day for April (30 days)
        let result = parse_date("2025-04-31");
        assert!(result.is_err());

        // Invalid day - zero
        let result = parse_date("2025-05-00");
        assert!(result.is_err());

        // Invalid day - too high
        let result = parse_date("2025-01-32");
        assert!(result.is_err());

        // Test all months with invalid day 31 for months that don't have 31 days
        let result = parse_date("2025-02-31"); // February
        assert!(result.is_err());

        let result = parse_date("2025-04-31"); // April
        assert!(result.is_err());

        let result = parse_date("2025-06-31"); // June
        assert!(result.is_err());

        let result = parse_date("2025-09-31"); // September
        assert!(result.is_err());

        let result = parse_date("2025-11-31"); // November
        assert!(result.is_err());
    }

    #[test]
    fn test_leap_year_edge_cases() {
        // Test leap year - divisible by 4
        let result = parse_date("2024-02-29");
        assert!(result.is_ok());

        // Test non-leap year - not divisible by 4
        let result = parse_date("2023-02-29");
        assert!(result.is_err());

        // Test century year not divisible by 400 (not leap year)
        let result = parse_date("1900-02-29");
        assert!(result.is_err());

        // Test century year divisible by 400 (leap year)
        let result = parse_date("2000-02-29");
        assert!(result.is_ok());
    }

    #[test]
    fn test_date_time_conversion() {
        let result = parse_date("2025-07-21").unwrap();

        // Check that time is set to 00:00:00
        assert_eq!(result.time().hour(), 0);
        assert_eq!(result.time().minute(), 0);
        assert_eq!(result.time().second(), 0);
        assert_eq!(result.time().nanosecond(), 0);

        // Check date components
        assert_eq!(result.date().year(), 2025);
        assert_eq!(result.date().month(), 7);
        assert_eq!(result.date().day(), 21);
    }

    #[test]
    fn test_error_message_format() {
        let result = parse_date("invalid-date");
        assert!(result.is_err());

        if let Err(PbError::InvalidTimeFormat { input }) = result {
            assert_eq!(input, "invalid-date");
        } else {
            panic!("Expected InvalidTimeFormat error with input");
        }
    }

    #[test]
    fn test_performance_repeated_parsing() {
        use std::time::Instant;

        let start = Instant::now();

        // Parse the same date 1000 times
        for _ in 0..1000 {
            let result = parse_date("2025-07-21");
            assert!(result.is_ok());
        }

        let duration = start.elapsed();

        // Should complete well under 1 second for 1000 parses
        assert!(duration.as_millis() < 1000, "Parsing took too long: {:?}", duration);
    }

    #[test]
    fn test_extreme_dates() {
        // Test very old date
        let result = parse_date("0001-01-01");
        assert!(result.is_ok());

        // Test far future date
        let result = parse_date("9999-12-31");
        assert!(result.is_ok());
    }

    // Tests for parse_relative_time function

    #[test]
    fn test_parse_valid_relative_times() {
        let base_time = NaiveDateTime::parse_from_str("2025-07-21 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();

        // Test minutes
        let result = parse_relative_time("30m", base_time);
        assert!(result.is_ok());
        let expected = base_time + Duration::minutes(30);
        assert_eq!(result.unwrap(), expected);

        // Test hours
        let result = parse_relative_time("2h", base_time);
        assert!(result.is_ok());
        let expected = base_time + Duration::hours(2);
        assert_eq!(result.unwrap(), expected);

        // Test days
        let result = parse_relative_time("1d", base_time);
        assert!(result.is_ok());
        let expected = base_time + Duration::days(1);
        assert_eq!(result.unwrap(), expected);

        // Test larger values
        let result = parse_relative_time("120m", base_time);
        assert!(result.is_ok());
        let expected = base_time + Duration::minutes(120);
        assert_eq!(result.unwrap(), expected);

        let result = parse_relative_time("24h", base_time);
        assert!(result.is_ok());
        let expected = base_time + Duration::hours(24);
        assert_eq!(result.unwrap(), expected);

        // Test boundary values
        let result = parse_relative_time("1m", base_time);
        assert!(result.is_ok());
        let expected = base_time + Duration::minutes(1);
        assert_eq!(result.unwrap(), expected);

        let result = parse_relative_time("999m", base_time);
        assert!(result.is_ok());
        let expected = base_time + Duration::minutes(999);
        assert_eq!(result.unwrap(), expected);

        let result = parse_relative_time("999h", base_time);
        assert!(result.is_ok());
        let expected = base_time + Duration::hours(999);
        assert_eq!(result.unwrap(), expected);

        let result = parse_relative_time("999d", base_time);
        assert!(result.is_ok());
        let expected = base_time + Duration::days(999);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_parse_invalid_relative_time_formats() {
        let base_time = NaiveDateTime::parse_from_str("2025-07-21 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();

        // Missing unit
        let result = parse_relative_time("30", base_time);
        assert!(result.is_err());
        if let Err(PbError::InvalidRelativeTimeFormat { input }) = result {
            assert_eq!(input, "30");
        } else {
            panic!("Expected InvalidRelativeTimeFormat error");
        }

        // Wrong order
        let result = parse_relative_time("m30", base_time);
        assert!(result.is_err());

        // Invalid unit
        let result = parse_relative_time("30x", base_time);
        assert!(result.is_err());

        // Verbose unit not supported
        let result = parse_relative_time("30mins", base_time);
        assert!(result.is_err());

        // Decimal not supported
        let result = parse_relative_time("2.5h", base_time);
        assert!(result.is_err());

        // Empty string
        let result = parse_relative_time("", base_time);
        assert!(result.is_err());

        // Only unit
        let result = parse_relative_time("m", base_time);
        assert!(result.is_err());

        // Only number
        let result = parse_relative_time("30", base_time);
        assert!(result.is_err());

        // Multiple units
        let result = parse_relative_time("30mh", base_time);
        assert!(result.is_err());

        // Negative values (not supported by regex)
        let result = parse_relative_time("-5h", base_time);
        assert!(result.is_err());

        // Plus sign
        let result = parse_relative_time("+5h", base_time);
        assert!(result.is_err());

        // Spaces
        let result = parse_relative_time("30 m", base_time);
        assert!(result.is_err());

        let result = parse_relative_time(" 30m", base_time);
        assert!(result.is_err());

        let result = parse_relative_time("30m ", base_time);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_relative_time_range_validation() {
        let base_time = NaiveDateTime::parse_from_str("2025-07-21 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();

        // Zero values not allowed
        let result = parse_relative_time("0m", base_time);
        assert!(result.is_err());
        if let Err(PbError::InvalidRelativeTimeFormat { input }) = result {
            assert_eq!(input, "0m");
        } else {
            panic!("Expected InvalidRelativeTimeFormat error");
        }

        let result = parse_relative_time("0h", base_time);
        assert!(result.is_err());

        let result = parse_relative_time("0d", base_time);
        assert!(result.is_err());

        // Values exceeding maximum range (999) not allowed
        let result = parse_relative_time("1000m", base_time);
        assert!(result.is_err());

        let result = parse_relative_time("1000h", base_time);
        assert!(result.is_err());

        let result = parse_relative_time("1000d", base_time);
        assert!(result.is_err());

        // Very large numbers
        let result = parse_relative_time("99999d", base_time);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_relative_time_edge_cases() {
        let _base_time = NaiveDateTime::parse_from_str("2025-07-21 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();

        // Test with different base times
        let early_base = NaiveDateTime::parse_from_str("2000-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let result = parse_relative_time("1d", early_base);
        assert!(result.is_ok());
        let expected = early_base + Duration::days(1);
        assert_eq!(result.unwrap(), expected);

        // Test near end of time range (potential overflow)
        let late_base = NaiveDateTime::parse_from_str("9999-12-30 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let result = parse_relative_time("1d", late_base);
        assert!(result.is_ok());

        // Test overflow case - try to go past the maximum chrono date
        // NaiveDateTime max is around year 262142, but we'll use a more conservative test
        let very_late_base = NaiveDateTime::parse_from_str("9999-12-31 23:59:59", "%Y-%m-%d %H:%M:%S").unwrap();
        let result = parse_relative_time("1d", very_late_base);
        // This might succeed or fail depending on chrono's limits, so let's just test it works
        // If it succeeds, that's fine - chrono handles it. If it fails, that's also fine.
        // The important thing is that our function doesn't panic.
        match result {
            Ok(_) => {
                // chrono handled it gracefully
            }
            Err(PbError::InvalidRelativeTimeFormat { input }) => {
                assert_eq!(input, "1d");
            }
            _ => panic!("Expected either success or InvalidRelativeTimeFormat error"),
        }
    }

    #[test]
    fn test_parse_relative_time_unit_conversions() {
        let base_time = NaiveDateTime::parse_from_str("2025-07-21 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();

        // Test 60 minutes = 1 hour
        let result_60m = parse_relative_time("60m", base_time).unwrap();
        let result_1h = parse_relative_time("1h", base_time).unwrap();
        assert_eq!(result_60m, result_1h);

        // Test 24 hours = 1 day
        let result_24h = parse_relative_time("24h", base_time).unwrap();
        let result_1d = parse_relative_time("1d", base_time).unwrap();
        assert_eq!(result_24h, result_1d);

        // Test 1440 minutes = 1 day
        let result_1440m = parse_relative_time("1440m", base_time);
        // This should fail because 1440 > 999
        assert!(result_1440m.is_err());

        // But we can test a smaller equivalent: 120m = 2h
        let result_120m = parse_relative_time("120m", base_time).unwrap();
        let result_2h = parse_relative_time("2h", base_time).unwrap();
        assert_eq!(result_120m, result_2h);
    }

    #[test]
    fn test_parse_relative_time_error_messages() {
        let base_time = NaiveDateTime::parse_from_str("2025-07-21 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();

        let test_cases = vec![
            "invalid",
            "30x",
            "0m",
            "1000d",
            "30",
            "m30",
            "30mins",
            "2.5h",
        ];

        for invalid_input in test_cases {
            let result = parse_relative_time(invalid_input, base_time);
            assert!(result.is_err());
            if let Err(PbError::InvalidRelativeTimeFormat { input }) = result {
                assert_eq!(input, invalid_input);
            } else {
                panic!("Expected InvalidRelativeTimeFormat error for input: {}", invalid_input);
            }
        }
    }

    #[test]
    fn test_parse_relative_time_performance() {
        use std::time::Instant;

        let base_time = NaiveDateTime::parse_from_str("2025-07-21 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let start = Instant::now();

        // Parse the same relative time 1000 times
        for _ in 0..1000 {
            let result = parse_relative_time("30m", base_time);
            assert!(result.is_ok());
        }

        let duration = start.elapsed();

        // Should complete within reasonable time for 1000 parses (allowing for Docker overhead)
        assert!(duration.as_millis() < 2000, "Relative time parsing took too long: {:?}", duration);
    }

    #[test]
    fn test_parse_relative_time_different_units() {
        let base_time = NaiveDateTime::parse_from_str("2025-07-21 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();

        // Test each unit individually to ensure correct conversion

        // Minutes
        let result = parse_relative_time("5m", base_time);
        assert!(result.is_ok());
        let expected = base_time + Duration::seconds(5 * 60);
        assert_eq!(result.unwrap(), expected);

        // Hours
        let result = parse_relative_time("3h", base_time);
        assert!(result.is_ok());
        let expected = base_time + Duration::seconds(3 * 3600);
        assert_eq!(result.unwrap(), expected);

        // Days
        let result = parse_relative_time("2d", base_time);
        assert!(result.is_ok());
        let expected = base_time + Duration::seconds(2 * 86400);
        assert_eq!(result.unwrap(), expected);
    }
}
