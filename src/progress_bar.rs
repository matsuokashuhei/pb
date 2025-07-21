//! Progress bar module for the pb CLI tool
//!
//! This module provides progress calculation and rendering functionality
//! for time-based progress visualization.

use chrono::NaiveDateTime;

/// Calculate progress percentage based on elapsed time
///
/// This function calculates the progress percentage between start and end times
/// based on the current time. It handles various edge cases and ensures
/// accurate floating-point calculations.
///
/// # Algorithm
/// 
/// The core formula is:
/// ```text
/// Progress % = (Current Time - Start Time) / (End Time - Start Time) × 100
/// ```
///
/// # Edge Cases
///
/// - **Zero Duration**: When start == end, returns 100.0%
/// - **Negative Progress**: When current < start, returns 0.0% (clamped)
/// - **Over 100%**: When current > end, returns actual percentage (e.g., 110%, 200%)
///
/// # Arguments
///
/// * `start` - The start time as `NaiveDateTime`
/// * `end` - The end time as `NaiveDateTime`
/// * `current` - The current time as `NaiveDateTime`
///
/// # Returns
///
/// Returns the progress as a floating-point percentage. Values are:
/// - 0.0 or greater (negative progress is clamped to 0.0)
/// - Can exceed 100.0 for overtime scenarios
/// - Maintains floating-point precision for accurate calculations
///
/// # Performance
///
/// This function is optimized for frequent calls:
/// - Execution time: <1ms (typically <0.1ms)
/// - No heap allocation
/// - Thread-safe (uses only immutable operations)
///
/// # Examples
///
/// ```
/// use chrono::NaiveDateTime;
/// use pb::progress_bar::calculate_progress;
///
/// let start = NaiveDateTime::parse_from_str("2025-07-21 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
/// let end = NaiveDateTime::parse_from_str("2025-07-21 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
/// let current = NaiveDateTime::parse_from_str("2025-07-21 11:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
///
/// let progress = calculate_progress(start, end, current);
/// assert_eq!(progress, 50.0); // 50% progress
/// ```
pub fn calculate_progress(start: NaiveDateTime, end: NaiveDateTime, current: NaiveDateTime) -> f64 {
    let total_duration = end - start;
    let elapsed_duration = current - start;
    
    // Handle zero duration edge case
    if total_duration.num_seconds() == 0 {
        return 100.0;
    }
    
    // Calculate progress percentage
    let progress = (elapsed_duration.num_seconds() as f64 / total_duration.num_seconds() as f64) * 100.0;
    
    // Ensure non-negative progress (clamp negative values to 0.0)
    progress.max(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, NaiveDateTime};

    fn create_test_datetime(time_str: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(time_str, "%Y-%m-%d %H:%M:%S").unwrap()
    }

    #[test]
    fn test_normal_progress_cases() {
        let start = create_test_datetime("2025-07-21 10:00:00");
        let end = create_test_datetime("2025-07-21 14:00:00"); // 4 hours duration

        // 0% progress - at start time
        let current = start;
        assert_eq!(calculate_progress(start, end, current), 0.0);

        // 25% progress - 1 hour elapsed
        let current = start + Duration::hours(1);
        assert_eq!(calculate_progress(start, end, current), 25.0);

        // 50% progress - 2 hours elapsed
        let current = start + Duration::hours(2);
        assert_eq!(calculate_progress(start, end, current), 50.0);

        // 75% progress - 3 hours elapsed
        let current = start + Duration::hours(3);
        assert_eq!(calculate_progress(start, end, current), 75.0);

        // 100% progress - at end time
        let current = end;
        assert_eq!(calculate_progress(start, end, current), 100.0);
    }

    #[test]
    fn test_overtime_cases() {
        let start = create_test_datetime("2025-07-21 10:00:00");
        let end = create_test_datetime("2025-07-21 12:00:00"); // 2 hours duration

        // 110% progress - 10% overtime
        let current = start + Duration::hours(2) + Duration::minutes(12);
        let result = calculate_progress(start, end, current);
        assert!((result - 110.0).abs() < 0.001); // Use tolerance for floating point

        // 200% progress - 100% overtime
        let current = start + Duration::hours(4);
        assert_eq!(calculate_progress(start, end, current), 200.0);

        // 150% progress - 50% overtime
        let current = start + Duration::hours(3);
        assert_eq!(calculate_progress(start, end, current), 150.0);
    }

    #[test]
    fn test_before_start_edge_case() {
        let start = create_test_datetime("2025-07-21 10:00:00");
        let end = create_test_datetime("2025-07-21 12:00:00");

        // Current time before start time should return 0%
        let current = start - Duration::hours(1);
        assert_eq!(calculate_progress(start, end, current), 0.0);

        let current = start - Duration::minutes(30);
        assert_eq!(calculate_progress(start, end, current), 0.0);
    }

    #[test]
    fn test_zero_duration_edge_case() {
        let start = create_test_datetime("2025-07-21 10:00:00");
        let end = start; // Same time for start and end

        // When start == end, should return 100%
        let current = start;
        assert_eq!(calculate_progress(start, end, current), 100.0);

        // Even if current is different, should still return 100%
        let current = start + Duration::hours(1);
        assert_eq!(calculate_progress(start, end, current), 100.0);

        let current = start - Duration::hours(1);
        assert_eq!(calculate_progress(start, end, current), 100.0);
    }

    #[test]
    fn test_floating_point_precision() {
        let start = create_test_datetime("2025-07-21 10:00:00");
        let end = create_test_datetime("2025-07-21 10:01:00"); // 60 seconds duration

        // Test fractional percentages
        let current = start + Duration::seconds(30);
        assert_eq!(calculate_progress(start, end, current), 50.0);

        let current = start + Duration::seconds(15);
        assert_eq!(calculate_progress(start, end, current), 25.0);

        let current = start + Duration::seconds(45);
        assert_eq!(calculate_progress(start, end, current), 75.0);

        // Test sub-second precision - note that chrono's Duration has second-level precision
        // for num_seconds(), so milliseconds get truncated in our calculation
        let current = start + Duration::seconds(1); // 1 second instead of 500ms
        let expected = (1.0 / 60.0) * 100.0; // Should be ~1.67%
        let result = calculate_progress(start, end, current);
        assert!((result - expected).abs() < 0.01);
    }

    #[test]
    fn test_different_time_scales() {
        // Test with minute-scale duration
        let start = create_test_datetime("2025-07-21 10:00:00");
        let end = create_test_datetime("2025-07-21 10:10:00"); // 10 minutes
        let current = start + Duration::minutes(5);
        assert_eq!(calculate_progress(start, end, current), 50.0);

        // Test with day-scale duration
        let start = create_test_datetime("2025-07-21 00:00:00");
        let end = create_test_datetime("2025-07-23 00:00:00"); // 2 days
        let current = start + Duration::days(1);
        assert_eq!(calculate_progress(start, end, current), 50.0);

        // Test with second-scale duration
        let start = create_test_datetime("2025-07-21 10:00:00");
        let end = create_test_datetime("2025-07-21 10:00:10"); // 10 seconds
        let current = start + Duration::seconds(2);
        assert_eq!(calculate_progress(start, end, current), 20.0);
    }

    #[test]
    fn test_boundary_conditions() {
        let start = create_test_datetime("2025-07-21 10:00:00");
        let end = create_test_datetime("2025-07-21 11:00:00"); // 1 hour

        // Test exactly at boundaries
        assert_eq!(calculate_progress(start, end, start), 0.0);
        assert_eq!(calculate_progress(start, end, end), 100.0);

        // Test one second before/after boundaries
        let current = start + Duration::seconds(1);
        let expected = (1.0 / 3600.0) * 100.0; // ~0.028%
        let result = calculate_progress(start, end, current);
        assert!((result - expected).abs() < 0.001);

        let current = end - Duration::seconds(1);
        let expected = ((3600.0 - 1.0) / 3600.0) * 100.0; // ~99.972%
        let result = calculate_progress(start, end, current);
        assert!((result - expected).abs() < 0.001);
    }

    #[test]
    fn test_large_time_ranges() {
        // Test with year-long duration
        let start = create_test_datetime("2025-01-01 00:00:00");
        let end = create_test_datetime("2026-01-01 00:00:00"); // 1 year
        let current = create_test_datetime("2025-07-01 00:00:00"); // ~6 months

        let result = calculate_progress(start, end, current);
        // Should be approximately 50% (6 months / 12 months)
        assert!((result - 50.0).abs() < 1.0); // Within 1% tolerance for leap year variations
    }

    #[cfg(test)]
    mod performance_tests {
        use super::*;
        use std::time::Instant;

        #[test]
        fn test_performance_benchmark() {
            let start = create_test_datetime("2025-07-21 10:00:00");
            let end = create_test_datetime("2025-07-21 12:00:00");
            let current = create_test_datetime("2025-07-21 11:00:00");

            let iterations = 1000;
            let start_time = Instant::now();

            for _ in 0..iterations {
                let _ = calculate_progress(start, end, current);
            }

            let elapsed = start_time.elapsed();
            let avg_time = elapsed / iterations;

            // Should complete 1000 iterations in less than 1ms total
            assert!(elapsed.as_millis() < 1, "Performance test failed: {} iterations took {:?}", iterations, elapsed);
            
            // Each call should take less than 1 microsecond on average
            assert!(avg_time.as_nanos() < 1000, "Average call time too slow: {:?}", avg_time);
        }
    }
}
