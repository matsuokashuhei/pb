//! Time parsing functionality for the pb CLI tool
//!
//! This module provides functions to parse various time formats into
//! `NaiveDateTime` objects for use in progress bar calculations.

use chrono::{NaiveDate, NaiveDateTime};
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
    chrono::NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S")
        .map_err(|_| PbError::InvalidTimeFormat { 
            input: input.to_string() 
        })
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
        assert!(duration.as_millis() < 1000, "DateTime parsing took too long: {:?}", duration);
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
}
