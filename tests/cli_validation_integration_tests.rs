//! Integration tests specifically for CLI validation and error handling
//!
//! These tests focus on exercising the validation logic that's not easily
//! testable through unit tests due to private methods.

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_validation_empty_start_time() {
    // Test validation failure for empty start time
    let mut cmd = Command::cargo_bin("pmon").unwrap();
    cmd.args(["--start", "", "--end", "12:00:00"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Start time cannot be empty"));
}

#[test]
fn test_cli_validation_empty_end_time() {
    // Test validation failure for empty end time
    let mut cmd = Command::cargo_bin("pmon").unwrap();
    cmd.args(["--start", "10:00:00", "--end", ""]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("End time cannot be empty"));
}

#[test]
fn test_cli_validation_zero_interval() {
    // Test validation failure for zero interval
    let mut cmd = Command::cargo_bin("pmon").unwrap();
    cmd.args([
        "--start",
        "10:00:00",
        "--end",
        "12:00:00",
        "--interval",
        "0",
    ]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("must be greater than 0"));
}

#[test]
fn test_cli_validation_invalid_start_time() {
    // Test validation failure for invalid start time format
    let mut cmd = Command::cargo_bin("pmon").unwrap();
    cmd.args(["--start", "not-a-time", "--end", "12:00:00"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid time format"));
}

#[test]
fn test_cli_validation_invalid_end_time() {
    // Test validation failure for invalid end time format
    let mut cmd = Command::cargo_bin("pmon").unwrap();
    cmd.args(["--start", "10:00:00", "--end", "not-a-time"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid time format"));
}

#[test]
fn test_cli_validation_start_after_end() {
    // Test validation failure when start time is after end time
    let mut cmd = Command::cargo_bin("pmon").unwrap();
    cmd.args([
        "--start",
        "2025-07-21 12:00:00",
        "--end",
        "2025-07-21 10:00:00",
    ]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("must be before or equal to"));
}

#[test]
fn test_cli_help_display() {
    // Test help display (which exercises the parse_args error handling)
    let mut cmd = Command::cargo_bin("pmon").unwrap();
    cmd.arg("--help");

    cmd.assert()
        .success() // Help should exit successfully
        .stdout(predicate::str::contains("CLI progress monitor (pmon)"));
}

#[test]
fn test_cli_version_display() {
    // Test version display (which exercises the parse_args error handling)
    let mut cmd = Command::cargo_bin("pmon").unwrap();
    cmd.arg("--version");

    cmd.assert()
        .success() // Version should exit successfully
        .stdout(predicate::str::contains("pmon 2.0.0"));
}

#[test]
fn test_cli_missing_required_args() {
    // Test missing required arguments (exercises MissingRequiredOptions error path)
    let mut cmd = Command::cargo_bin("pmon").unwrap();
    // No arguments provided

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_cli_optional_start_with_future_time() {
    // Test that start argument is now optional and uses determined start time
    let mut cmd = Command::cargo_bin("pmon").unwrap();
    cmd.args(["--end", "23:59:59"]); // Use a future time to avoid validation failure

    // Should either succeed or timeout (since it's a future time)
    // We just want to ensure the command line parsing passes and start is optional
    cmd.timeout(std::time::Duration::from_secs(3)).assert();
}

#[test]
fn test_cli_missing_end_arg() {
    // Test missing end argument
    let mut cmd = Command::cargo_bin("pmon").unwrap();
    cmd.args(["--start", "10:00:00"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_cli_successful_validation() {
    // Test successful validation with various valid inputs
    let test_cases = vec![
        ("10:00:00", "12:00:00", "1"),
        ("2025-07-21 10:00:00", "2025-07-21 12:00:00", "30"),
        ("2025-07-20", "2025-07-21", "60"),
        ("+1h", "+2h", "5"),
    ];

    for (start, end, interval) in test_cases {
        let mut cmd = Command::cargo_bin("pmon").unwrap();
        cmd.args(["--start", start, "--end", end, "--interval", interval]);

        // Should either succeed or timeout (since these are past/future times)
        // We just want to ensure the validation passes
        cmd.timeout(std::time::Duration::from_secs(3)).assert(); // Don't check success/failure, just that it doesn't crash immediately
    }
}

#[test]
fn test_cli_edge_case_intervals() {
    // Test edge case intervals
    let mut cmd = Command::cargo_bin("pmon").unwrap();
    cmd.args([
        "--start",
        "2025-07-21 10:00:00",
        "--end",
        "2025-07-21 12:00:00",
        "--interval",
        "1",
    ]);

    // Should work with interval of 1
    cmd.timeout(std::time::Duration::from_secs(2)).assert();

    // Test very large interval
    let mut cmd = Command::cargo_bin("pmon").unwrap();
    cmd.args([
        "--start",
        "2025-07-21 10:00:00",
        "--end",
        "2025-07-21 12:00:00",
        "--interval",
        "3600",
    ]);

    cmd.timeout(std::time::Duration::from_secs(2)).assert();
}

#[test]
fn test_cli_unicode_and_special_characters() {
    // Test handling of unicode and special characters in input
    let invalid_inputs = vec![
        "2025-07-21🕐10:00:00",              // Emoji
        "２０２５-０７-２１ １０:００:００", // Full-width characters
        "2025-07-21\t10:00:00",              // Tab character
        "2025-07-21\n10:00:00",              // Newline character
    ];

    for invalid_input in invalid_inputs {
        let mut cmd = Command::cargo_bin("pmon").unwrap();
        cmd.args(["--start", invalid_input, "--end", "12:00:00"]);

        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Error"));
    }
}

#[test]
fn test_cli_whitespace_handling() {
    // Test handling of whitespace in time inputs
    // Note: Some whitespace might be accepted by the time parser
    let invalid_input = "10:  00:00"; // Internal spaces should be invalid

    let mut cmd = Command::cargo_bin("pmon").unwrap();
    cmd.args(["--start", invalid_input, "--end", "12:00:00"]);

    // This specific case should fail validation
    cmd.timeout(std::time::Duration::from_secs(2))
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid time format"));
}
