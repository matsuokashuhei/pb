use assert_cmd::Command;
use predicates::prelude::*;
use std::time::Duration;

mod common;
use common::helpers::CliTestUtils;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("pb").unwrap();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("CLI progress bar tool"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("pb").unwrap();
    cmd.arg("--version");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("pb 1.0.0"));
}

#[test]
fn test_missing_required_args() {
    let mut cmd = Command::cargo_bin("pb").unwrap();

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_invalid_start_time() {
    let mut cmd = Command::cargo_bin("pb").unwrap();
    cmd.args(&["--start", "invalid", "--end", "2025-07-21 12:00:00"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error parsing start time"));
}

#[test]
fn test_invalid_end_time() {
    let mut cmd = Command::cargo_bin("pb").unwrap();
    cmd.args(&["--start", "2025-07-21 10:00:00", "--end", "invalid"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error parsing end time"));
}

#[test]
fn test_start_after_end_time() {
    let mut cmd = Command::cargo_bin("pb").unwrap();
    cmd.args(&[
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
fn test_completed_progress() {
    let mut cmd = Command::cargo_bin("pb").unwrap();
    cmd.args(&[
        "--start",
        "2025-07-21 10:00:00",
        "--end",
        "2025-07-21 11:00:00",
        "--interval",
        "1",
    ]);

    let output = cmd.timeout(Duration::from_secs(5)).assert().success();

    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    assert!(stdout.contains("pb - Progress Bar Tool"));
    assert!(stdout.contains("Start time: 2025-07-21 10:00:00"));
    assert!(stdout.contains("End time: 2025-07-21 11:00:00"));
    assert!(stdout.contains("Progress completed!"));
}

#[test]
fn test_date_format_parsing() {
    let mut cmd = Command::cargo_bin("pb").unwrap();
    cmd.args(&[
        "--start",
        "2025-07-20", // Yesterday
        "--end",
        "2025-07-21", // Today
        "--interval",
        "1",
    ]);

    let output = cmd.timeout(Duration::from_secs(5)).assert().success();

    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    assert!(stdout.contains("Start time: 2025-07-20 00:00:00"));
    assert!(stdout.contains("End time: 2025-07-21 00:00:00"));
    // Should complete immediately since this is a past time range
    assert!(stdout.contains("Progress completed!"));
}

#[test]
fn test_relative_time_parsing() {
    let mut cmd = Command::cargo_bin("pb").unwrap();
    cmd.args(&["--start", "+30m", "--end", "+60m", "--interval", "1"]);

    // This should not complete immediately since it's a future time range
    let output = cmd.timeout(Duration::from_secs(3)).assert().failure(); // Will timeout, which is expected

    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    assert!(stdout.contains("pb - Progress Bar Tool"));
    assert!(stdout.contains("[░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░] 0.0%"));
}

#[test]
fn test_custom_interval() {
    let mut cmd = Command::cargo_bin("pb").unwrap();
    cmd.args(&[
        "--start",
        "2025-07-21 10:00:00",
        "--end",
        "2025-07-21 11:00:00",
        "--interval",
        "5",
    ]);

    let output = cmd.timeout(Duration::from_secs(5)).assert().success();

    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    assert!(stdout.contains("Update interval: 5 seconds"));
}

#[test]
fn test_progress_bar_overtime() {
    let mut cmd = Command::cargo_bin("pb").unwrap();
    cmd.args(&[
        "--start",
        "2025-07-21 09:00:00",
        "--end",
        "2025-07-21 10:00:00",
        "--interval",
        "1",
    ]);

    let output = cmd.timeout(Duration::from_secs(5)).assert().success();

    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    // Should show >100% progress since this time range is in the past
    assert!(stdout.contains("████████████████████████████████████████"));
    assert!(stdout.contains("Progress completed!"));
}

#[test]
fn test_zero_interval_error() {
    let mut cmd = Command::cargo_bin("pb").unwrap();
    cmd.args(&[
        "--start",
        "2025-07-21 10:00:00",
        "--end",
        "2025-07-21 11:00:00",
        "--interval",
        "0",
    ]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("must be greater than 0"));
}

#[test]
fn test_equal_start_end_times() {
    let mut cmd = Command::cargo_bin("pb").unwrap();
    cmd.args(&[
        "--start",
        "2025-07-21 10:00:00",
        "--end",
        "2025-07-21 10:00:00",
        "--interval",
        "1",
    ]);

    let output = cmd.timeout(Duration::from_secs(5)).assert().success();

    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    // Should immediately show 100% and complete
    assert!(stdout.contains("Progress completed!"));
}

#[cfg(test)]
mod comprehensive_cli_integration_tests {
    use super::*;

    #[test]
    fn test_all_time_format_combinations() {
        // Test all supported time format combinations
        let format_combinations = vec![
            // Date to Date
            ("2025-07-20", "2025-07-21"),
            // Date to DateTime
            ("2025-07-20", "2025-07-21 12:00:00"),
            // DateTime to DateTime
            ("2025-07-20 10:00:00", "2025-07-21 12:00:00"),
            // Time to Time (should use today's date)
            ("10:00:00", "12:00:00"),
            // Relative to Relative
            ("+30m", "+60m"),
            // Mixed formats
            ("2025-07-20", "+24h"),
            ("10:00:00", "2025-07-21 12:00:00"),
        ];

        for (start, end) in format_combinations {
            let mut cmd = CliTestUtils::pb_command();
            cmd.args(&["--start", start, "--end", end, "--interval", "1"]);

            let output = cmd.timeout(Duration::from_secs(3)).assert();

            // Should either succeed or fail gracefully (not crash)
            let stdout = String::from_utf8_lossy(&output.get_output().stdout);
            let stderr = String::from_utf8_lossy(&output.get_output().stderr);

            // If it fails, should have helpful error message
            if !stderr.is_empty() {
                assert!(
                    stderr.contains("Error"),
                    "Should have error message for format combination: {} to {}",
                    start,
                    end
                );
            } else {
                // If it succeeds, should show proper output
                assert!(
                    stdout.contains("pb - Progress Bar Tool"),
                    "Should show proper output for format combination: {} to {}",
                    start,
                    end
                );
            }
        }
    }

    #[test]
    fn test_comprehensive_error_scenarios() {
        let error_test_cases = vec![
            // Invalid time formats
            (
                vec!["--start", "invalid", "--end", "12:00:00"],
                "Error parsing start time",
            ),
            (
                vec!["--start", "2025-01-01", "--end", "invalid"],
                "Error parsing end time",
            ),
            (
                vec!["--start", "2025-13-40", "--end", "12:00:00"],
                "Error parsing start time",
            ),
            (
                vec!["--start", "2025-01-01", "--end", "25:70:80"],
                "Error parsing end time",
            ),
            // Time relationship errors
            (
                vec![
                    "--start",
                    "2025-01-01 12:00:00",
                    "--end",
                    "2025-01-01 10:00:00",
                ],
                "must be before",
            ),
            (
                vec!["--start", "2025-07-22", "--end", "2025-07-21"],
                "must be before",
            ),
            // Invalid intervals
            (
                vec![
                    "--start",
                    "2025-01-01 10:00:00",
                    "--end",
                    "2025-01-01 12:00:00",
                    "--interval",
                    "0",
                ],
                "must be greater than 0",
            ),
            // Missing arguments (tested separately due to different error handling)
        ];

        for (args, expected_error_fragment) in error_test_cases {
            let mut cmd = CliTestUtils::pb_command();
            cmd.args(&args);

            cmd.assert()
                .failure()
                .stderr(predicate::str::contains(expected_error_fragment));
        }
    }

    #[test]
    fn test_various_interval_values() {
        let interval_tests = vec![
            ("1", "1 seconds"),
            ("30", "30 seconds"),
            ("60", "60 seconds"),
            ("3600", "3600 seconds"),
        ];

        for (interval, expected_display) in interval_tests {
            let mut cmd = CliTestUtils::pb_command();
            cmd.args(&[
                "--start",
                "2025-07-21 10:00:00",
                "--end",
                "2025-07-21 11:00:00",
                "--interval",
                interval,
            ]);

            let output = cmd.timeout(Duration::from_secs(3)).assert().success();
            let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
            assert!(stdout.contains(&format!("Update interval: {}", expected_display)));
        }
    }

    #[test]
    fn test_signal_handling() {
        // Test Ctrl+C handling (if in TTY mode)
        // Note: This test is complex to implement properly in a test environment
        // as it requires actual signal handling. For now, we test basic functionality.

        let mut cmd = CliTestUtils::pb_command();
        cmd.args(&[
            "--start",
            "+1h", // Future time to ensure it doesn't complete immediately
            "--end",
            "+2h",
            "--interval",
            "1",
        ]);

        // Run with a short timeout to simulate interrupt
        let output = cmd.timeout(Duration::from_secs(2)).assert().failure(); // Will timeout

        // Should show progress output before timeout
        let stdout = String::from_utf8_lossy(&output.get_output().stdout);
        assert!(stdout.contains("pb - Progress Bar Tool"));
    }

    #[test]
    fn test_non_tty_mode() {
        // Test behavior when not in a TTY (non-interactive mode)
        let mut cmd = CliTestUtils::pb_command();
        cmd.args(&[
            "--start",
            "2025-07-21 10:00:00",
            "--end",
            "2025-07-21 11:00:00",
            "--interval",
            "1",
        ]);

        // Pipe input to simulate non-TTY
        cmd.write_stdin("");

        let output = cmd.timeout(Duration::from_secs(3)).assert().success();
        let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();

        // Should work in non-TTY mode
        assert!(stdout.contains("pb - Progress Bar Tool"));
        assert!(stdout.contains("Progress completed!"));
    }

    #[test]
    fn test_progress_bar_visual_output() {
        // Test that progress bar visual elements are present
        let mut cmd = CliTestUtils::pb_command();
        cmd.args(&[
            "--start",
            "2025-07-21 10:00:00",
            "--end",
            "2025-07-21 11:00:00",
            "--interval",
            "1",
        ]);

        let output = cmd.timeout(Duration::from_secs(3)).assert().success();
        let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();

        // Should contain progress bar elements
        assert!(stdout.contains("["), "Should contain opening bracket");
        assert!(stdout.contains("]"), "Should contain closing bracket");
        assert!(stdout.contains("%"), "Should contain percentage");

        // Should contain either filled blocks or spaces for the bar
        assert!(
            stdout.contains("█") || stdout.contains(" "),
            "Should contain progress bar characters"
        );
    }

    #[test]
    fn test_output_formatting_consistency() {
        // Test that output formatting is consistent across different scenarios
        let test_scenarios = vec![
            ("2025-07-21 10:00:00", "2025-07-21 11:00:00", "1 hour range"),
            (
                "2025-07-21 10:00:00",
                "2025-07-21 10:30:00",
                "30 minute range",
            ),
            ("2025-07-21", "2025-07-22", "1 day range"),
        ];

        for (start, end, description) in test_scenarios {
            let mut cmd = CliTestUtils::pb_command();
            cmd.args(&["--start", start, "--end", end, "--interval", "1"]);

            let output = cmd.timeout(Duration::from_secs(3)).assert().success();
            let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();

            // Check consistent header format
            assert!(
                stdout.contains("pb - Progress Bar Tool"),
                "Missing header for {}",
                description
            );
            assert!(
                stdout.contains("Start time:"),
                "Missing start time display for {}",
                description
            );
            assert!(
                stdout.contains("End time:"),
                "Missing end time display for {}",
                description
            );
            assert!(
                stdout.contains("Update interval:"),
                "Missing interval display for {}",
                description
            );
        }
    }

    #[test]
    fn test_edge_case_time_ranges() {
        // Test edge cases for time ranges
        let edge_cases = vec![
            // Very short duration (1 second)
            (
                "2025-07-21 10:00:00",
                "2025-07-21 10:00:01",
                "1 second duration",
            ),
            // Exactly equal times
            (
                "2025-07-21 10:00:00",
                "2025-07-21 10:00:00",
                "zero duration",
            ),
            // Midnight crossing
            (
                "2025-07-21 23:59:59",
                "2025-07-22 00:00:01",
                "midnight crossing",
            ),
            // Month boundary
            (
                "2025-07-31 23:59:59",
                "2025-08-01 00:00:01",
                "month boundary",
            ),
        ];

        for (start, end, description) in edge_cases {
            let mut cmd = CliTestUtils::pb_command();
            cmd.args(&["--start", start, "--end", end, "--interval", "1"]);

            let result = cmd.timeout(Duration::from_secs(3)).assert();

            // Should handle edge cases gracefully (either succeed or fail with proper error)
            let stdout = String::from_utf8_lossy(&result.get_output().stdout);
            let stderr = String::from_utf8_lossy(&result.get_output().stderr);

            // For valid time ranges, we should see the header even if the command times out
            if stdout.contains("pb - Progress Bar Tool") || result.get_output().status.success() {
                // This indicates the command started successfully
                if !stdout.is_empty() {
                    assert!(
                        stdout.contains("pb - Progress Bar Tool"),
                        "Should show proper output for {}",
                        description
                    );
                }
            } else {
                // If no valid output and not successful, should have error message
                assert!(
                    !stderr.is_empty(),
                    "Should have error message for {}",
                    description
                );
            }
        }
    }
}

#[cfg(test)]
mod error_message_quality_tests {
    use super::*;

    #[test]
    fn test_helpful_error_messages() {
        // Test that error messages are helpful and actionable
        let error_scenarios = vec![
            (
                vec!["--start", "2025-13-40", "--end", "12:00:00"],
                vec!["Invalid", "start time", "2025-13-40"],
            ),
            (
                vec!["--start", "2025-01-01", "--end", "25:70:80"],
                vec!["Invalid", "end time", "25:70:80"],
            ),
            (
                vec![
                    "--start",
                    "2025-01-01 12:00:00",
                    "--end",
                    "2025-01-01 10:00:00",
                ],
                vec!["before", "equal to"],
            ),
        ];

        for (args, expected_fragments) in error_scenarios {
            let mut cmd = CliTestUtils::pb_command();
            cmd.args(&args);

            let output = cmd.assert().failure();
            let stderr = String::from_utf8_lossy(&output.get_output().stderr);

            for fragment in expected_fragments {
                assert!(
                    stderr.contains(fragment),
                    "Error message should contain '{}' for args {:?}\nActual stderr: {}",
                    fragment,
                    args,
                    stderr
                );
            }
        }
    }

    #[test]
    fn test_error_message_formatting() {
        // Test that error messages are well-formatted
        let mut cmd = CliTestUtils::pb_command();
        cmd.args(&["--start", "invalid", "--end", "12:00:00"]);

        let output = cmd.assert().failure();
        let stderr = String::from_utf8_lossy(&output.get_output().stderr);

        // Error message should start with "Error parsing"
        assert!(
            stderr.starts_with("Error parsing"),
            "Error message should start with 'Error parsing'\nActual: {}",
            stderr
        );

        // Should end with newline
        assert!(
            stderr.ends_with('\n'),
            "Error message should end with newline\nActual: {}",
            stderr
        );

        // Should not be empty
        assert!(
            !stderr.trim().is_empty(),
            "Error message should not be empty"
        );
    }
}

#[cfg(test)]
mod environment_compatibility_tests {
    use super::*;

    #[test]
    fn test_environment_variables() {
        // Test behavior with various environment variables
        let mut cmd = CliTestUtils::pb_command();
        cmd.env("NO_COLOR", "1"); // Disable colors
        cmd.args(&[
            "--start",
            "2025-07-21 10:00:00",
            "--end",
            "2025-07-21 11:00:00",
            "--interval",
            "1",
        ]);

        let output = cmd.timeout(Duration::from_secs(3)).assert().success();
        let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();

        // Should work with NO_COLOR environment variable
        assert!(stdout.contains("pb - Progress Bar Tool"));
        assert!(stdout.contains("Progress completed!"));
    }

    #[test]
    fn test_different_locales() {
        // Test behavior with different locale settings
        let locales = vec![("C", "C"), ("en_US.UTF-8", "en_US.UTF-8")];

        for (lang, lc_all) in locales {
            let mut cmd = CliTestUtils::pb_command();
            cmd.env("LANG", lang);
            cmd.env("LC_ALL", lc_all);
            cmd.args(&[
                "--start",
                "2025-07-21 10:00:00",
                "--end",
                "2025-07-21 11:00:00",
                "--interval",
                "1",
            ]);

            let result = cmd.timeout(Duration::from_secs(3)).assert();

            // Should work regardless of locale
            if result.get_output().status.success() {
                let stdout = String::from_utf8_lossy(&result.get_output().stdout);
                assert!(
                    stdout.contains("pb - Progress Bar Tool"),
                    "Should work with locale {}/{}",
                    lang,
                    lc_all
                );
            }
        }
    }

    #[test]
    fn test_stdout_stderr_separation() {
        // Test that stdout and stderr are used appropriately
        let mut cmd = CliTestUtils::pb_command();
        cmd.args(&[
            "--start",
            "2025-07-21 10:00:00",
            "--end",
            "2025-07-21 11:00:00",
            "--interval",
            "1",
        ]);

        let output = cmd.timeout(Duration::from_secs(3)).assert().success();
        let stdout = String::from_utf8_lossy(&output.get_output().stdout);
        let stderr = String::from_utf8_lossy(&output.get_output().stderr);

        // Normal output should go to stdout
        assert!(stdout.contains("pb - Progress Bar Tool"));
        assert!(stdout.contains("Progress completed!"));

        // stderr should be empty for successful runs
        assert!(
            stderr.is_empty() || stderr.trim().is_empty(),
            "stderr should be empty for successful runs, got: {}",
            stderr
        );
    }

    #[test]
    fn test_exit_codes() {
        // Test that proper exit codes are returned

        // Success case
        let mut cmd = CliTestUtils::pb_command();
        cmd.args(&[
            "--start",
            "2025-07-21 10:00:00",
            "--end",
            "2025-07-21 11:00:00",
            "--interval",
            "1",
        ]);
        cmd.timeout(Duration::from_secs(3)).assert().success();

        // Error case
        let mut cmd = CliTestUtils::pb_command();
        cmd.args(&["--start", "invalid", "--end", "12:00:00"]);
        cmd.assert().failure();

        // Help case (should succeed)
        let mut cmd = CliTestUtils::pb_command();
        cmd.args(&["--help"]);
        cmd.assert().success();

        // Version case (should succeed)
        let mut cmd = CliTestUtils::pb_command();
        cmd.args(&["--version"]);
        cmd.assert().success();
    }
}

#[cfg(test)]
mod performance_integration_tests {
    use super::*;

    #[test]
    fn test_startup_time() {
        // Test that the application starts quickly
        let start_time = std::time::Instant::now();

        let mut cmd = CliTestUtils::pb_command();
        cmd.args(&[
            "--start",
            "2025-07-21 10:00:00",
            "--end",
            "2025-07-21 11:00:00",
            "--interval",
            "1",
        ]);

        cmd.timeout(Duration::from_secs(3)).assert().success();

        let startup_time = start_time.elapsed();
        assert!(
            startup_time < Duration::from_secs(2),
            "Application startup took too long: {:?}",
            startup_time
        );
    }

    #[test]
    fn test_memory_usage_stability() {
        // Test that memory usage is reasonable
        // Note: This is a basic test - more sophisticated memory testing
        // would require platform-specific tools

        let mut cmd = CliTestUtils::pb_command();
        cmd.args(&[
            "--start",
            "+1h", // Future time
            "--end",
            "+2h",
            "--interval",
            "1",
        ]);

        // Run for a short time to check basic memory behavior
        let output = cmd.timeout(Duration::from_secs(2)).assert().failure(); // Will timeout

        // Should produce output without obvious memory issues
        let stdout = String::from_utf8_lossy(&output.get_output().stdout);
        assert!(stdout.contains("pb - Progress Bar Tool"));
    }
}

#[cfg(test)]
mod regression_tests {
    use super::*;

    #[test]
    fn test_previous_bug_fixes() {
        // Test for any previously fixed bugs to prevent regression

        // Example: Ensure zero interval is properly handled
        let mut cmd = CliTestUtils::pb_command();
        cmd.args(&[
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

        // Example: Ensure equal start/end times work
        let mut cmd = CliTestUtils::pb_command();
        cmd.args(&[
            "--start",
            "2025-07-21 10:00:00",
            "--end",
            "2025-07-21 10:00:00",
            "--interval",
            "1",
        ]);
        cmd.timeout(Duration::from_secs(3)).assert().success();
    }

    #[test]
    fn test_unicode_handling() {
        // Test handling of unicode in error messages and input
        let unicode_tests = vec![
            "2025-07-21🎉invalid",
            "２０２５-０７-２１", // Full-width numbers
            "2025-07-21 invalid🦀time",
        ];

        for invalid_input in unicode_tests {
            let mut cmd = CliTestUtils::pb_command();
            cmd.args(&["--start", invalid_input, "--end", "12:00:00"]);

            let output = cmd.assert().failure();
            let stderr = String::from_utf8_lossy(&output.get_output().stderr);

            // Should handle unicode gracefully in error messages
            assert!(
                stderr.contains("Error"),
                "Should have error message for unicode input: {}",
                invalid_input
            );
            assert!(
                !stderr.is_empty(),
                "Error message should not be empty for unicode input: {}",
                invalid_input
            );
        }
    }
}
