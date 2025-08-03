//! CLI argument parsing for pb tool
//!
//! This module provides command-line argument parsing using `clap` derive API.
//! It handles required and optional arguments, validation, and help generation.

use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, TimeZone, Timelike};
use clap::Parser;

/// CLI progress monitor tool for time-based visualization
#[derive(Parser, Debug)]
#[command(name = "pmon")]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(
        short,
        long,
        value_parser = parse_start_time,
        default_value_t = Local::now().with_nanosecond(0).unwrap(),
        help = "Start time")]
    pub start: DateTime<Local>,
    #[arg(
        short,
        long,
        value_parser = parse_end_time,
        help = "End time")]
    pub end: DateTime<Local>,
    #[arg(
        short,
        long,
        default_value = "1",
        value_parser = clap::value_parser!(u64).range(1..60),
        help = "Update interval in seconds")]
    pub interval: u64,
    #[arg(
        short,
        long,
        default_value = "false",
        help = "Display verbose output with header information"
    )]
    pub verbose: bool,
}

fn parse_start_time(s: &str) -> Result<DateTime<Local>, String> {
    if let Ok(datetime) = parse_datetime_as_ymd_hmsz(s) {
        return Ok(datetime);
    }
    if let Ok(datetime) = parse_datetime_as_ymd_hms(s) {
        return Ok(datetime);
    }
    if let Ok(datetime) = parse_datetime_as_ymd_hm(s) {
        return Ok(datetime);
    }
    if let Ok(date) = parse_date(s) {
        let naive_dt = date.and_hms_opt(0, 0, 0).unwrap();
        return Ok(TimeZone::from_utc_datetime(&Local, &naive_dt));
    }
    Err(format!("Invalid start time format: {}", s))
}

fn parse_end_time(s: &str) -> Result<DateTime<Local>, String> {
    if let Ok(datetime) = parse_datetime_as_ymd_hmsz(s) {
        return Ok(datetime);
    }
    if let Ok(datetime) = parse_datetime_as_ymd_hms(s) {
        return Ok(datetime);
    }
    if let Ok(datetime) = parse_datetime_as_ymd_hm(s) {
        return Ok(datetime.with_second(59).unwrap());
    }
    if let Ok(date) = parse_date(s) {
        let datetime = date.and_hms_opt(23, 59, 59).unwrap();
        return Ok(TimeZone::from_utc_datetime(&Local, &datetime));
    }
    Err(format!("Invalid end time format: {}", s))
}

#[warn(non_snake_case)]
fn parse_datetime_as_ymd_hmsz(s: &str) -> Result<DateTime<Local>, String> {
    let formats = ["%Y-%m-%dT%H:%M:%S%z", "%Y-%m-%d %H:%M:%S%z"];
    for format in &formats {
        if let Ok(naive_datetime) = NaiveDateTime::parse_from_str(s, format) {
            return Ok(TimeZone::from_utc_datetime(&Local, &naive_datetime));
        }
    }
    Err(format!("Invalid datetime format: {}", s))
}

fn parse_datetime_as_ymd_hms(s: &str) -> Result<DateTime<Local>, String> {
    let formats = ["%Y-%m-%dT%H:%M:%S", "%Y-%m-%d %H:%M:%S", "%Y%m%d%H%M%S"];
    for format in &formats {
        if let Ok(naive_datetime) = NaiveDateTime::parse_from_str(s, format) {
            return Ok(TimeZone::from_utc_datetime(&Local, &naive_datetime));
        }
    }
    Err(format!("Invalid datetime format: {}", s))
}

fn parse_datetime_as_ymd_hm(s: &str) -> Result<DateTime<Local>, String> {
    let formats = ["%Y-%m-%dT%H:%M", "%Y-%m-%d %H:%M", "%Y%m%d%H%M"];
    for format in &formats {
        if let Ok(naive_datetime) = NaiveDateTime::parse_from_str(s, format) {
            return Ok(TimeZone::from_utc_datetime(&Local, &naive_datetime));
        }
    }
    Err(format!("Invalid datetime format: {}", s))
}

fn parse_date(s: &str) -> Result<NaiveDate, String> {
    let date_formats = ["%Y-%m-%d", "%Y%m%d"];
    for format in &date_formats {
        if let Ok(date) = chrono::NaiveDate::parse_from_str(s, format) {
            return Ok(date);
        }
    }
    Err(format!("Invalid date format: {}", s))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_with_start() {
        let args = vec![
            "pmon",
            "--start",
            "2025-01-01 10:20:30",
            "--end",
            "2025-01-31 23:59:59",
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        assert_eq!(
            cli.start.format("%Y-%m-%d %H:%M:%S").to_string(),
            "2025-01-01 10:20:30"
        );
    }

    #[test]
    fn test_parse_without_start() {
        let args = vec!["pmon", "--end", "2025-12-31"];
        let cli = Cli::try_parse_from(args).unwrap();

        // Get current time and allow small tolerance for execution time
        let now = Local::now().with_nanosecond(0).unwrap();
        let time_diff = (cli.start - now).num_seconds().abs();

        // Assert that the start time is within 1 second of current time
        assert!(
            time_diff <= 1,
            "Start time should be current time, but was {} seconds off",
            time_diff
        );
    }

    #[test]
    fn test_parse_with_end() {
        let args = vec![
            "pmon",
            "--start",
            "2025-01-01 10:20:30",
            "--end",
            "2025-01-31 23:59:59",
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        assert_eq!(
            cli.end.format("%Y-%m-%d %H:%M:%S").to_string(),
            "2025-01-31 23:59:59"
        );
    }

    #[test]
    fn test_parse_with_interval() {
        let args = vec![
            "pmon",
            "--start",
            "2025-01-01 10:20:30",
            "--end",
            "2025-01-31 23:59:59",
            "--interval",
            "5",
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        assert_eq!(cli.interval, 5);
    }

    #[test]
    fn test_parse_without_interval() {
        let args = vec![
            "pmon",
            "--start",
            "2025-01-01 10:20:30",
            "--end",
            "2025-01-31 23:59:59",
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        assert_eq!(cli.interval, 1);
    }

    #[test]
    fn test_parse_with_verbose() {
        let args = vec![
            "pmon",
            "--start",
            "2025-01-01 10:20:30",
            "--end",
            "2025-01-31 23:59:59",
            "--verbose",
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        assert_eq!(cli.verbose, true);
    }

    #[test]
    fn test_parse_without_verbose() {
        let args = vec![
            "pmon",
            "--start",
            "2025-01-01 10:20:30",
            "--end",
            "2025-01-31 23:59:59",
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        assert_eq!(cli.verbose, false);
    }

    #[test]
    fn test_parse_start_time() {
        assert_eq!(
            parse_start_time("2025-10-01 01:02")
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            "2025-10-01 01:02:00"
        );
        assert_eq!(
            parse_start_time("2025-10-01 01:02:03")
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            "2025-10-01 01:02:03"
        );
        assert_eq!(
            parse_start_time("2025-10-01T01:02:03+00:00")
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            "2025-10-01 01:02:03"
        );
        assert_eq!(
            parse_start_time("2025-10-01T01:02:03+09:00")
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            "2025-10-01 01:02:03"
        );
        assert_eq!(
            parse_start_time("20251001010203")
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            "2025-10-01 01:02:03"
        );
        assert_eq!(
            parse_start_time("202510010102")
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            "2025-10-01 01:02:00"
        );
        assert_eq!(
            parse_start_time("20251001")
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            "2025-10-01 00:00:00"
        );
        assert_eq!(
            parse_start_time("2025-10-01")
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            "2025-10-01 00:00:00"
        );
    }

    #[test]
    fn test_parse_end_time() {
        assert_eq!(
            parse_end_time("2025-10-01 01:02")
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            "2025-10-01 01:02:59"
        );
        assert_eq!(
            parse_end_time("2025-10-01 01:02:03")
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            "2025-10-01 01:02:03"
        );
        assert_eq!(
            parse_end_time("2025-10-01T01:02:03+00:00")
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            "2025-10-01 01:02:03"
        );
        assert_eq!(
            parse_end_time("2025-10-01T01:02:03+09:00")
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            "2025-10-01 01:02:03"
        );
        assert_eq!(
            parse_end_time("20251001010203")
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            "2025-10-01 01:02:03"
        );
        assert_eq!(
            // TODO: Fix this test to match the expected behavior
            parse_end_time("202510010102")
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            "2025-10-01 01:02:59"
        );
        assert_eq!(
            parse_end_time("20251001")
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            "2025-10-01 23:59:59"
        );
        assert_eq!(
            parse_end_time("2025-10-01")
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            "2025-10-01 23:59:59"
        );
    }
}
