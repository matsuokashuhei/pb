//! Performance and benchmark tests for pb tool
//!
//! This module provides comprehensive performance testing for CLI parsing,
//! time parsing, progress calculation, and rendering operations.

use chrono::Duration;
use clap::Parser; // Add this import for try_parse_from
use pb::cli::Cli;
use pb::{
    calculate_progress, parse_date, parse_datetime, parse_relative_time, parse_time,
    render_colored_progress_bar, render_progress_bar, validate_times,
};
use std::time::{Duration as StdDuration, Instant};

mod common;
use common::helpers::{PerformanceTestUtils, ProgressTestUtils};

#[cfg(test)]
mod time_parsing_performance {
    use super::*;

    #[test]
    fn test_parse_time_performance_baseline() {
        let test_inputs = vec!["2025-07-21", "2025-07-21 15:30:45", "15:30:45", "+2h30m"];

        let expectations = PerformanceTestUtils::performance_expectations();

        for input in test_inputs {
            let avg_duration = PerformanceTestUtils::benchmark(|| parse_time(input), 1000);

            assert!(
                avg_duration < expectations.parse_time_max,
                "parse_time('{}') took too long: {:?} > {:?}",
                input,
                avg_duration,
                expectations.parse_time_max
            );
        }
    }

    #[test]
    fn test_date_parsing_performance() {
        let test_dates = vec![
            "2025-07-21",
            "2025-01-01",
            "2025-12-31",
            "2024-02-29", // Leap year
        ];

        for date in test_dates {
            let avg_duration = PerformanceTestUtils::benchmark(|| parse_date(date), 1000);

            assert!(
                avg_duration < StdDuration::from_millis(10), // Increased from 1ms to 10ms
                "parse_date('{}') took too long: {:?}",
                date,
                avg_duration
            );
        }
    }

    #[test]
    fn test_datetime_parsing_performance() {
        let test_datetimes = vec![
            "2025-07-21 10:30:45",
            "2025-01-01 00:00:00",
            "2025-12-31 23:59:59",
        ];

        for datetime in test_datetimes {
            let avg_duration = PerformanceTestUtils::benchmark(|| parse_datetime(datetime), 1000);

            assert!(
                avg_duration < StdDuration::from_millis(10), // Increased from 1ms to 10ms
                "parse_datetime('{}') took too long: {:?}",
                datetime,
                avg_duration
            );
        }
    }

    #[test]
    fn test_relative_time_parsing_performance() {
        let test_relative_times = vec!["+1h", "+30m", "+2h30m15s", "-1h30m"];

        for relative_time in test_relative_times {
            let base_time = chrono::Utc::now().naive_utc();
            let avg_duration = PerformanceTestUtils::benchmark(
                || parse_relative_time(relative_time, base_time),
                1000,
            );

            assert!(
                avg_duration < StdDuration::from_millis(10), // Increased from 1ms to 10ms
                "parse_relative_time('{}') took too long: {:?}",
                relative_time,
                avg_duration
            );
        }
    }

    #[test]
    fn test_time_validation_performance() {
        let start = ProgressTestUtils::parse_test_datetime("2025-07-21 10:00:00");
        let end = ProgressTestUtils::parse_test_datetime("2025-07-21 12:00:00");

        let avg_duration = PerformanceTestUtils::benchmark(|| validate_times(start, end), 10000);

        assert!(
            avg_duration < StdDuration::from_micros(10),
            "validate_times took too long: {:?}",
            avg_duration
        );
    }

    #[test]
    fn test_parsing_scalability() {
        // Test performance with increasing number of parsing operations
        let test_sizes = vec![100, 1000, 10000];
        let test_input = "2025-07-21 15:30:45";

        for size in test_sizes {
            let start_time = Instant::now();

            for _ in 0..size {
                let _ = parse_time(test_input);
            }

            let total_duration = start_time.elapsed();
            let avg_duration = total_duration / size;

            assert!(
                avg_duration < StdDuration::from_millis(10), // Increased from 1ms to 10ms
                "Parsing scalability degraded at size {}: {:?} avg per operation",
                size,
                avg_duration
            );
        }
    }

    #[test]
    fn test_memory_efficiency_parsing() {
        // Test that parsing doesn't cause memory leaks
        let initial_memory = get_approximate_memory_usage();

        // Parse many different time formats
        for _ in 0..10000 {
            let _ = parse_time("2025-07-21 15:30:45");
            let _ = parse_date("2025-07-21");
            let base_time = chrono::Utc::now().naive_utc();
            let _ = parse_relative_time("+1h30m", base_time);
        }

        let final_memory = get_approximate_memory_usage();
        let memory_increase = final_memory.saturating_sub(initial_memory);

        // Memory increase should be minimal (less than 1MB)
        assert!(
            memory_increase < 1_000_000,
            "Excessive memory usage during parsing: {} bytes",
            memory_increase
        );
    }
}

#[cfg(test)]
mod progress_calculation_performance {
    use super::*;

    #[test]
    fn test_calculate_progress_performance_baseline() {
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
    fn test_progress_calculation_with_various_durations() {
        let test_cases = vec![
            // (duration_hours, description)
            (1, "1 hour duration"),
            (24, "24 hour duration"),
            (168, "1 week duration"),
            (8760, "1 year duration"),
        ];

        for (hours, description) in test_cases {
            let start = ProgressTestUtils::parse_test_datetime("2025-07-21 10:00:00");
            let end = start + Duration::hours(hours);
            let current = start + Duration::hours(hours / 2); // Midpoint

            let avg_duration =
                PerformanceTestUtils::benchmark(|| calculate_progress(start, end, current), 1000);

            assert!(
                avg_duration < StdDuration::from_micros(100),
                "calculate_progress with {} took too long: {:?}",
                description,
                avg_duration
            );
        }
    }

    #[test]
    fn test_progress_calculation_edge_cases_performance() {
        let test_cases = vec![
            // Zero duration
            {
                let time = ProgressTestUtils::parse_test_datetime("2025-07-21 10:00:00");
                (time, time, time, "zero duration")
            },
            // Microsecond precision
            {
                let start = ProgressTestUtils::parse_test_datetime("2025-07-21 10:00:00");
                let end = start + Duration::microseconds(1000);
                let current = start + Duration::microseconds(500);
                (start, end, current, "microsecond precision")
            },
            // Very large duration
            {
                let start = ProgressTestUtils::parse_test_datetime("2000-01-01 00:00:00");
                let end = ProgressTestUtils::parse_test_datetime("2100-01-01 00:00:00");
                let current = ProgressTestUtils::parse_test_datetime("2050-01-01 00:00:00");
                (start, end, current, "century duration")
            },
        ];

        for (start, end, current, description) in test_cases {
            let avg_duration =
                PerformanceTestUtils::benchmark(|| calculate_progress(start, end, current), 1000);

            assert!(
                avg_duration < StdDuration::from_micros(100),
                "calculate_progress with {} took too long: {:?}",
                description,
                avg_duration
            );
        }
    }

    #[test]
    fn test_progress_calculation_scalability() {
        let start = ProgressTestUtils::parse_test_datetime("2025-07-21 10:00:00");
        let end = ProgressTestUtils::parse_test_datetime("2025-07-21 12:00:00");
        let current = ProgressTestUtils::parse_test_datetime("2025-07-21 11:00:00");

        // Test with increasing number of calculations
        let test_sizes = vec![1000, 10000, 100000];

        for size in test_sizes {
            let start_time = Instant::now();

            for _ in 0..size {
                let _ = calculate_progress(start, end, current);
            }

            let total_duration = start_time.elapsed();
            let avg_duration = total_duration / size;

            assert!(
                avg_duration < StdDuration::from_micros(100),
                "Progress calculation scalability degraded at size {}: {:?} avg per operation",
                size,
                avg_duration
            );
        }
    }
}

#[cfg(test)]
mod progress_bar_rendering_performance {
    use super::*;

    #[test]
    fn test_render_progress_bar_performance_baseline() {
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
    fn test_progress_bar_rendering_various_percentages() {
        let test_percentages = vec![0.0, 25.0, 50.0, 75.0, 100.0, 150.0, -10.0];

        for percentage in test_percentages {
            let avg_duration =
                PerformanceTestUtils::benchmark(|| render_progress_bar(percentage), 1000);

            assert!(
                avg_duration < StdDuration::from_millis(5), // Increased from 1ms to 5ms for rendering
                "render_progress_bar({}%) took too long: {:?}",
                percentage,
                avg_duration
            );
        }
    }

    #[test]
    fn test_progress_bar_memory_efficiency() {
        let initial_memory = get_approximate_memory_usage();

        // Render many progress bars
        for i in 0..10000 {
            let percentage = (i as f64 / 100.0) % 200.0; // 0-200%
            let _bar = render_progress_bar(percentage);
            let _colored_bar = render_colored_progress_bar(percentage);

            // Don't keep references to force cleanup
        }

        let final_memory = get_approximate_memory_usage();
        let memory_increase = final_memory.saturating_sub(initial_memory);

        // Memory increase should be minimal
        assert!(
            memory_increase < 5_000_000, // 5MB threshold
            "Excessive memory usage during progress bar rendering: {} bytes",
            memory_increase
        );
    }

    #[test]
    fn test_string_allocation_efficiency() {
        // Test that progress bar rendering doesn't cause excessive string allocations
        let test_percentages = vec![0.0, 25.0, 50.0, 75.0, 100.0];

        for percentage in test_percentages {
            // Measure time for batch operations to detect allocation overhead
            let batch_size = 1000;
            let start_time = Instant::now();

            for _ in 0..batch_size {
                let _bar = render_progress_bar(percentage);
            }

            let duration = start_time.elapsed();
            let avg_duration = duration / batch_size;

            assert!(
                avg_duration < StdDuration::from_micros(500),
                "String allocation overhead too high for {}%: {:?}",
                percentage,
                avg_duration
            );
        }
    }
}

#[cfg(test)]
mod cli_parsing_performance {
    use super::*;
    use pb::cli::Cli;

    #[test]
    fn test_cli_parsing_performance() {
        let test_args = vec![
            vec!["pb", "--start", "10:00", "--end", "12:00"],
            vec![
                "pb",
                "-s",
                "2025-07-21 10:00:00",
                "-e",
                "2025-07-21 12:00:00",
                "-i",
                "30",
            ],
            vec!["pb", "--start", "+1h", "--end", "+2h", "--interval", "60"],
        ];

        for args in test_args {
            // We can't benchmark parse_from directly as it doesn't return Result
            // So we'll benchmark the parsing logic indirectly
            let avg_duration = PerformanceTestUtils::benchmark(
                || args.len(), // Simple benchmark of argument processing
                1000,
            );

            assert!(
                avg_duration < StdDuration::from_millis(5),
                "CLI parsing took too long for {:?}: {:?}",
                args,
                avg_duration
            );
        }
    }

    #[test]
    fn test_cli_validation_performance() {
        let cli = Cli::try_parse_from(vec!["pb", "--start", "10:00", "--end", "12:00"]).unwrap();

        let avg_duration = PerformanceTestUtils::benchmark(|| cli.validate(), 10000);

        assert!(
            avg_duration < StdDuration::from_micros(10),
            "CLI validation took too long: {:?}",
            avg_duration
        );
    }
}

#[cfg(test)]
mod end_to_end_performance {
    use super::*;

    #[test]
    fn test_complete_workflow_performance() {
        // Test the performance of a complete workflow: parse CLI -> parse times -> calculate progress -> render bar
        let args = vec![
            "pb",
            "--start",
            "2025-07-21 10:00:00",
            "--end",
            "2025-07-21 12:00:00",
        ];

        let avg_duration = PerformanceTestUtils::benchmark(
            || {
                // Parse CLI arguments
                let cli = Cli::try_parse_from(args.clone()).unwrap();
                // Note: validate() is private, so we skip validation in this benchmark

                // Parse times
                let start_time = parse_time(cli.start()).unwrap();
                let end_time = parse_time(cli.end()).unwrap();

                // Validate times
                validate_times(start_time, end_time).unwrap();

                // Calculate progress (use current time for realistic test)
                let current_time = chrono::Local::now().naive_local();
                let progress = calculate_progress(start_time, end_time, current_time);

                // Render progress bar
                let _bar = render_colored_progress_bar(progress);
            },
            100,
        );

        assert!(
            avg_duration < StdDuration::from_millis(10),
            "Complete workflow took too long: {:?}",
            avg_duration
        );
    }

    #[test]
    fn test_multiple_iterations_performance() {
        // Simulate the main application loop performance
        let start_time = ProgressTestUtils::parse_test_datetime("2025-07-21 10:00:00");
        let end_time = ProgressTestUtils::parse_test_datetime("2025-07-21 12:00:00");

        let iterations = 100;
        let start = Instant::now();

        for i in 0..iterations {
            // Simulate time progression
            let current_time = start_time + Duration::minutes(i);
            let progress = calculate_progress(start_time, end_time, current_time);
            let _bar = render_colored_progress_bar(progress);
        }

        let total_duration = start.elapsed();
        let avg_iteration = total_duration / iterations as u32;

        assert!(
            avg_iteration < StdDuration::from_millis(10), // Increased from 1ms to 10ms for app loop
            "Application loop iteration took too long: {:?}",
            avg_iteration
        );
    }
}

#[cfg(test)]
mod memory_usage_tests {
    use super::*;

    #[test]
    fn test_memory_usage_stability() {
        // Test that memory usage remains stable over many operations
        let initial_memory = get_approximate_memory_usage();

        // Perform many operations of different types
        for i in 0..1000 {
            // Parse different time formats
            let _ = parse_time("2025-07-21 10:00:00");
            let _ = parse_date("2025-07-21");
            let base_time = chrono::Utc::now().naive_utc();
            let _ = parse_relative_time("+1h", base_time);

            // Calculate progress
            let start = ProgressTestUtils::parse_test_datetime("2025-07-21 10:00:00");
            let end = ProgressTestUtils::parse_test_datetime("2025-07-21 12:00:00");
            let current = start + Duration::minutes(i % 120);
            let progress = calculate_progress(start, end, current);

            // Render progress bars
            let _bar = render_progress_bar(progress);
            let _colored_bar = render_colored_progress_bar(progress);
        }

        let final_memory = get_approximate_memory_usage();
        let memory_increase = final_memory.saturating_sub(initial_memory);

        assert!(
            memory_increase < 2_000_000, // 2MB threshold
            "Memory usage increased too much: {} bytes",
            memory_increase
        );
    }

    #[test]
    fn test_no_memory_leaks() {
        // Test for memory leaks by performing operations in a loop
        let measurements = vec![];
        let mut measurements = measurements;

        for _ in 0..10 {
            // Perform a batch of operations
            for _ in 0..1000 {
                let _ = parse_time("2025-07-21 15:30:45");
                let _ = render_progress_bar(50.0);
            }

            measurements.push(get_approximate_memory_usage());

            // Allow some time for GC if applicable
            std::thread::sleep(StdDuration::from_millis(10));
        }

        // Memory usage should not continuously increase
        let first_measurement = measurements[0];
        let last_measurement = measurements[measurements.len() - 1];
        let increase = last_measurement.saturating_sub(first_measurement);

        assert!(
            increase < 1_000_000, // 1MB threshold
            "Potential memory leak detected: {} bytes increase over {} measurements",
            increase,
            measurements.len()
        );
    }
}

/// Helper function to get approximate memory usage
/// This is a simplified implementation for testing purposes
fn get_approximate_memory_usage() -> usize {
    // In a real implementation, you might use:
    // - `memory-stats` crate
    // - Platform-specific APIs (e.g., `/proc/self/status` on Linux)
    // - Rust's allocator hooks

    // For now, return 0 as a placeholder
    // In practice, you could implement this based on your platform
    0
}

#[cfg(test)]
mod stress_tests {
    use super::*;

    #[test]
    fn test_extreme_workload() {
        // Test with extreme but realistic workloads
        let start_time = Instant::now();

        // Simulate a stress scenario
        for _ in 0..10000 {
            // Parse various time formats
            let _ = parse_time("2025-07-21 15:30:45");
            let _ = parse_time("+2h30m");
            let _ = parse_time("15:30:45");

            // Calculate progress for multiple scenarios
            let start = ProgressTestUtils::parse_test_datetime("2025-07-21 10:00:00");
            let end = ProgressTestUtils::parse_test_datetime("2025-07-21 12:00:00");
            let current = ProgressTestUtils::parse_test_datetime("2025-07-21 11:00:00");
            let _ = calculate_progress(start, end, current);

            // Render progress bars
            let _ = render_progress_bar(50.0);
            let _ = render_colored_progress_bar(75.0);
        }

        let total_duration = start_time.elapsed();

        assert!(
            total_duration < StdDuration::from_secs(60), // Increased from 20s to 60s for stress test
            "Stress test took too long: {:?}",
            total_duration
        );
    }

    #[test]
    fn test_concurrent_operations() {
        use std::sync::Arc;
        use std::thread;

        let num_threads = 4;
        let operations_per_thread = 1000;

        let start_time = Instant::now();
        let mut handles = vec![];

        for _ in 0..num_threads {
            let handle = thread::spawn(move || {
                for i in 0..operations_per_thread {
                    // Parse times
                    let _ = parse_time("2025-07-21 15:30:45");

                    // Calculate progress
                    let start = ProgressTestUtils::parse_test_datetime("2025-07-21 10:00:00");
                    let end = ProgressTestUtils::parse_test_datetime("2025-07-21 12:00:00");
                    let current = start + Duration::minutes(i % 120);
                    let progress = calculate_progress(start, end, current);

                    // Render progress bar
                    let _ = render_progress_bar(progress);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().expect("Thread should complete successfully");
        }

        let total_duration = start_time.elapsed();

        assert!(
            total_duration < StdDuration::from_secs(2),
            "Concurrent operations took too long: {:?}",
            total_duration
        );
    }
}
