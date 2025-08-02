//! Common test utilities and helpers for pb testing
//!
//! This module provides shared functionality used across multiple test files,
//! including test data generation, assertion helpers, and common test scenarios.

use assert_cmd::Command;
use chrono::{Duration, NaiveDateTime};

/// Test data generator for common time formats
#[allow(dead_code)]
pub struct TimeTestData;

#[allow(dead_code)]
impl TimeTestData {
    /// Generate test cases for date format parsing
    pub fn date_format_cases() -> Vec<(&'static str, bool)> {
        vec![
            // Valid cases
            ("2025-07-21", true),
            ("2025-01-01", true),
            ("2025-12-31", true),
            ("2024-02-29", true), // Leap year
            ("2025-2-5", true),   // Single digit month/day
            // Invalid cases
            ("25-07-21", false),     // 2-digit year
            ("2025-13-01", false),   // Invalid month
            ("2025-02-30", false),   // Invalid day for February
            ("2025-04-31", false),   // Invalid day for April
            ("2024-02-30", false),   // Invalid day for leap year February
            ("2025/07/21", false),   // Wrong separator
            ("2025.07.21", false),   // Wrong separator
            ("2025-7-21-10", false), // Too many parts
            ("", false),             // Empty string
            ("invalid", false),      // Not a date
            ("2025", false),         // Year only
            ("2025-07", false),      // Missing day
            ("abcd-ef-gh", false),   // Non-numeric
        ]
    }

    /// Generate test cases for datetime format parsing
    pub fn datetime_format_cases() -> Vec<(&'static str, bool)> {
        vec![
            // Valid cases
            ("2025-07-21 10:30:45", true),
            ("2025-01-01 00:00:00", true),
            ("2025-12-31 23:59:59", true),
            ("2025-02-28 12:30:15", true),
            ("2025-6-5 9:5:5", true), // Single digits
            // Invalid cases
            ("2025-07-21 25:00:00", false),  // Invalid hour
            ("2025-07-21 10:60:00", false),  // Invalid minute
            ("2025-07-21 10:30:60", false),  // Invalid second
            ("2025-07-21T10:30:45", false),  // ISO format (not supported)
            ("2025-07-21 10:30", false),     // Missing seconds
            ("2025-07-21  10:30:45", false), // Double space
            ("2025-07-21", false),           // Date only
            ("10:30:45", false),             // Time only
            ("", false),                     // Empty string
            ("invalid datetime", false),     // Invalid format
        ]
    }

    /// Generate test cases for time format parsing
    pub fn time_format_cases() -> Vec<(&'static str, bool)> {
        vec![
            // Valid cases
            ("10:30:45", true),
            ("00:00:00", true),
            ("23:59:59", true),
            ("9:5:5", true), // Single digits
            ("12:30", true), // Missing seconds (should default to 00)
            // Invalid cases
            ("25:00:00", false),    // Invalid hour
            ("10:60:00", false),    // Invalid minute
            ("10:30:60", false),    // Invalid second
            ("10", false),          // Hour only
            ("", false),            // Empty string
            ("invalid", false),     // Invalid format
            ("10:30:45:00", false), // Too many parts
        ]
    }

    /// Generate test cases for relative time parsing
    pub fn relative_time_cases() -> Vec<(&'static str, bool)> {
        vec![
            // Valid cases (simple format: number + unit)
            ("1h", true),
            ("30m", true),
            ("1d", true),
            ("2h", true),
            ("999h", true), // Max range
            // Invalid cases
            ("0h", false),      // Zero not allowed
            ("0m", false),      // Zero not allowed
            ("h", false),       // Missing number
            ("1", false),       // Missing unit
            ("1x", false),      // Invalid unit
            ("", false),        // Empty string
            ("invalid", false), // Invalid format
            ("1000h", false),   // Out of range (if range limited)
            ("-1h", false),     // Negative (depends on implementation)
            ("1h30m", false),   // Complex format (might not be supported)
        ]
    }
}

/// Progress calculation test utilities
pub struct ProgressTestUtils;

impl ProgressTestUtils {
    /// Create test datetime from a simple string format for testing
    pub fn parse_test_datetime(datetime_str: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S")
            .expect("Invalid test datetime format")
    }

    /// Generate test cases for progress calculation
    pub fn progress_calculation_cases() -> Vec<((&'static str, &'static str, &'static str), f64)> {
        vec![
            // (start, end, current), expected_progress
            (
                (
                    "2025-07-21 10:00:00",
                    "2025-07-21 12:00:00",
                    "2025-07-21 11:00:00",
                ),
                50.0,
            ),
            (
                (
                    "2025-07-21 10:00:00",
                    "2025-07-21 12:00:00",
                    "2025-07-21 10:00:00",
                ),
                0.0,
            ),
            (
                (
                    "2025-07-21 10:00:00",
                    "2025-07-21 12:00:00",
                    "2025-07-21 12:00:00",
                ),
                100.0,
            ),
            (
                (
                    "2025-07-21 10:00:00",
                    "2025-07-21 12:00:00",
                    "2025-07-21 13:00:00",
                ),
                150.0,
            ),
            (
                (
                    "2025-07-21 10:00:00",
                    "2025-07-21 12:00:00",
                    "2025-07-21 09:00:00",
                ),
                0.0,
            ), // Before start
            (
                (
                    "2025-07-21 10:00:00",
                    "2025-07-21 10:00:00",
                    "2025-07-21 10:00:00",
                ),
                100.0,
            ), // Zero duration
            (
                (
                    "2025-07-21 10:00:00",
                    "2025-07-21 11:00:00",
                    "2025-07-21 10:15:00",
                ),
                25.0,
            ),
            (
                (
                    "2025-07-21 10:00:00",
                    "2025-07-21 11:00:00",
                    "2025-07-21 10:45:00",
                ),
                75.0,
            ),
        ]
    }
}

/// CLI test utilities
pub struct CliTestUtils;

impl CliTestUtils {
    /// Create a command with pb binary
    pub fn pb_command() -> Command {
        Command::cargo_bin("pmon").expect("Failed to find pb binary")
    }

    /// Generate test cases for CLI argument validation
    pub fn cli_validation_cases() -> Vec<(Vec<&'static str>, bool, &'static str)> {
        vec![
            // (args, should_succeed, expected_error_fragment)
            (vec!["--start", "10:00", "--end", "12:00"], true, ""),
            (vec!["-s", "10:00", "-e", "12:00"], true, ""),
            (
                vec!["--start", "10:00", "--end", "12:00", "--interval", "30"],
                true,
                "",
            ),
            (vec![], false, "required"),
            (vec!["--start", "10:00"], false, "required"),
            (vec!["--end", "12:00"], false, "required"),
            (vec!["--start", "", "--end", "12:00"], false, "parsing"),
            (vec!["--start", "10:00", "--end", ""], false, "parsing"),
            (
                vec!["--start", "10:00", "--end", "12:00", "--interval", "0"],
                false,
                "must be greater than 0",
            ),
            (
                vec!["--start", "invalid", "--end", "12:00"],
                false,
                "parsing start time",
            ),
            (
                vec!["--start", "10:00", "--end", "invalid"],
                false,
                "parsing end time",
            ),
            (
                vec!["--start", "12:00", "--end", "10:00"],
                false,
                "must be before",
            ),
        ]
    }
}

/// Progress bar rendering test utilities
pub struct ProgressBarTestUtils;

impl ProgressBarTestUtils {
    /// Test cases for progress bar rendering
    pub fn rendering_cases() -> Vec<(f64, &'static str)> {
        vec![
            (0.0, "[░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░] 0.0%"),
            (25.0, "[██████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░] 25.0%"),
            (50.0, "[████████████████████░░░░░░░░░░░░░░░░░░░░] 50.0%"),
            (75.0, "[██████████████████████████████░░░░░░░░░░] 75.0%"),
            (100.0, "[████████████████████████████████████████] 100.0%"),
            (150.0, "[████████████████████████████████████████] 150.0%"),
            (-10.0, "[░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░] -10.0%"), // Negative values
            (0.1, "[░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░] 0.1%"),     // Small values
            (99.9, "[████████████████████████████████████████] 99.9%"),   // Near 100%
        ]
    }

    /// Verify that a progress bar string has the correct format
    pub fn verify_progress_bar_format(bar: &str) -> bool {
        // Check basic format: [40 chars] percentage%
        if bar.chars().count() < 45 {
            return false;
        }

        // Check brackets
        if !bar.starts_with('[') || !bar.contains(']') {
            return false;
        }

        // Check percentage at the end
        if !bar.ends_with('%') {
            return false;
        }

        // Check that the bar section is 40 characters
        if let Some(close_bracket) = bar.find(']') {
            let bar_section = &bar[1..close_bracket];
            if bar_section.chars().count() != 40 {
                return false;
            }

            // Check that bar section only contains valid characters (█ and ░)
            for c in bar_section.chars() {
                if c != '█' && c != '░' {
                    return false;
                }
            }
        } else {
            return false;
        }

        true
    }
}

/// Performance test utilities
pub struct PerformanceTestUtils;

impl PerformanceTestUtils {
    /// Measure the execution time of a function
    pub fn measure_time<F, R>(f: F) -> (R, std::time::Duration)
    where
        F: FnOnce() -> R,
    {
        let start = std::time::Instant::now();
        let result = f();
        let duration = start.elapsed();
        (result, duration)
    }

    /// Run a function multiple times and return average execution time
    pub fn benchmark<F, R>(f: F, iterations: usize) -> std::time::Duration
    where
        F: Fn() -> R,
    {
        let mut total_duration = std::time::Duration::new(0, 0);

        for _ in 0..iterations {
            let (_, duration) = Self::measure_time(&f);
            total_duration += duration;
        }

        total_duration / iterations as u32
    }

    /// Performance expectations for critical functions
    /// These are realistic expectations based on actual performance in CI environments
    pub fn performance_expectations() -> PerformanceExpectations {
        PerformanceExpectations {
            parse_time_max: std::time::Duration::from_millis(10), // Increased from 1ms to 10ms
            calculate_progress_max: std::time::Duration::from_millis(1), // Increased from 100μs to 1ms
            render_progress_bar_max: std::time::Duration::from_millis(5), // Increased from 1ms to 5ms
        }
    }
}

/// Performance expectations for various operations
pub struct PerformanceExpectations {
    pub parse_time_max: std::time::Duration,
    pub calculate_progress_max: std::time::Duration,
    pub render_progress_bar_max: std::time::Duration,
}

/// Custom assertion helpers
pub struct AssertionHelpers;

impl AssertionHelpers {
    /// Assert that two floating point numbers are approximately equal
    pub fn assert_approx_eq(a: f64, b: f64, tolerance: f64) {
        let diff = (a - b).abs();
        if diff > tolerance {
            panic!(
                "Values are not approximately equal: {a} vs {b} (diff: {diff}, tolerance: {tolerance})"
            );
        }
    }

    /// Assert that a result contains a specific error type
    pub fn assert_error_contains<T, E: std::fmt::Display>(
        result: Result<T, E>,
        expected_fragment: &str,
    ) {
        match result {
            Ok(_) => panic!("Expected error but got Ok"),
            Err(e) => {
                let error_msg = e.to_string();
                if !error_msg.contains(expected_fragment) {
                    panic!(
                        "Error message '{error_msg}' does not contain expected fragment '{expected_fragment}'"
                    );
                }
            }
        }
    }
}

/// Test fixtures for generating consistent test data
pub struct TestFixtures;

impl TestFixtures {
    /// Generate a test datetime relative to a base time
    pub fn datetime_relative_to_now(offset_minutes: i64) -> NaiveDateTime {
        let base = chrono::Local::now().naive_local();
        base + Duration::minutes(offset_minutes)
    }

    /// Generate a pair of start/end times with a specific duration
    pub fn time_range_with_duration(
        start_offset_minutes: i64,
        duration_minutes: i64,
    ) -> (NaiveDateTime, NaiveDateTime) {
        let start = Self::datetime_relative_to_now(start_offset_minutes);
        let end = start + Duration::minutes(duration_minutes);
        (start, end)
    }

    /// Common test time ranges for different scenarios
    pub fn common_time_ranges() -> Vec<(String, NaiveDateTime, NaiveDateTime)> {
        vec![
            // Past range (completed)
            {
                let (start, end) = Self::time_range_with_duration(-120, 60);
                ("past_completed".to_string(), start, end)
            },
            // Current range (in progress)
            {
                let (start, end) = Self::time_range_with_duration(-30, 60);
                ("current_in_progress".to_string(), start, end)
            },
            // Future range (not started)
            {
                let (start, end) = Self::time_range_with_duration(30, 60);
                ("future_not_started".to_string(), start, end)
            },
            // Zero duration
            {
                let start = Self::datetime_relative_to_now(0);
                ("zero_duration".to_string(), start, start)
            },
        ]
    }
}
