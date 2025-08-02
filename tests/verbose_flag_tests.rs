use assert_cmd::Command;
use std::time::Duration;

#[test]
fn test_verbose_flag_shows_header() {
    let mut cmd = Command::cargo_bin("pb").unwrap();
    cmd.args([
        "--start",
        "2025-07-21 10:00:00",
        "--end",
        "2025-07-21 11:00:00",
        "--interval",
        "1",
        "--verbose",
    ]);

    let output = cmd.timeout(Duration::from_secs(3)).assert().success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();

    // With verbose flag, should show header
    assert!(stdout.contains("pb - Progress Bar Tool"));
    assert!(stdout.contains("Start time: 2025-07-21 10:00:00"));
    assert!(stdout.contains("End time: 2025-07-21 11:00:00"));
    assert!(stdout.contains("Update interval: 1 seconds"));
    assert!(stdout.contains("Press Ctrl+C to exit"));
}

#[test]
fn test_verbose_flag_short_form() {
    let mut cmd = Command::cargo_bin("pb").unwrap();
    cmd.args([
        "--start",
        "2025-07-21 10:00:00",
        "--end",
        "2025-07-21 11:00:00",
        "--interval",
        "1",
        "-v",
    ]);

    let output = cmd.timeout(Duration::from_secs(3)).assert().success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();

    // With -v flag, should show header
    assert!(stdout.contains("pb - Progress Bar Tool"));
    assert!(stdout.contains("Start time: 2025-07-21 10:00:00"));
    assert!(stdout.contains("End time: 2025-07-21 11:00:00"));
    assert!(stdout.contains("Update interval: 1 seconds"));
    assert!(stdout.contains("Press Ctrl+C to exit"));
}

#[test]
fn test_default_behavior_no_header() {
    let mut cmd = Command::cargo_bin("pb").unwrap();
    cmd.args([
        "--start",
        "2025-07-21 10:00:00",
        "--end",
        "2025-07-21 11:00:00",
        "--interval",
        "1",
    ]);

    let output = cmd.timeout(Duration::from_secs(3)).assert().success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();

    // Without verbose flag, should NOT show header
    assert!(!stdout.contains("pb - Progress Bar Tool"));
    assert!(!stdout.contains("Start time:"));
    assert!(!stdout.contains("End time:"));
    assert!(!stdout.contains("Update interval:"));
    assert!(!stdout.contains("Press Ctrl+C to exit"));

    // But should still show progress bar
    assert!(stdout.contains("["));
    assert!(stdout.contains("]"));
    assert!(stdout.contains("%"));
}

#[test]
fn test_verbose_flag_with_different_intervals() {
    let mut cmd = Command::cargo_bin("pb").unwrap();
    cmd.args([
        "--start",
        "2025-07-21 10:00:00",
        "--end",
        "2025-07-21 11:00:00",
        "--interval",
        "30",
        "--verbose",
    ]);

    let output = cmd.timeout(Duration::from_secs(3)).assert().success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();

    // Should show correct interval in header
    assert!(stdout.contains("Update interval: 30 seconds"));
}

#[test]
fn test_help_includes_verbose_flag() {
    let mut cmd = Command::cargo_bin("pb").unwrap();
    cmd.arg("--help");

    let output = cmd.assert().success();
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();

    // Should show verbose flag in help
    assert!(stdout.contains("-v, --verbose"));
    assert!(stdout.contains("Display verbose output with header information"));
}
