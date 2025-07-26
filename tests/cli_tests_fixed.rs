//! Comprehensive unit tests for CLI module
//!
//! This module tests command-line argument parsing, validation,
//! error handling, and help generation.

use clap::{CommandFactory, Parser};
use pb::cli::*;

mod common;

#[cfg(test)]
mod cli_parsing_tests {
    use super::*;

    #[test]
    fn test_basic_arguments_parsing() {
        let test_cases = vec![
            (vec!["pb", "--start", "10:00", "--end", "12:00"], true),
            (vec!["pb", "-s", "10:00", "-e", "12:00"], true),
            (
                vec![
                    "pb",
                    "--start",
                    "2023-12-25 10:00:00",
                    "--end",
                    "2023-12-25 12:00:00",
                ],
                true,
            ),
        ];

        for (args, should_pass) in test_cases {
            let result = Cli::try_parse_from(args.clone());

            if should_pass {
                let cli = Cli::try_parse_from(args.clone()).unwrap();
                assert_eq!(cli.start, args[2]);
                assert_eq!(cli.end, args[4]);
                assert_eq!(cli.interval, 60); // default value
            } else {
                assert!(result.is_err(), "Expected parsing to fail for: {:?}", args);
            }
        }
    }

    #[test]
    fn test_interval_argument_parsing() {
        let test_cases = vec![
            (
                vec![
                    "pb",
                    "--start",
                    "10:00",
                    "--end",
                    "12:00",
                    "--interval",
                    "30",
                ],
                30,
            ),
            (vec!["pb", "-s", "10:00", "-e", "12:00", "-i", "120"], 120),
            (vec!["pb", "--start", "10:00", "--end", "12:00"], 60), // default
        ];

        for (args, expected_interval) in test_cases {
            let cli = Cli::try_parse_from(args).unwrap();
            assert_eq!(cli.interval, expected_interval);
        }
    }

    #[test]
    fn test_required_arguments_missing() {
        let invalid_args = vec![
            vec!["pb"],                     // no arguments
            vec!["pb", "--start", "10:00"], // missing end
            vec!["pb", "--end", "12:00"],   // missing start
        ];

        for args in invalid_args {
            let result = Cli::try_parse_from(args.clone());
            assert!(result.is_err(), "Expected parsing to fail for: {:?}", args);
        }
    }
}

#[cfg(test)]
mod cli_validation_tests {
    use super::*;

    #[test]
    fn test_basic_validation_success() {
        // Test basic validation through parse_args
        // Note: we can't test the private validate() method directly
        let basic_cases = vec![
            ("10:00", "12:00"),
            ("2023-12-25 10:00:00", "2023-12-25 12:00:00"),
            ("+1h", "+3h"),
        ];

        for (start, end) in basic_cases {
            let cli = Cli::try_parse_from(vec!["pb", "--start", start, "--end", end]).unwrap();
            // Since validate() is private, we just check the fields are set
            assert!(!cli.start().is_empty());
            assert!(!cli.end().is_empty());
            assert!(cli.interval() > 0);
        }
    }

    #[test]
    fn test_empty_string_detection() {
        // Test that empty strings are handled appropriately
        let cli = Cli::try_parse_from(vec!["pb", "--start", "", "--end", "12:00"]).unwrap();
        // We can't call validate() directly since it's private
        // But we can check that empty strings are present
        assert_eq!(cli.start(), "");
        assert_eq!(cli.end(), "12:00");
    }

    #[test]
    fn test_zero_interval_handling() {
        let cli = Cli::try_parse_from(vec![
            "pb",
            "--start",
            "10:00",
            "--end",
            "12:00",
            "--interval",
            "0",
        ])
        .unwrap();
        // Check that zero interval is parsed
        assert_eq!(cli.interval(), 0);
    }
}

#[cfg(test)]
mod comprehensive_cli_tests {
    use super::*;

    #[test]
    fn test_comprehensive_argument_combinations() {
        let test_cases = vec![
            // Valid combinations
            (
                vec!["pb", "--start", "10:00", "--end", "12:00"],
                true,
                "Basic time format",
            ),
            (
                vec!["pb", "-s", "2023-12-25", "-e", "2023-12-26"],
                true,
                "Short flags with dates",
            ),
            (
                vec!["pb", "--start", "+1h", "--end", "+3h", "--interval", "30"],
                true,
                "Relative times",
            ),
            // Edge cases that should parse but might fail validation
            (
                vec![
                    "pb",
                    "--start",
                    "10:00",
                    "--end",
                    "12:00",
                    "--interval",
                    "1",
                ],
                true,
                "Minimum interval",
            ),
            (
                vec!["pb", "--start", "invalid", "--end", "12:00"],
                true,
                "Invalid time format - parsing only",
            ),
        ];

        for (args, should_parse, description) in test_cases {
            let result = Cli::try_parse_from(args.clone());

            if should_parse {
                assert!(
                    result.is_ok(),
                    "Parsing should succeed for {}: {:?}",
                    description,
                    args
                );
            } else {
                assert!(
                    result.is_err(),
                    "Parsing should fail for {}: {:?}",
                    description,
                    args
                );
            }
        }
    }
}

#[cfg(test)]
mod clap_integration_tests {
    use super::*;

    #[test]
    fn test_help_generation() {
        // Test that help can be generated without panicking
        let mut cmd = Cli::command();
        let help = cmd.render_help();
        assert!(help.to_string().contains("CLI progress bar tool"));
    }

    #[test]
    fn test_version_information() {
        let cmd = Cli::command();
        let version = cmd.get_version();
        assert!(version.is_some());
    }

    #[test]
    fn test_argument_metadata() {
        let cmd = Cli::command();

        // Check that required arguments are defined
        let args: Vec<_> = cmd.get_arguments().collect();
        let arg_names: Vec<&str> = args.iter().map(|arg| arg.get_id().as_str()).collect();

        assert!(arg_names.contains(&"start"));
        assert!(arg_names.contains(&"end"));
        assert!(arg_names.contains(&"interval"));
    }
}

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    #[test]
    fn test_invalid_argument_handling() {
        let test_cases = vec![
            vec!["pb", "--invalid-flag"],
            vec!["pb", "--start", "10:00", "--end", "12:00", "--unknown"],
        ];

        for args in test_cases {
            let result = Cli::try_parse_from(args.clone());
            assert!(result.is_err(), "Should fail for invalid args: {:?}", args);
        }
    }

    #[test]
    fn test_missing_values() {
        let test_cases = vec![
            vec!["pb", "--start"],
            vec!["pb", "--end"],
            vec!["pb", "--start", "10:00", "--end"],
        ];

        for args in test_cases {
            let result = Cli::try_parse_from(args.clone());
            assert!(
                result.is_err(),
                "Should fail for incomplete args: {:?}",
                args
            );
        }
    }
}

#[cfg(test)]
mod cli_field_access_tests {
    use super::*;

    #[test]
    fn test_field_accessor_methods() {
        let cli = Cli::try_parse_from(vec![
            "pb",
            "--start",
            "10:00",
            "--end",
            "12:00",
            "--interval",
            "30",
        ])
        .unwrap();

        assert_eq!(cli.start(), "10:00");
        assert_eq!(cli.end(), "12:00");
        assert_eq!(cli.interval(), 30);
    }

    #[test]
    fn test_default_interval_value() {
        let cli = Cli::try_parse_from(vec!["pb", "--start", "10:00", "--end", "12:00"]).unwrap();
        assert_eq!(cli.interval(), 60);
    }
}
