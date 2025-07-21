use assert_cmd::Command;
use predicates::prelude::*;
use std::time::Duration;

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
        .stdout(predicate::str::contains("pb 0.1.0"));
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
    cmd.args(&["--start", "2025-07-21 12:00:00", "--end", "2025-07-21 10:00:00"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("must be before or equal to"));
}

#[test]
fn test_completed_progress() {
    let mut cmd = Command::cargo_bin("pb").unwrap();
    cmd.args(&[
        "--start", "2025-07-21 10:00:00",
        "--end", "2025-07-21 11:00:00",
        "--interval", "1"
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
        "--start", "2025-07-20",  // Yesterday
        "--end", "2025-07-21",    // Today  
        "--interval", "1"
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
    cmd.args(&[
        "--start", "+30m", 
        "--end", "+60m",
        "--interval", "1"
    ]);

    // This should not complete immediately since it's a future time range
    let output = cmd.timeout(Duration::from_secs(3)).assert().failure(); // Will timeout, which is expected
    
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    assert!(stdout.contains("pb - Progress Bar Tool"));
    assert!(stdout.contains("[                                        ] 0%"));
}

#[test]
fn test_custom_interval() {
    let mut cmd = Command::cargo_bin("pb").unwrap();
    cmd.args(&[
        "--start", "2025-07-21 10:00:00",
        "--end", "2025-07-21 11:00:00",
        "--interval", "5"
    ]);

    let output = cmd.timeout(Duration::from_secs(5)).assert().success();
    
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    assert!(stdout.contains("Update interval: 5 seconds"));
}

#[test]
fn test_progress_bar_overtime() {
    let mut cmd = Command::cargo_bin("pb").unwrap();
    cmd.args(&[
        "--start", "2025-07-21 09:00:00",
        "--end", "2025-07-21 10:00:00",
        "--interval", "1"
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
        "--start", "2025-07-21 10:00:00",
        "--end", "2025-07-21 11:00:00",
        "--interval", "0"
    ]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("must be greater than 0"));
}

#[test]
fn test_equal_start_end_times() {
    let mut cmd = Command::cargo_bin("pb").unwrap();
    cmd.args(&[
        "--start", "2025-07-21 10:00:00",
        "--end", "2025-07-21 10:00:00",
        "--interval", "1"
    ]);

    let output = cmd.timeout(Duration::from_secs(5)).assert().success();
    
    let stdout = String::from_utf8(output.get_output().stdout.clone()).unwrap();
    // Should immediately show 100% and complete
    assert!(stdout.contains("Progress completed!"));
}
