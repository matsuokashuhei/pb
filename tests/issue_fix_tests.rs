//! Tests to validate the fixes for Issues 1 and 2

use pb::{parse_time, parse_time_with_base};
use chrono::NaiveDateTime;

#[cfg(test)]
mod issue_fix_tests {
    use super::*;

    #[test]
    fn test_issue_1_base_time_calculation_fix() {
        // Test that relative end times are calculated from start time, not current time
        let start_time = NaiveDateTime::parse_from_str("2025-01-27 14:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        
        // Test all examples from README
        
        // Example 1: "2h" should add 2 hours to start time
        let end_time_2h = parse_time_with_base("2h", Some(start_time)).unwrap();
        let expected_2h = NaiveDateTime::parse_from_str("2025-01-27 16:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        assert_eq!(end_time_2h, expected_2h, "2h should be 2 hours after start time");
        
        // Example 2: "90m" should add 90 minutes to start time  
        let end_time_90m = parse_time_with_base("90m", Some(start_time)).unwrap();
        let expected_90m = NaiveDateTime::parse_from_str("2025-01-27 15:30:00", "%Y-%m-%d %H:%M:%S").unwrap();
        assert_eq!(end_time_90m, expected_90m, "90m should be 90 minutes after start time");
        
        // Example 4: "3600s" should add 3600 seconds (1 hour) to start time
        let end_time_3600s = parse_time_with_base("3600s", Some(start_time)).unwrap();
        let expected_3600s = NaiveDateTime::parse_from_str("2025-01-27 15:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        assert_eq!(end_time_3600s, expected_3600s, "3600s should be 3600 seconds (1 hour) after start time");
        
        // Example 3: "7d" should add 7 days to start time (date-only start)
        let start_date = NaiveDateTime::parse_from_str("2025-01-27 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let end_time_7d = parse_time_with_base("7d", Some(start_date)).unwrap();
        let expected_7d = NaiveDateTime::parse_from_str("2025-02-03 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        assert_eq!(end_time_7d, expected_7d, "7d should be 7 days after start date");
    }
    
    #[test]
    fn test_issue_2_seconds_support_fix() {
        // Test that seconds unit is now supported
        let base_time = NaiveDateTime::parse_from_str("2025-01-27 14:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        
        // Test basic seconds support
        let result_30s = parse_time_with_base("30s", Some(base_time));
        assert!(result_30s.is_ok(), "30s should be valid");
        let expected_30s = NaiveDateTime::parse_from_str("2025-01-27 14:00:30", "%Y-%m-%d %H:%M:%S").unwrap();
        assert_eq!(result_30s.unwrap(), expected_30s, "30s should add 30 seconds to base time");
        
        // Test the problematic example from README: 3600s
        let result_3600s = parse_time_with_base("3600s", Some(base_time));
        assert!(result_3600s.is_ok(), "3600s should be valid");
        let expected_3600s = NaiveDateTime::parse_from_str("2025-01-27 15:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        assert_eq!(result_3600s.unwrap(), expected_3600s, "3600s should add 3600 seconds (1 hour) to base time");
        
        // Test various second values
        let test_cases = vec![
            ("1s", "2025-01-27 14:00:01"),
            ("60s", "2025-01-27 14:01:00"),  // 1 minute
            ("120s", "2025-01-27 14:02:00"), // 2 minutes
            ("3661s", "2025-01-27 15:01:01"), // 1 hour, 1 minute, 1 second
        ];
        
        for (input, expected_str) in test_cases {
            let result = parse_time_with_base(input, Some(base_time));
            assert!(result.is_ok(), "{} should be valid", input);
            let expected = NaiveDateTime::parse_from_str(expected_str, "%Y-%m-%d %H:%M:%S").unwrap();
            assert_eq!(result.unwrap(), expected, "{} should produce correct result", input);
        }
    }
    
    #[test]
    fn test_backward_compatibility() {
        // Test that the original parse_time function still works for non-relative times
        
        // Date parsing should work the same
        let date_result = parse_time("2025-01-27");
        assert!(date_result.is_ok(), "Date parsing should still work");
        
        // DateTime parsing should work the same  
        let datetime_result = parse_time("2025-01-27 14:00:00");
        assert!(datetime_result.is_ok(), "DateTime parsing should still work");
        
        // Relative time parsing should still use current time when no base is provided
        let relative_result = parse_time("2h");
        assert!(relative_result.is_ok(), "Relative time parsing should still work");
        
        // The relative time should be different from when we provide a specific base
        let specific_base = NaiveDateTime::parse_from_str("2025-01-27 14:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let with_base_result = parse_time_with_base("2h", Some(specific_base));
        assert!(with_base_result.is_ok(), "Relative time with base should work");
        
        // They should be different (unless current time happens to match the base, which is extremely unlikely)
        assert_ne!(relative_result.unwrap(), with_base_result.unwrap(), 
                  "Relative time should be different when using current time vs specific base");
    }
    
    #[test]
    fn test_edge_cases_for_fixes() {
        let base_time = NaiveDateTime::parse_from_str("2025-01-27 14:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        
        // Test boundary values for seconds
        let result_1s = parse_time_with_base("1s", Some(base_time));
        assert!(result_1s.is_ok(), "1s should be valid (minimum)");
        
        let result_99999s = parse_time_with_base("99999s", Some(base_time));
        assert!(result_99999s.is_ok(), "99999s should be valid (maximum)");
        
        // Test that values over the limit are rejected
        let result_100000s = parse_time_with_base("100000s", Some(base_time));
        assert!(result_100000s.is_err(), "100000s should be invalid (over limit)");
        
        // Test combined with other units still work
        let result_1m = parse_time_with_base("1m", Some(base_time));
        assert!(result_1m.is_ok(), "Minutes should still work");
        
        let result_1h = parse_time_with_base("1h", Some(base_time));
        assert!(result_1h.is_ok(), "Hours should still work");
        
        let result_1d = parse_time_with_base("1d", Some(base_time));
        assert!(result_1d.is_ok(), "Days should still work");
    }
}