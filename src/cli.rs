//! CLI argument parsing for pb tool
//!
//! This module provides command-line argument parsing using `clap` derive API.
//! It handles required and optional arguments, validation, and help generation.

use crate::error::{PbError, PbResult};
use clap::Parser;

/// CLI progress bar tool for time-based visualization
#[derive(Parser, Debug)]
#[command(name = "pb")]
#[command(about = "A CLI progress bar tool for time-based visualization")]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    /// Start time (e.g., "2023-12-01 10:00:00", "10:00", "+1h")
    #[arg(short, long, help = "Start time")]
    pub start: Option<String>,

    /// End time (e.g., "2023-12-01 12:00:00", "12:00", "+3h")
    #[arg(short, long, help = "End time")]
    pub end: String,

    /// Update interval in seconds
    #[arg(short, long, default_value = "60", help = "Update interval in seconds")]
    pub interval: u64,

    /// Display verbose output including header information
    #[arg(
        short,
        long,
        default_value = "false",
        help = "Display verbose output with header information"
    )]
    pub verbose: bool,
}

impl Cli {
    /// Parse command line arguments
    ///
    /// This method parses command line arguments and validates them.
    /// Returns a `PbResult<Cli>` which can be an error if parsing fails.
    pub fn parse_args() -> PbResult<Self> {
        let cli = Self::try_parse().map_err(|e| {
            // Handle clap errors and convert to our error types
            match e.kind() {
                clap::error::ErrorKind::DisplayHelp | clap::error::ErrorKind::DisplayVersion => {
                    // For help and version, print the message and exit successfully
                    println!("{e}");
                    std::process::exit(0);
                }
                _ => {
                    // For other clap errors, create an InvalidTimeFormat error
                    // This is a fallback - in practice, most validation will be done elsewhere
                    PbError::invalid_time_format(format!("CLI parsing error: {e}"))
                }
            }
        })?;

        cli.validate()?;
        Ok(cli)
    }

    /// Validate the parsed arguments
    ///
    /// Performs basic validation on the parsed arguments.
    /// More detailed time parsing validation will be handled by the time_parser module.
    pub fn validate(&self) -> PbResult<()> {
        // Basic validation - more detailed validation will be in time_parser
        if let Some(start) = &self.start {
            if start.trim().is_empty() {
                return Err(PbError::invalid_time_format("Start time cannot be empty"));
            }
        }

        if self.end.trim().is_empty() {
            return Err(PbError::invalid_time_format("End time cannot be empty"));
        }

        if self.interval == 0 {
            return Err(PbError::invalid_time_format(
                "Interval must be greater than 0",
            ));
        }

        Ok(())
    }

    /// Get start time as string
    pub fn start(&self) -> Option<&str> {
        self.start.as_deref()
    }

    /// Get end time as string
    pub fn end(&self) -> &str {
        &self.end
    }

    /// Get interval in seconds
    pub fn interval(&self) -> u64 {
        self.interval
    }

    /// Get verbose flag
    pub fn verbose(&self) -> bool {
        self.verbose
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_cli_structure() {
        // Test that the CLI structure is valid
        Cli::command().debug_assert();
    }

    #[test]
    fn test_parse_valid_args() {
        // Test parsing valid arguments
        let args = vec!["pb", "--start", "10:00", "--end", "12:00"];
        let cli = Cli::try_parse_from(args).unwrap();

        assert_eq!(cli.start(), Some("10:00"));
        assert_eq!(cli.end(), "12:00");
        assert_eq!(cli.interval(), 60); // default value
    }

    #[test]
    fn test_parse_with_interval() {
        // Test parsing with custom interval
        let args = vec!["pb", "-s", "10:00", "-e", "12:00", "-i", "30"];
        let cli = Cli::try_parse_from(args).unwrap();

        assert_eq!(cli.start(), Some("10:00"));
        assert_eq!(cli.end(), "12:00");
        assert_eq!(cli.interval(), 30);
    }

    #[test]
    fn test_parse_long_form() {
        // Test parsing with long form arguments
        let args = vec![
            "pb",
            "--start",
            "2023-12-01 10:00:00",
            "--end",
            "2023-12-01 12:00:00",
            "--interval",
            "120",
        ];
        let cli = Cli::try_parse_from(args).unwrap();

        assert_eq!(cli.start(), Some("2023-12-01 10:00:00"));
        assert_eq!(cli.end(), "2023-12-01 12:00:00");
        assert_eq!(cli.interval(), 120);
    }

    #[test]
    fn test_missing_required_args() {
        // Test that missing required arguments are handled
        let args = vec!["pb"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_err()); // --end is still required

        // --start is now optional, so this should succeed
        let args = vec!["pb", "--end", "12:00"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_ok());

        let cli = result.unwrap();
        assert_eq!(cli.start(), None); // start should be None when not provided
        assert_eq!(cli.end(), "12:00");
    }

    #[test]
    fn test_parse_args_validation() {
        // Test the parse_args method with validation

        // Mock command line args for testing
        // In real usage, this would use std::env::args()
        let test_cases = vec![
            (vec!["pb", "--start", "10:00", "--end", "12:00"], true),
            (vec!["pb", "-s", "10:00", "-e", "12:00", "-i", "30"], true),
        ];

        for (args, should_succeed) in test_cases {
            let result = Cli::try_parse_from(args);
            if should_succeed {
                assert!(result.is_ok(), "Expected parsing to succeed");
                if let Ok(cli) = result {
                    assert!(cli.validate().is_ok(), "Expected validation to succeed");
                }
            } else {
                assert!(result.is_err(), "Expected parsing to fail");
            }
        }
    }

    #[test]
    fn test_validation_empty_strings() {
        // Test validation with empty strings
        let args = vec!["pb", "--start", "", "--end", "12:00"];
        let cli = Cli::try_parse_from(args).unwrap();
        assert!(cli.validate().is_err());

        let args = vec!["pb", "--start", "10:00", "--end", ""];
        let cli = Cli::try_parse_from(args).unwrap();
        assert!(cli.validate().is_err());
    }

    #[test]
    fn test_validation_zero_interval() {
        // Test validation with zero interval
        let args = vec![
            "pb",
            "--start",
            "10:00",
            "--end",
            "12:00",
            "--interval",
            "0",
        ];
        let cli = Cli::try_parse_from(args).unwrap();
        assert!(cli.validate().is_err());
    }

    #[test]
    fn test_help_generation() {
        // Test that help can be generated
        let mut cmd = Cli::command();
        let help = cmd.render_help();
        let help_str = help.to_string();

        assert!(help_str.contains("A CLI progress bar tool for time-based visualization"));
        assert!(help_str.contains("Start time"));
        assert!(help_str.contains("End time"));
        assert!(help_str.contains("Update interval in seconds"));
        assert!(help_str.contains("-s, --start"));
        assert!(help_str.contains("-e, --end"));
        assert!(help_str.contains("-i, --interval"));
    }

    #[test]
    fn test_debug_output() {
        // Test that the debug output is reasonable
        let args = vec!["pb", "--start", "10:00", "--end", "12:00"];
        let cli = Cli::try_parse_from(args).unwrap();
        let debug_str = format!("{cli:?}");

        assert!(debug_str.contains("start: Some(\"10:00\")"));
        assert!(debug_str.contains("end: \"12:00\""));
        assert!(debug_str.contains("interval: 60"));
    }

    #[test]
    fn test_getters() {
        // Test getter methods
        let args = vec![
            "pb",
            "--start",
            "10:00",
            "--end",
            "12:00",
            "--interval",
            "30",
        ];
        let cli = Cli::try_parse_from(args).unwrap();

        assert_eq!(cli.start(), Some("10:00"));
        assert_eq!(cli.end(), "12:00");
        assert_eq!(cli.interval(), 30);
    }
}
