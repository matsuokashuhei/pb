//! Tests for optional start time functionality
//!
//! This module tests the new feature that allows calling pb without
//! explicitly providing the --start parameter.

use chrono::Timelike;
use clap::Parser;
use pmon::{determine_start_time_for_end, get_current_time, Cli};

#[cfg(test)]
mod optional_start_time_tests {
    use super::*;

    #[test]
    fn test_determine_start_time_for_datetime_end() {
        // For datetime end times, should use current time
        let start = determine_start_time_for_end("2025-07-27 17:00:00");
        let current = get_current_time();

        // Should be close to current time (within 1 second)
        let diff = (start.and_utc().timestamp() - current.and_utc().timestamp()).abs();
        assert!(diff <= 1, "Start time should be close to current time");
    }

    #[test]
    fn test_determine_start_time_for_time_only_end() {
        // For time-only end times, should use current time
        let start = determine_start_time_for_end("17:00:00");
        let current = get_current_time();

        // Should be close to current time (within 1 second)
        let diff = (start.and_utc().timestamp() - current.and_utc().timestamp()).abs();
        assert!(diff <= 1, "Start time should be close to current time");
    }

    #[test]
    fn test_determine_start_time_for_relative_end() {
        // For relative end times, should use current time
        let start = determine_start_time_for_end("2h");
        let current = get_current_time();

        // Should be close to current time (within 1 second)
        let diff = (start.and_utc().timestamp() - current.and_utc().timestamp()).abs();
        assert!(diff <= 1, "Start time should be close to current time");
    }

    #[test]
    fn test_determine_start_time_for_date_only_end() {
        // For date-only end times, should use today at 00:00:00
        let start = determine_start_time_for_end("2025-12-31");
        let today = get_current_time().date();

        assert_eq!(start.date(), today);
        assert_eq!(start.hour(), 0);
        assert_eq!(start.minute(), 0);
        assert_eq!(start.second(), 0);
    }

    #[test]
    fn test_determine_start_time_edge_cases() {
        // Test various edge cases to ensure proper format detection

        // Datetime variants - should use current time
        let test_cases = vec![
            "2025-01-01 00:00:00",
            "2025-12-31 23:59:59",
            "2025-02-29 12:00:00", // leap year
        ];

        for case in test_cases {
            let start = determine_start_time_for_end(case);
            let current = get_current_time();
            let diff = (start.and_utc().timestamp() - current.and_utc().timestamp()).abs();
            assert!(diff <= 1, "Failed for case: {case}");
        }

        // Time-only variants - should use current time
        let time_cases = vec!["00:00:00", "12:30:45", "23:59:59"];

        for case in time_cases {
            let start = determine_start_time_for_end(case);
            let current = get_current_time();
            let diff = (start.and_utc().timestamp() - current.and_utc().timestamp()).abs();
            assert!(diff <= 1, "Failed for time case: {case}");
        }

        // Relative variants - should use current time
        let relative_cases = vec!["1s", "30m", "2h", "1d", "+1s", "+30m", "+2h", "+1d"];

        for case in relative_cases {
            let start = determine_start_time_for_end(case);
            let current = get_current_time();
            let diff = (start.and_utc().timestamp() - current.and_utc().timestamp()).abs();
            assert!(diff <= 1, "Failed for relative case: {case}");
        }

        // Date-only variants - should use today at 00:00:00
        let date_cases = vec![
            "2025-01-01",
            "2025-12-31",
            "2024-02-29", // leap year
        ];

        for case in date_cases {
            let start = determine_start_time_for_end(case);
            let today = get_current_time().date();

            assert_eq!(start.date(), today, "Failed date check for case: {case}");
            assert_eq!(start.hour(), 0, "Failed hour check for case: {case}");
            assert_eq!(start.minute(), 0, "Failed minute check for case: {case}");
            assert_eq!(start.second(), 0, "Failed second check for case: {case}");
        }
    }

    #[test]
    fn test_cli_optional_start_parsing() {
        // Test that CLI correctly parses when start is omitted
        let cli = Cli::try_parse_from(vec!["pmon", "--end", "17:00:00"]).unwrap();

        assert_eq!(cli.start(), None);
        assert_eq!(cli.end(), "17:00:00");
        assert_eq!(cli.interval(), 60); // default
    }

    #[test]
    fn test_cli_optional_start_with_interval() {
        // Test that CLI correctly parses when start is omitted but interval is provided
        let cli =
            Cli::try_parse_from(vec!["pmon", "--end", "17:00:00", "--interval", "30"]).unwrap();

        assert_eq!(cli.start(), None);
        assert_eq!(cli.end(), "17:00:00");
        assert_eq!(cli.interval(), 30);
    }

    #[test]
    fn test_cli_backward_compatibility() {
        // Test that providing start still works as before
        let cli =
            Cli::try_parse_from(vec!["pmon", "--start", "15:00:00", "--end", "17:00:00"]).unwrap();

        assert_eq!(cli.start(), Some("15:00:00"));
        assert_eq!(cli.end(), "17:00:00");
        assert_eq!(cli.interval(), 60);
    }

    #[test]
    fn test_cli_validation_with_optional_start() {
        // Test validation when start is None
        let cli = Cli::try_parse_from(vec!["pmon", "--end", "17:00:00"]).unwrap();
        assert!(cli.validate().is_ok());

        // Test validation when start is Some but empty
        let cli = Cli::try_parse_from(vec!["pmon", "--start", "", "--end", "17:00:00"]).unwrap();
        assert!(cli.validate().is_err());

        // Test validation when end is empty
        let cli = Cli::try_parse_from(vec!["pmon", "--end", ""]).unwrap();
        assert!(cli.validate().is_err());
    }

    #[test]
    fn test_cli_missing_end_still_fails() {
        // End time is still required
        let result = Cli::try_parse_from(vec!["pmon"]);
        assert!(result.is_err());

        let result = Cli::try_parse_from(vec!["pmon", "--interval", "30"]);
        assert!(result.is_err());
    }
}
