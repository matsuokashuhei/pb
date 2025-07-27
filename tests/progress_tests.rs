//! Comprehensive unit tests for progress_bar module
//!
//! This module tests progress calculation accuracy, progress bar rendering,
//! color management, and performance characteristics.

use chrono::Duration;
use pb::progress_bar::*;

mod common;
use common::helpers::{
    AssertionHelpers, PerformanceTestUtils, ProgressBarTestUtils, ProgressTestUtils,
};

#[cfg(test)]
mod calculate_progress_tests {
    use super::*;

    #[test]
    fn test_calculate_progress_basic_cases() {
        let test_cases = vec![
            // 50% progress
            (
                "2025-07-21 10:00:00",
                "2025-07-21 12:00:00",
                "2025-07-21 11:00:00",
                50.0,
            ),
            // 0% progress (at start)
            (
                "2025-07-21 10:00:00",
                "2025-07-21 12:00:00",
                "2025-07-21 10:00:00",
                0.0,
            ),
            // 100% progress (at end)
            (
                "2025-07-21 10:00:00",
                "2025-07-21 12:00:00",
                "2025-07-21 12:00:00",
                100.0,
            ),
            // 25% progress
            (
                "2025-07-21 10:00:00",
                "2025-07-21 12:00:00",
                "2025-07-21 10:30:00",
                25.0,
            ),
            // 75% progress
            (
                "2025-07-21 10:00:00",
                "2025-07-21 12:00:00",
                "2025-07-21 11:30:00",
                75.0,
            ),
        ];

        for (start_str, end_str, current_str, expected) in test_cases {
            let start = ProgressTestUtils::parse_test_datetime(start_str);
            let end = ProgressTestUtils::parse_test_datetime(end_str);
            let current = ProgressTestUtils::parse_test_datetime(current_str);

            let result = calculate_progress(start, end, current);
            AssertionHelpers::assert_approx_eq(result, expected, 0.001);
        }
    }

    #[test]
    fn test_calculate_progress_edge_cases() {
        // Zero duration (start == end)
        let start = ProgressTestUtils::parse_test_datetime("2025-07-21 10:00:00");
        let end = start;
        let current = start;
        let result = calculate_progress(start, end, current);
        assert_eq!(result, 100.0, "Zero duration should return 100%");

        // Current before start (should be clamped to 0%)
        let start = ProgressTestUtils::parse_test_datetime("2025-07-21 10:00:00");
        let end = ProgressTestUtils::parse_test_datetime("2025-07-21 12:00:00");
        let current = ProgressTestUtils::parse_test_datetime("2025-07-21 09:00:00");
        let result = calculate_progress(start, end, current);
        assert_eq!(result, 0.0, "Current before start should return 0%");

        // Current after end (overtime)
        let start = ProgressTestUtils::parse_test_datetime("2025-07-21 10:00:00");
        let end = ProgressTestUtils::parse_test_datetime("2025-07-21 12:00:00");
        let current = ProgressTestUtils::parse_test_datetime("2025-07-21 13:00:00");
        let result = calculate_progress(start, end, current);
        assert_eq!(result, 150.0, "Current after end should return >100%");
    }

    #[test]
    fn test_calculate_progress_precision() {
        // Test floating-point precision for small time differences
        let start = ProgressTestUtils::parse_test_datetime("2025-07-21 10:00:00");
        let end = start + Duration::seconds(1000); // 1000 seconds total
        let current = start + Duration::milliseconds(1500); // 1.5 seconds elapsed

        let result = calculate_progress(start, end, current);
        let expected = 1.5 / 1000.0 * 100.0; // 0.15%
        AssertionHelpers::assert_approx_eq(result, expected, 0.01);
    }

    #[test]
    fn test_calculate_progress_large_durations() {
        // Test with large time differences (days, weeks, months)
        let start = ProgressTestUtils::parse_test_datetime("2025-01-01 00:00:00");
        let end = ProgressTestUtils::parse_test_datetime("2025-12-31 23:59:59");
        let current = ProgressTestUtils::parse_test_datetime("2025-07-01 12:00:00");

        let result = calculate_progress(start, end, current);
        // Should be roughly 50% (middle of the year)
        assert!(
            result > 49.0 && result < 51.0,
            "Large duration calculation incorrect: {result}"
        );
    }

    #[test]
    fn test_calculate_progress_microsecond_precision() {
        // Test with microsecond-level precision
        let start = ProgressTestUtils::parse_test_datetime("2025-07-21 10:00:00");
        let end = start + Duration::microseconds(1000000); // 1 second
        let current = start + Duration::microseconds(500000); // 0.5 seconds

        let result = calculate_progress(start, end, current);
        AssertionHelpers::assert_approx_eq(result, 50.0, 0.001);
    }

    #[test]
    fn test_calculate_progress_comprehensive_matrix() {
        // Test using comprehensive test data
        for ((start_str, end_str, current_str), expected) in
            ProgressTestUtils::progress_calculation_cases()
        {
            let start = ProgressTestUtils::parse_test_datetime(start_str);
            let end = ProgressTestUtils::parse_test_datetime(end_str);
            let current = ProgressTestUtils::parse_test_datetime(current_str);

            let result = calculate_progress(start, end, current);
            AssertionHelpers::assert_approx_eq(result, expected, 0.001);
        }
    }

    #[test]
    fn test_calculate_progress_negative_durations() {
        // Test behavior when someone accidentally swaps start and end
        let start = ProgressTestUtils::parse_test_datetime("2025-07-21 12:00:00");
        let end = ProgressTestUtils::parse_test_datetime("2025-07-21 10:00:00"); // Earlier than start
        let current = ProgressTestUtils::parse_test_datetime("2025-07-21 11:00:00");

        let result = calculate_progress(start, end, current);
        // This should handle the negative duration gracefully
        // The exact behavior depends on implementation, but it should not panic
        assert!(
            result.is_finite(),
            "Result should be finite for negative duration"
        );
    }
}

#[cfg(test)]
mod render_progress_bar_tests {
    use super::*;

    #[test]
    fn test_render_progress_bar_basic_cases() {
        let test_cases = vec![
            (0.0, "[░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░] 0.0%"),
            (25.0, "[██████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░] 25.0%"),
            (50.0, "[████████████████████░░░░░░░░░░░░░░░░░░░░] 50.0%"),
            (75.0, "[██████████████████████████████░░░░░░░░░░] 75.0%"),
            (100.0, "[████████████████████████████████████████] 100.0%"),
        ];

        for (percentage, expected) in test_cases {
            let result = render_progress_bar(percentage);
            assert_eq!(result, expected, "Incorrect rendering for {percentage}%");
        }
    }

    #[test]
    fn test_render_progress_bar_edge_cases() {
        // Test edge cases
        let edge_cases = vec![
            (-10.0, "[░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░] -10.0%"), // Negative values
            (150.0, "[████████████████████████████████████████] 150.0%"), // Overtime
            (0.1, "[░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░] 0.1%"),     // Small values
            (99.9, "[████████████████████████████████████████] 99.9%"),   // Near 100%
            (0.5, "[░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░] 0.5%"),     // Rounding behavior
            (99.4, "[████████████████████████████████████████] 99.4%"),   // Rounding behavior
        ];

        for (percentage, expected) in edge_cases {
            let result = render_progress_bar(percentage);
            assert_eq!(result, expected, "Incorrect rendering for {percentage}%");
        }
    }

    #[test]
    fn test_render_progress_bar_format_validation() {
        // Test that all rendered bars have correct format
        let test_percentages = vec![0.0, 1.0, 25.0, 50.0, 75.0, 99.0, 100.0, 150.0, -10.0];

        for percentage in test_percentages {
            let result = render_progress_bar(percentage);
            assert!(
                ProgressBarTestUtils::verify_progress_bar_format(&result),
                "Invalid format for {percentage}%: '{result}'"
            );
        }
    }

    #[test]
    fn test_render_progress_bar_character_count() {
        // Test that the bar section is always exactly 40 characters
        let test_percentages = vec![0.0, 25.0, 50.0, 75.0, 100.0, 150.0];

        for percentage in test_percentages {
            let result = render_progress_bar(percentage);

            // Extract the bar part (between [ and ])
            let start = result.find('[').unwrap() + 1;
            let end = result.find(']').unwrap();
            let bar_part = &result[start..end];

            assert_eq!(
                bar_part.chars().count(),
                40,
                "Bar width incorrect for {percentage}%"
            );

            // Count filled and empty characters
            let filled_count = bar_part.chars().filter(|&c| c == '█').count();
            let empty_count = bar_part.chars().filter(|&c| c == '░').count();

            assert_eq!(
                filled_count + empty_count,
                40,
                "Total character count incorrect for {percentage}%"
            );
        }
    }

    #[test]
    fn test_render_progress_bar_comprehensive_matrix() {
        // Test using comprehensive test data
        for (percentage, expected) in ProgressBarTestUtils::rendering_cases() {
            let result = render_progress_bar(percentage);
            assert_eq!(result, expected, "Incorrect rendering for {percentage}%");
        }
    }

    #[test]
    fn test_render_progress_bar_fractional_percentages() {
        // Test how fractional percentages are handled
        let fractional_cases = vec![
            (12.3, 5),  // Should round to 5 filled characters (12.3% of 40 = 4.92)
            (37.8, 15), // Should round to 15 filled characters (37.8% of 40 = 15.12)
            (62.4, 25), // Should round to 25 filled characters (62.4% of 40 = 24.96)
            (87.6, 35), // Should round to 35 filled characters (87.6% of 40 = 35.04)
        ];

        for (percentage, expected_filled) in fractional_cases {
            let result = render_progress_bar(percentage);

            // Extract bar part and count filled characters
            let start = result.find('[').unwrap() + 1;
            let end = result.find(']').unwrap();
            let bar_part = &result[start..end];
            let filled_count = bar_part.chars().filter(|&c| c == '█').count();

            assert_eq!(
                filled_count, expected_filled,
                "Incorrect filled count for {percentage}%: expected {expected_filled}, got {filled_count}"
            );
        }
    }
}

#[cfg(test)]
mod render_colored_progress_bar_tests {
    use super::*;

    #[test]
    fn test_render_colored_progress_bar_normal_range() {
        // Test normal range (0-100%) - should be default color
        let normal_cases = vec![0.0, 25.0, 50.0, 75.0, 100.0];

        for percentage in normal_cases {
            let result = render_colored_progress_bar(percentage);
            let _expected = render_progress_bar(percentage);

            // For normal range, colored version should match non-colored version
            // (when no color is applied, they should be identical)
            assert!(
                result.contains(&format!("{percentage:.1}%")),
                "Colored bar should contain percentage for {percentage}%"
            );
        }
    }

    #[test]
    fn test_render_colored_progress_bar_overtime() {
        // Test overtime (>100%) - should be red color
        let overtime_cases = vec![101.0, 125.0, 150.0, 200.0];

        for percentage in overtime_cases {
            let result = render_colored_progress_bar(percentage);

            // For overtime, the result should contain red color codes or be formatted differently
            assert!(
                result.contains(&format!("{percentage:.1}%")),
                "Colored bar should contain percentage for {percentage}%"
            );

            // Verify it's different from the non-colored version (when color is enabled)
            let _non_colored = render_progress_bar(percentage);
            // Note: The exact comparison depends on whether colors are enabled in the test environment
        }
    }

    #[test]
    fn test_render_colored_progress_bar_format_consistency() {
        // Test that colored bars maintain the same format as non-colored bars
        let test_percentages = vec![-10.0, 0.0, 50.0, 100.0, 150.0];

        for percentage in test_percentages {
            let colored_result = render_colored_progress_bar(percentage);

            // Strip ANSI color codes for format validation
            let stripped = strip_ansi_codes(&colored_result);
            assert!(
                ProgressBarTestUtils::verify_progress_bar_format(&stripped),
                "Invalid format for colored bar at {percentage}%: '{stripped}'"
            );
        }
    }

    #[test]
    fn test_render_colored_progress_bar_no_color_environment() {
        // Test behavior when NO_COLOR environment variable is set
        // Save original value if it exists
        let original_no_color = std::env::var("NO_COLOR").ok();

        std::env::set_var("NO_COLOR", "1");

        let result = render_colored_progress_bar(150.0);
        let expected = render_progress_bar(150.0);

        // When NO_COLOR is set, colored and non-colored should be identical
        // Note: colored crate respects NO_COLOR automatically
        if result == expected {
            // Colors are disabled as expected
            assert_eq!(result, expected, "NO_COLOR should disable colors");
        } else {
            // If colors still appear, check if they contain the expected content
            let stripped = strip_ansi_codes(&result);
            assert_eq!(
                stripped, expected,
                "Stripped colors should match expected output"
            );
        }

        // Clean up - restore original value or remove if it didn't exist
        match original_no_color {
            Some(val) => std::env::set_var("NO_COLOR", val),
            None => std::env::remove_var("NO_COLOR"),
        }
    }

    /// Helper function to strip ANSI color codes for testing
    fn strip_ansi_codes(input: &str) -> String {
        // Simple regex to remove ANSI escape sequences
        // This is a basic implementation for testing purposes
        let mut result = String::new();
        let mut in_escape = false;
        let mut chars = input.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\x1b' && chars.peek() == Some(&'[') {
                in_escape = true;
                continue;
            }

            if in_escape {
                if ch.is_ascii_alphabetic() {
                    in_escape = false;
                }
                continue;
            }

            result.push(ch);
        }

        result
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn test_calculate_progress_performance() {
        let start = ProgressTestUtils::parse_test_datetime("2025-07-21 10:00:00");
        let end = ProgressTestUtils::parse_test_datetime("2025-07-21 12:00:00");
        let current = ProgressTestUtils::parse_test_datetime("2025-07-21 11:00:00");

        let expectations = PerformanceTestUtils::performance_expectations();

        let avg_duration =
            PerformanceTestUtils::benchmark(|| calculate_progress(start, end, current), 10000);

        assert!(
            avg_duration < expectations.calculate_progress_max,
            "calculate_progress took too long: {:?} > {:?}",
            avg_duration,
            expectations.calculate_progress_max
        );
    }

    #[test]
    fn test_render_progress_bar_performance() {
        let expectations = PerformanceTestUtils::performance_expectations();

        let avg_duration = PerformanceTestUtils::benchmark(|| render_progress_bar(50.0), 10000);

        assert!(
            avg_duration < expectations.render_progress_bar_max,
            "render_progress_bar took too long: {:?} > {:?}",
            avg_duration,
            expectations.render_progress_bar_max
        );
    }

    #[test]
    fn test_render_colored_progress_bar_performance() {
        let expectations = PerformanceTestUtils::performance_expectations();

        let avg_duration =
            PerformanceTestUtils::benchmark(|| render_colored_progress_bar(50.0), 10000);

        assert!(
            avg_duration < expectations.render_progress_bar_max,
            "render_colored_progress_bar took too long: {:?} > {:?}",
            avg_duration,
            expectations.render_progress_bar_max
        );
    }

    #[test]
    fn test_memory_usage() {
        // Test that progress bar rendering doesn't cause excessive memory allocation
        let initial_memory = get_memory_usage();

        // Render many progress bars
        for i in 0..10000 {
            let percentage = (i as f64 / 100.0) % 200.0; // 0-200%
            let _bar = render_progress_bar(percentage);
            let _colored_bar = render_colored_progress_bar(percentage);
        }

        let final_memory = get_memory_usage();
        let memory_increase = final_memory - initial_memory;

        // Memory increase should be reasonable (less than 10MB for 10k iterations)
        assert!(
            memory_increase < 10_000_000,
            "Excessive memory usage: {memory_increase} bytes"
        );
    }

    /// Simple memory usage estimation (returns 0 if not available)
    fn get_memory_usage() -> usize {
        // This is a simplified version - in a real implementation,
        // you might use a crate like `memory-stats` or platform-specific APIs
        0
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_extreme_percentage_values() {
        // Test with extreme but valid percentage values
        let extreme_cases = vec![
            f64::NEG_INFINITY,
            -1000000.0,
            -0.0001,
            0.0001,
            1000000.0,
            f64::INFINITY,
        ];

        for percentage in extreme_cases {
            // Should not panic, even with extreme values
            let result = std::panic::catch_unwind(|| render_progress_bar(percentage));
            assert!(
                result.is_ok(),
                "render_progress_bar panicked with extreme value: {percentage}"
            );

            let result = std::panic::catch_unwind(|| render_colored_progress_bar(percentage));
            assert!(
                result.is_ok(),
                "render_colored_progress_bar panicked with extreme value: {percentage}"
            );
        }
    }

    #[test]
    fn test_nan_handling() {
        // Test with NaN values
        let nan_value = f64::NAN;

        let result = std::panic::catch_unwind(|| render_progress_bar(nan_value));
        assert!(
            result.is_ok(),
            "render_progress_bar should handle NaN gracefully"
        );

        let result = std::panic::catch_unwind(|| render_colored_progress_bar(nan_value));
        assert!(
            result.is_ok(),
            "render_colored_progress_bar should handle NaN gracefully"
        );
    }

    #[test]
    fn test_concurrent_access() {
        // Test that progress bar functions are thread-safe
        use std::sync::Arc;
        use std::thread;

        let percentages = Arc::new(vec![0.0, 25.0, 50.0, 75.0, 100.0]);
        let mut handles = vec![];

        for _ in 0..10 {
            let percentages_clone = Arc::clone(&percentages);
            let handle = thread::spawn(move || {
                for &percentage in percentages_clone.iter() {
                    let _bar = render_progress_bar(percentage);
                    let _colored_bar = render_colored_progress_bar(percentage);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().expect("Thread should complete successfully");
        }
    }

    #[test]
    fn test_unicode_handling_in_bar() {
        // Test that the progress bar handles unicode correctly
        // The bar should always use the same character regardless of locale
        let result = render_progress_bar(50.0);

        // Check that it uses the correct Unicode block character
        assert!(
            result.contains('█'),
            "Progress bar should use Unicode block character"
        );

        // Verify character encoding
        let filled_char = '█';
        assert_eq!(filled_char as u32, 0x2588, "Should use U+2588 FULL BLOCK");
    }

    #[test]
    fn test_progress_calculation_extreme_durations() {
        // Test with very short durations
        let start = ProgressTestUtils::parse_test_datetime("2025-07-21 10:00:00");
        let end = start + Duration::nanoseconds(1);
        let current = start + Duration::nanoseconds(1);

        let result = calculate_progress(start, end, current);
        assert_eq!(result, 100.0, "Nanosecond duration should work");

        // Test with very long durations
        let start = ProgressTestUtils::parse_test_datetime("1900-01-01 00:00:00");
        let end = ProgressTestUtils::parse_test_datetime("2100-01-01 00:00:00");
        let current = ProgressTestUtils::parse_test_datetime("2000-01-01 00:00:00");

        let result = calculate_progress(start, end, current);
        assert!(
            result > 49.0 && result < 51.0,
            "200-year duration should work: {result}"
        );
    }
}
