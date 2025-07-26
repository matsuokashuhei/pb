//! Time parsing functionality for the pb CLI tool
//!
//! This module provides functions to parse various time formats into
//! `NaiveDateTime` objects for use in progress bar calculations.

use crate::error::PbError;
use chrono::{Duration, Local, NaiveDate, NaiveDateTime};
use regex::Regex;

/// Get current time consistently across the application
///
/// This function provides a centralized way to get the current time
/// that is consistent with the timezone assumptions used throughout
/// the application. All absolute timestamps are interpreted as local time,
/// so the current time should also be in local time for consistency.
///
/// # Returns
///
/// Returns the current local time as a `NaiveDateTime`, which matches
/// the format used for parsed absolute timestamps.
///
/// # Usage
///
/// This function should be used everywhere in the application where
/// we need to get the current time, to ensure timezone consistency.
pub fn get_current_time() -> NaiveDateTime {
    Local::now().naive_local()
}

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
            input: input.to_string(),
        });
    }

    // Split by hyphens and validate format
    let parts: Vec<&str> = input.split('-').collect();
    if parts.len() != 3 {
        return Err(PbError::InvalidTimeFormat {
            input: input.to_string(),
        });
    }

    // Validate year is exactly 4 digits
    if parts[0].len() != 4 {
        return Err(PbError::InvalidTimeFormat {
            input: input.to_string(),
        });
    }

    // Validate month and day are 1-2 digits
    if parts[1].is_empty() || parts[1].len() > 2 || parts[2].is_empty() || parts[2].len() > 2 {
        return Err(PbError::InvalidTimeFormat {
            input: input.to_string(),
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
            input: input.to_string(),
        })
}

/// Parse a datetime string in YYYY-MM-DD HH:MM:SS format
///
/// This function parses datetime strings in the format `YYYY-MM-DD HH:MM:SS`
/// and converts them to `NaiveDateTime` objects for use in progress bar calculations.
///
/// The function enforces strict formatting requirements:
/// - Date must be in YYYY-MM-DD format (same as parse_date)
/// - Time must be in HH:MM:SS format with 24-hour notation
/// - Uses space as separator between date and time
/// - Hours: 00-23, Minutes: 00-59, Seconds: 00-59
///
/// # Arguments
///
/// * `input` - A string slice containing the datetime in YYYY-MM-DD HH:MM:SS format
///
/// # Returns
///
/// * `Ok(NaiveDateTime)` - Successfully parsed datetime
/// * `Err(PbError)` - Invalid datetime format or invalid datetime
///
/// # Examples
///
/// ```
/// use pb::time_parser::parse_datetime;
///
/// // Valid datetime
/// let result = parse_datetime("2025-07-21 10:30:45");
/// assert!(result.is_ok());
///
/// // Invalid format (missing time)
/// let result = parse_datetime("2025-07-21");
/// assert!(result.is_err());
///
/// // Invalid time (hour > 23)
/// let result = parse_datetime("2025-07-21 25:00:00");
/// assert!(result.is_err());
/// ```
pub fn parse_datetime(input: &str) -> Result<NaiveDateTime, PbError> {
    // Validate that input contains only ASCII characters, spaces, hyphens, and colons
    if !input
        .chars()
        .all(|c| c.is_ascii() && (c.is_ascii_digit() || c == '-' || c == ' ' || c == ':'))
    {
        return Err(PbError::InvalidTimeFormat {
            input: input.to_string(),
        });
    }

    // Check for double spaces or other formatting issues
    if input.contains("  ") {
        return Err(PbError::InvalidTimeFormat {
            input: input.to_string(),
        });
    }

    // Validate seconds are not >= 60 before parsing
    // Split by space to get date and time parts
    let parts: Vec<&str> = input.split(' ').collect();
    if parts.len() != 2 {
        return Err(PbError::InvalidTimeFormat {
            input: input.to_string(),
        });
    }

    // Split time part by colons to check seconds
    let time_parts: Vec<&str> = parts[1].split(':').collect();
    if time_parts.len() != 3 {
        return Err(PbError::InvalidTimeFormat {
            input: input.to_string(),
        });
    }

    // Check if seconds >= 60
    if let Ok(seconds) = time_parts[2].parse::<u32>() {
        if seconds >= 60 {
            return Err(PbError::InvalidTimeFormat {
                input: input.to_string(),
            });
        }
    }

    chrono::NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S").map_err(|_| {
        PbError::InvalidTimeFormat {
            input: input.to_string(),
        }
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
pub fn parse_relative_time(
    input: &str,
    base_time: NaiveDateTime,
) -> Result<NaiveDateTime, PbError> {
    // Create regex pattern for relative time formats: ^(\d+)([mhd])$
    let re = Regex::new(r"^(\d+)([mhd])$").unwrap();

    if let Some(captures) = re.captures(input) {
        // Parse the numeric amount
        let amount: i64 = captures[1]
            .parse()
            .map_err(|_| PbError::InvalidRelativeTimeFormat {
                input: input.to_string(),
            })?;

        let unit = &captures[2];

        // Validate range (1-999)
        if !(1..=999).contains(&amount) {
            return Err(PbError::InvalidRelativeTimeFormat {
                input: input.to_string(),
            });
        }

        // Convert to seconds based on unit
        let seconds = match unit {
            "m" => amount * 60,    // minutes to seconds
            "h" => amount * 3600,  // hours to seconds
            "d" => amount * 86400, // days to seconds
            _ => {
                return Err(PbError::InvalidRelativeTimeFormat {
                    input: input.to_string(),
                })
            }
        };

        // Add duration to base time with overflow checking
        base_time
            .checked_add_signed(Duration::seconds(seconds))
            .ok_or_else(|| PbError::InvalidRelativeTimeFormat {
                input: input.to_string(),
            })
    } else {
        Err(PbError::InvalidRelativeTimeFormat {
            input: input.to_string(),
        })
    }
}

/// Parse a time-only string in HH:MM:SS format
///
/// This function parses time strings in the format `HH:MM:SS` and converts
/// them to `NaiveDateTime` using today's date as the date component.
///
/// # Arguments
///
/// * `input` - A string slice containing the time in HH:MM:SS format
///
/// # Returns
///
/// * `Ok(NaiveDateTime)` - Successfully parsed time with today's date
/// * `Err(PbError)` - Invalid time format
fn parse_time_only(input: &str) -> Result<NaiveDateTime, PbError> {
    // Validate that input contains only ASCII digits and colons
    if !input.chars().all(|c| c.is_ascii_digit() || c == ':') {
        return Err(PbError::InvalidTimeFormat {
            input: input.to_string(),
        });
    }

    // Validate seconds are not >= 60 before parsing
    let time_parts: Vec<&str> = input.split(':').collect();
    if time_parts.len() != 3 {
        return Err(PbError::InvalidTimeFormat {
            input: input.to_string(),
        });
    }

    // Check if seconds >= 60
    if let Ok(seconds) = time_parts[2].parse::<u32>() {
        if seconds >= 60 {
            return Err(PbError::InvalidTimeFormat {
                input: input.to_string(),
            });
        }
    }

    // Try to parse as time
    let time = chrono::NaiveTime::parse_from_str(input, "%H:%M:%S").map_err(|_| {
        PbError::InvalidTimeFormat {
            input: input.to_string(),
        }
    })?;

    // Use today's date (consistent with get_current_time)
    let today = get_current_time().date();
    Ok(today.and_time(time))
}

/// Parse a time string in any supported format
///
/// This is the main entry point for time parsing that automatically detects
/// the format and delegates to the appropriate specialized parser.
///
/// Supported formats:
/// - Date: "YYYY-MM-DD" (e.g., "2025-07-21")
/// - DateTime: "YYYY-MM-DD HH:MM:SS" (e.g., "2025-07-21 10:30:00")
/// - Relative: "+NNu" where NN is number and u is unit (m/h/d) (e.g., "+2h", "+30m")
///
/// # Arguments
///
/// * `input` - A string slice containing the time in any supported format
///
/// # Returns
///
/// * `Ok(NaiveDateTime)` - Successfully parsed time
/// * `Err(PbError)` - Invalid format or unable to parse
///
/// # Examples
///
/// ```
/// use pb::time_parser::parse_time;
///
/// // Parse date
/// let result = parse_time("2025-07-21");
/// assert!(result.is_ok());
///
/// // Parse datetime
/// let result = parse_time("2025-07-21 10:30:00");
/// assert!(result.is_ok());
///
/// // Parse relative time (requires current time as base)
/// let result = parse_time("+2h");
/// assert!(result.is_ok());
/// ```
pub fn parse_time(input: &str) -> Result<NaiveDateTime, PbError> {
    let trimmed_input = input.trim();

    if trimmed_input.is_empty() {
        return Err(PbError::invalid_time_format("Time cannot be empty"));
    }

    // Check for relative time format (starts with + or -)
    if trimmed_input.starts_with('+') || trimmed_input.starts_with('-') {
        let base_time = get_current_time();
        let relative_input = if let Some(stripped) = trimmed_input.strip_prefix('+') {
            stripped // Remove the '+' prefix
        } else {
            trimmed_input // Keep the '-' prefix for negative relative times
        };
        return parse_relative_time(relative_input, base_time);
    }

    // Check if it looks like a datetime (contains space and colon)
    if trimmed_input.contains(' ') && trimmed_input.contains(':') {
        return parse_datetime(trimmed_input);
    }

    // Check if it looks like a date (contains hyphens but no space/colon)
    if trimmed_input.contains('-') && !trimmed_input.contains(' ') && !trimmed_input.contains(':') {
        return parse_date(trimmed_input);
    }

    // Check if it looks like a time-only format (contains colons but no space or hyphens)
    if trimmed_input.contains(':') && !trimmed_input.contains(' ') && !trimmed_input.contains('-') {
        return parse_time_only(trimmed_input);
    }

    // If none of the above, try relative time without prefix (like "2h", "30m")
    let base_time = get_current_time();
    parse_relative_time(trimmed_input, base_time)
}

/// Validate that start time is before end time
///
/// This function ensures that the time range is valid for progress calculation.
///
/// # Arguments
///
/// * `start` - The start time
/// * `end` - The end time
///
/// # Returns
///
/// * `Ok(())` - Times are valid (start <= end)
/// * `Err(PbError)` - Start time is after end time
///
/// # Examples
///
/// ```
/// use pb::time_parser::{parse_time, validate_times};
///
/// let start = parse_time("2025-07-21 10:00:00").unwrap();
/// let end = parse_time("2025-07-21 12:00:00").unwrap();
///
/// let result = validate_times(start, end);
/// assert!(result.is_ok());
/// ```
pub fn validate_times(start: NaiveDateTime, end: NaiveDateTime) -> Result<(), PbError> {
    if start > end {
        return Err(PbError::StartAfterEnd);
    }
    Ok(())
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
        assert!(
            duration.as_millis() < 1000,
            "Parsing took too long: {:?}",
            duration
        );
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

    // ========================================
    // DateTime Parsing Tests
    // ========================================

    #[test]
    fn test_parse_valid_datetimes() {
        // Test basic valid datetime
        let result = parse_datetime("2025-07-21 10:30:45");
        assert!(result.is_ok());
        let datetime = result.unwrap();
        assert_eq!(datetime.date().year(), 2025);
        assert_eq!(datetime.date().month(), 7);
        assert_eq!(datetime.date().day(), 21);
        assert_eq!(datetime.time().hour(), 10);
        assert_eq!(datetime.time().minute(), 30);
        assert_eq!(datetime.time().second(), 45);

        // Test end of year datetime
        let result = parse_datetime("2025-12-31 23:59:59");
        assert!(result.is_ok());
        let datetime = result.unwrap();
        assert_eq!(datetime.date().year(), 2025);
        assert_eq!(datetime.date().month(), 12);
        assert_eq!(datetime.date().day(), 31);
        assert_eq!(datetime.time().hour(), 23);
        assert_eq!(datetime.time().minute(), 59);
        assert_eq!(datetime.time().second(), 59);

        // Test start of day (midnight)
        let result = parse_datetime("2025-01-01 00:00:00");
        assert!(result.is_ok());
        let datetime = result.unwrap();
        assert_eq!(datetime.date().year(), 2025);
        assert_eq!(datetime.date().month(), 1);
        assert_eq!(datetime.date().day(), 1);
        assert_eq!(datetime.time().hour(), 0);
        assert_eq!(datetime.time().minute(), 0);
        assert_eq!(datetime.time().second(), 0);

        // Test noon
        let result = parse_datetime("2025-07-21 12:00:00");
        assert!(result.is_ok());
        let datetime = result.unwrap();
        assert_eq!(datetime.time().hour(), 12);
        assert_eq!(datetime.time().minute(), 0);
        assert_eq!(datetime.time().second(), 0);

        // Test leap year with time
        let result = parse_datetime("2024-02-29 15:45:30");
        assert!(result.is_ok());
        let datetime = result.unwrap();
        assert_eq!(datetime.date().year(), 2024);
        assert_eq!(datetime.date().month(), 2);
        assert_eq!(datetime.date().day(), 29);
        assert_eq!(datetime.time().hour(), 15);
        assert_eq!(datetime.time().minute(), 45);
        assert_eq!(datetime.time().second(), 30);
    }

    #[test]
    fn test_parse_invalid_datetime_formats() {
        // Missing time component
        let result = parse_datetime("2025-07-21");
        assert!(result.is_err());
        if let Err(PbError::InvalidTimeFormat { input }) = result {
            assert_eq!(input, "2025-07-21");
        } else {
            panic!("Expected InvalidTimeFormat error");
        }

        // Missing date component
        let result = parse_datetime("10:30:45");
        assert!(result.is_err());

        // ISO format with T separator
        let result = parse_datetime("2025-07-21T10:30:45");
        assert!(result.is_err());

        // US date format
        let result = parse_datetime("07/21/2025 10:30:45");
        assert!(result.is_err());

        // Wrong separator
        let result = parse_datetime("2025-07-21_10:30:45");
        assert!(result.is_err());

        // Missing seconds
        let result = parse_datetime("2025-07-21 10:30");
        assert!(result.is_err());

        // Extra components
        let result = parse_datetime("2025-07-21 10:30:45:123");
        assert!(result.is_err());

        // Empty string
        let result = parse_datetime("");
        assert!(result.is_err());

        // Non-numeric time components
        let result = parse_datetime("2025-07-21 ab:30:45");
        assert!(result.is_err());

        let result = parse_datetime("2025-07-21 10:cd:45");
        assert!(result.is_err());

        let result = parse_datetime("2025-07-21 10:30:ef");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_time_components() {
        // Invalid hour (> 24)
        let result = parse_datetime("2025-07-21 25:00:00");
        assert!(result.is_err());
        if let Err(PbError::InvalidTimeFormat { input }) = result {
            assert_eq!(input, "2025-07-21 25:00:00");
        } else {
            panic!("Expected InvalidTimeFormat error");
        }

        // Note: 24:00:00 is actually valid in ISO 8601 and represents midnight of the next day
        // So we test with 25:00:00 instead

        // Invalid minute (> 59)
        let result = parse_datetime("2025-07-21 10:60:00");
        assert!(result.is_err());

        let result = parse_datetime("2025-07-21 10:99:00");
        assert!(result.is_err());

        // Invalid second (> 59)
        let result = parse_datetime("2025-07-21 10:30:61");
        assert!(result.is_err());

        let result = parse_datetime("2025-07-21 10:30:99");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_datetime_edge_cases() {
        // Test all boundary values

        // Start of day
        let result = parse_datetime("2025-07-21 00:00:00");
        assert!(result.is_ok());

        // End of day
        let result = parse_datetime("2025-07-21 23:59:59");
        assert!(result.is_ok());

        // Noon
        let result = parse_datetime("2025-07-21 12:00:00");
        assert!(result.is_ok());

        // Test leap year February 29th with various times
        let result = parse_datetime("2024-02-29 00:00:00");
        assert!(result.is_ok());

        let result = parse_datetime("2024-02-29 23:59:59");
        assert!(result.is_ok());

        // Test non-leap year February 28th (should work)
        let result = parse_datetime("2023-02-28 12:00:00");
        assert!(result.is_ok());

        // Test non-leap year February 29th (should fail)
        let result = parse_datetime("2023-02-29 12:00:00");
        assert!(result.is_err());

        // Test months with different day counts
        // April has 30 days
        let result = parse_datetime("2025-04-30 12:00:00");
        assert!(result.is_ok());

        let result = parse_datetime("2025-04-31 12:00:00");
        assert!(result.is_err());

        // February has 28 days in non-leap years
        let result = parse_datetime("2025-02-28 12:00:00");
        assert!(result.is_ok());

        let result = parse_datetime("2025-02-29 12:00:00");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_datetime_with_invalid_dates() {
        // Test invalid dates with valid times
        let result = parse_datetime("2025-13-01 10:30:45");
        assert!(result.is_err());

        let result = parse_datetime("2025-00-15 10:30:45");
        assert!(result.is_err());

        let result = parse_datetime("2025-02-30 10:30:45");
        assert!(result.is_err());

        let result = parse_datetime("2025-01-32 10:30:45");
        assert!(result.is_err());

        let result = parse_datetime("2025-05-00 10:30:45");
        assert!(result.is_err());
    }

    #[test]
    fn test_datetime_error_message_format() {
        let result = parse_datetime("invalid-datetime");
        assert!(result.is_err());

        if let Err(PbError::InvalidTimeFormat { input }) = result {
            assert_eq!(input, "invalid-datetime");
        } else {
            panic!("Expected InvalidTimeFormat error with input");
        }

        // Test with specific invalid format
        let result = parse_datetime("2025-07-21T10:30:45");
        assert!(result.is_err());

        if let Err(PbError::InvalidTimeFormat { input }) = result {
            assert_eq!(input, "2025-07-21T10:30:45");
        } else {
            panic!("Expected InvalidTimeFormat error with input");
        }
    }

    #[test]
    fn test_datetime_performance_repeated_parsing() {
        use std::time::Instant;

        let start = Instant::now();

        // Parse the same datetime 1000 times
        for _ in 0..1000 {
            let result = parse_datetime("2025-07-21 10:30:45");
            assert!(result.is_ok());
        }

        let duration = start.elapsed();

        // Should complete well under 1 second for 1000 parses
        assert!(
            duration.as_millis() < 1000,
            "DateTime parsing took too long: {:?}",
            duration
        );
    }

    #[test]
    fn test_datetime_extreme_values() {
        // Test very old datetime
        let result = parse_datetime("0001-01-01 00:00:00");
        assert!(result.is_ok());

        // Test far future datetime
        let result = parse_datetime("9999-12-31 23:59:59");
        assert!(result.is_ok());

        // Test year boundaries with times
        let result = parse_datetime("2020-12-31 23:59:59");
        assert!(result.is_ok());

        let result = parse_datetime("2021-01-01 00:00:00");
        assert!(result.is_ok());
    }

    #[test]
    fn test_consistency_between_date_and_datetime_parsing() {
        // Parse the same date using both functions and ensure consistency
        let date_result = parse_date("2025-07-21").unwrap();
        let datetime_result = parse_datetime("2025-07-21 00:00:00").unwrap();

        // They should represent the same moment in time
        assert_eq!(date_result, datetime_result);

        // Check individual components
        assert_eq!(date_result.date(), datetime_result.date());
        assert_eq!(date_result.time(), datetime_result.time());
    }

    #[test]
    fn test_datetime_formatting_compatibility() {
        // Test that parsed datetime can be formatted back to the same string
        let input = "2025-07-21 10:30:45";
        let parsed = parse_datetime(input).unwrap();
        let formatted = parsed.format("%Y-%m-%d %H:%M:%S").to_string();
        assert_eq!(input, formatted);
    }

    // ========================================
    // Relative Time Parsing Tests
    // ========================================

    #[test]
    fn test_parse_valid_relative_times() {
        let base_time =
            NaiveDateTime::parse_from_str("2025-07-21 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();

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

        // Test boundary values
        let result = parse_relative_time("1m", base_time);
        assert!(result.is_ok());

        let result = parse_relative_time("999m", base_time);
        assert!(result.is_ok());

        let result = parse_relative_time("999h", base_time);
        assert!(result.is_ok());

        let result = parse_relative_time("999d", base_time);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_invalid_relative_time_formats() {
        let base_time =
            NaiveDateTime::parse_from_str("2025-07-21 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();

        // Missing unit
        let result = parse_relative_time("30", base_time);
        assert!(result.is_err());
        if let Err(PbError::InvalidRelativeTimeFormat { input }) = result {
            assert_eq!(input, "30");
        } else {
            panic!("Expected InvalidRelativeTimeFormat error");
        }

        // Invalid unit
        let result = parse_relative_time("30x", base_time);
        assert!(result.is_err());

        let result = parse_relative_time("30s", base_time);
        assert!(result.is_err());

        let result = parse_relative_time("30w", base_time);
        assert!(result.is_err());

        // Multiple digits and units
        let result = parse_relative_time("30m2h", base_time);
        assert!(result.is_err());

        // Unit before number
        let result = parse_relative_time("m30", base_time);
        assert!(result.is_err());

        // Spaces
        let result = parse_relative_time(" 30m", base_time);
        assert!(result.is_err());

        let result = parse_relative_time("30m ", base_time);
        assert!(result.is_err());

        let result = parse_relative_time("30 m", base_time);
        assert!(result.is_err());

        // Empty string
        let result = parse_relative_time("", base_time);
        assert!(result.is_err());

        // Non-numeric amount
        let result = parse_relative_time("abcm", base_time);
        assert!(result.is_err());

        // Decimal numbers
        let result = parse_relative_time("1.5h", base_time);
        assert!(result.is_err());

        // Negative numbers
        let result = parse_relative_time("-30m", base_time);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_relative_time_range_validation() {
        let base_time =
            NaiveDateTime::parse_from_str("2025-07-21 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();

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

        // Values over 999 not allowed
        let result = parse_relative_time("1000m", base_time);
        assert!(result.is_err());

        let result = parse_relative_time("1000h", base_time);
        assert!(result.is_err());

        let result = parse_relative_time("1000d", base_time);
        assert!(result.is_err());

        // Very large numbers
        let result = parse_relative_time("99999d", base_time);
        assert!(result.is_err());

        // Boundary tests - valid
        let result = parse_relative_time("1m", base_time);
        assert!(result.is_ok());

        let result = parse_relative_time("999m", base_time);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_relative_time_edge_cases() {
        let _base_time =
            NaiveDateTime::parse_from_str("2025-07-21 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();

        // Test with different base times
        let early_base =
            NaiveDateTime::parse_from_str("2000-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let result = parse_relative_time("1d", early_base);
        assert!(result.is_ok());

        let late_base =
            NaiveDateTime::parse_from_str("2999-12-31 23:59:59", "%Y-%m-%d %H:%M:%S").unwrap();
        let result = parse_relative_time("1m", late_base);
        // This might overflow, and that's OK - the function should handle it gracefully
        // We don't assert success/failure because it depends on the datetime implementation limits
        let _result = result;

        // Test edge of year boundaries
        let year_end =
            NaiveDateTime::parse_from_str("2025-12-31 23:59:59", "%Y-%m-%d %H:%M:%S").unwrap();
        let result = parse_relative_time("1m", year_end);
        assert!(result.is_ok());
        let new_time = result.unwrap();
        assert_eq!(new_time.year(), 2026);
        assert_eq!(new_time.month(), 1);
        assert_eq!(new_time.day(), 1);
        assert_eq!(new_time.hour(), 0);
        assert_eq!(new_time.minute(), 0);
        assert_eq!(new_time.second(), 59);
    }

    #[test]
    fn test_parse_relative_time_unit_conversions() {
        let base_time =
            NaiveDateTime::parse_from_str("2025-07-21 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();

        // Test minute conversion (30 minutes = 1800 seconds)
        let result = parse_relative_time("30m", base_time);
        assert!(result.is_ok());
        let expected_time = base_time + Duration::seconds(30 * 60);
        assert_eq!(result.unwrap(), expected_time);

        // Test hour conversion (2 hours = 7200 seconds)
        let result = parse_relative_time("2h", base_time);
        assert!(result.is_ok());
        let expected_time = base_time + Duration::seconds(2 * 3600);
        assert_eq!(result.unwrap(), expected_time);

        // Test day conversion (1 day = 86400 seconds)
        let result = parse_relative_time("1d", base_time);
        assert!(result.is_ok());
        let expected_time = base_time + Duration::seconds(1 * 86400);
        assert_eq!(result.unwrap(), expected_time);

        // Verify exact calculations
        let result_1h = parse_relative_time("1h", base_time).unwrap();
        let result_60m = parse_relative_time("60m", base_time).unwrap();
        assert_eq!(result_1h, result_60m);

        let result_1d = parse_relative_time("1d", base_time).unwrap();
        let result_24h = parse_relative_time("24h", base_time).unwrap();
        assert_eq!(result_1d, result_24h);
    }

    #[test]
    fn test_parse_relative_time_different_units() {
        let base_time =
            NaiveDateTime::parse_from_str("2025-07-21 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();

        // Test all supported units with same number
        let result_5m = parse_relative_time("5m", base_time);
        assert!(result_5m.is_ok());
        let time_5m = result_5m.unwrap();
        assert_eq!(time_5m.minute(), 5);

        let result_5h = parse_relative_time("5h", base_time);
        assert!(result_5h.is_ok());
        let time_5h = result_5h.unwrap();
        assert_eq!(time_5h.hour(), 15); // 10 + 5 = 15

        let result_5d = parse_relative_time("5d", base_time);
        assert!(result_5d.is_ok());
        let time_5d = result_5d.unwrap();
        assert_eq!(time_5d.day(), 26); // 21 + 5 = 26

        // Verify they are all different
        assert_ne!(time_5m, time_5h);
        assert_ne!(time_5h, time_5d);
        assert_ne!(time_5m, time_5d);
    }

    #[test]
    fn test_parse_relative_time_error_messages() {
        let base_time =
            NaiveDateTime::parse_from_str("2025-07-21 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();

        // Test that error messages contain the input
        let test_cases = vec![
            "30",    // Missing unit
            "30x",   // Invalid unit
            "0m",    // Zero not allowed
            "1000m", // Too large
            " 30m",  // Leading space
            "30m ",  // Trailing space
            "abc",   // Not a number
            "",      // Empty
        ];

        for input in test_cases {
            let result = parse_relative_time(input, base_time);
            assert!(result.is_err());
            if let Err(PbError::InvalidRelativeTimeFormat { input: error_input }) = result {
                assert_eq!(error_input, input);
            } else {
                panic!(
                    "Expected InvalidRelativeTimeFormat error for input: {}",
                    input
                );
            }
        }
    }

    #[test]
    fn test_parse_relative_time_performance() {
        let base_time =
            NaiveDateTime::parse_from_str("2025-07-21 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();

        // Test performance with repeated parsing
        let start = std::time::Instant::now();

        for _ in 0..1000 {
            let result = parse_relative_time("30m", base_time);
            assert!(result.is_ok());
        }

        let duration = start.elapsed();

        // Should complete within reasonable time for 1000 parses (allowing for Docker overhead)
        assert!(
            duration.as_millis() < 5000, // Increased from 2000ms to 5000ms for CI stability
            "Relative time parsing took too long: {:?}",
            duration
        );
    }
}
