use chrono::NaiveDateTime;
use pmon::progress_bar::format_verbose_layout;

#[test]
fn test_verbose_layout_exact_format() {
    // Test with the exact scenario from the user's comment
    let start = NaiveDateTime::parse_from_str("2025-08-02 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let end = NaiveDateTime::parse_from_str("2025-08-31 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let current =
        NaiveDateTime::parse_from_str("2025-08-02 18:00:00", "%Y-%m-%d %H:%M:%S").unwrap();

    let layout = format_verbose_layout(2.5, start, end, current);
    let lines: Vec<&str> = layout.split('\n').collect();

    // Verify we have exactly 3 lines
    assert_eq!(lines.len(), 3, "Verbose layout should have exactly 3 lines");

    // Verify line 1: date line with proper formatting
    assert_eq!(lines[0], "2025-08-02                    2025-08-31");
    assert_eq!(
        lines[0].len(),
        40,
        "Date line should be exactly 40 characters"
    );

    // Verify line 2: progress bar with no leading spaces
    assert!(
        lines[1].starts_with('█'),
        "Progress bar should start with filled character"
    );
    assert_eq!(
        lines[1].chars().count(),
        40,
        "Progress bar should be exactly 40 visual characters"
    );
    assert_eq!(
        lines[1].chars().take_while(|&c| c == ' ').count(),
        0,
        "Progress bar should have no leading spaces"
    );

    // Verify line 3: statistics with no leading spaces
    assert!(
        lines[2].starts_with("2.5%"),
        "Statistics should start with percentage"
    );
    assert!(
        lines[2].contains("elapsed"),
        "Statistics should contain 'elapsed'"
    );
    assert!(
        lines[2].contains("remaining"),
        "Statistics should contain 'remaining'"
    );
    assert_eq!(
        lines[2].chars().take_while(|&c| c == ' ').count(),
        0,
        "Statistics should have no leading spaces"
    );
}

#[test]
fn test_verbose_layout_alignment_consistency() {
    // Test that the layout maintains consistent left alignment across different scenarios
    let test_cases = vec![
        (
            "2025-01-01 00:00:00",
            "2025-12-31 00:00:00",
            "2025-06-15 00:00:00",
            50.0,
        ),
        (
            "2025-08-02 00:00:00",
            "2025-08-31 00:00:00",
            "2025-08-02 18:00:00",
            2.5,
        ),
        (
            "2025-01-01 10:00:00",
            "2025-01-01 20:00:00",
            "2025-01-01 15:00:00",
            50.0,
        ),
    ];

    for (start_str, end_str, current_str, percentage) in test_cases {
        let start = NaiveDateTime::parse_from_str(start_str, "%Y-%m-%d %H:%M:%S").unwrap();
        let end = NaiveDateTime::parse_from_str(end_str, "%Y-%m-%d %H:%M:%S").unwrap();
        let current = NaiveDateTime::parse_from_str(current_str, "%Y-%m-%d %H:%M:%S").unwrap();

        let layout = format_verbose_layout(percentage, start, end, current);
        let lines: Vec<&str> = layout.split('\n').collect();

        // All lines should be left-aligned (no leading spaces)
        for (i, line) in lines.iter().enumerate() {
            let leading_spaces = line.chars().take_while(|&c| c == ' ').count();
            assert_eq!(
                leading_spaces, 0,
                "Line {} should have no leading spaces in scenario ({}, {}, {}, {}), got line: '{}'",
                i + 1, start_str, end_str, current_str, percentage, line
            );
        }

        // Date line should be exactly 40 characters
        assert_eq!(
            lines[0].len(),
            40,
            "Date line should be 40 characters in scenario ({}, {}, {}, {})",
            start_str,
            end_str,
            current_str,
            percentage
        );

        // Progress bar should be exactly 40 visual characters
        assert_eq!(
            lines[1].chars().count(),
            40,
            "Progress bar should be 40 visual characters in scenario ({}, {}, {}, {})",
            start_str,
            end_str,
            current_str,
            percentage
        );
    }
}
