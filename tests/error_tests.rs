//! Comprehensive unit tests for error handling module
//!
//! This module tests all error types, error conversion, and error messaging
//! to ensure robust error handling throughout the application.

use anyhow::Context;
use pmon::error::*;
use std::error::Error;

mod common;

#[cfg(test)]
mod error_type_tests {
    use super::*;

    #[test]
    fn test_start_after_end_error() {
        let error = PbError::StartAfterEnd;

        // Test display message
        assert_eq!(
            error.to_string(),
            "Start time must be before or equal to end time"
        );

        // Test that it implements Error trait
        assert!(error.source().is_none()); // No underlying cause

        // Test debug formatting
        let debug_str = format!("{error:?}");
        assert!(debug_str.contains("StartAfterEnd"));
    }

    #[test]
    fn test_invalid_time_format_error() {
        let input = "invalid-time-format";
        let error = PbError::InvalidTimeFormat {
            input: input.to_string(),
        };

        // Test display message
        let expected_msg = format!("Invalid time format: {input}");
        assert_eq!(error.to_string(), expected_msg);

        // Test debug formatting
        let debug_str = format!("{error:?}");
        assert!(debug_str.contains("InvalidTimeFormat"));
        assert!(debug_str.contains(input));
    }

    #[test]
    fn test_end_time_already_passed_error() {
        let error = PbError::EndTimeAlreadyPassed;

        // Test display message
        assert_eq!(
            error.to_string(),
            "The specified end time has already passed"
        );

        // Test debug formatting
        let debug_str = format!("{error:?}");
        assert!(debug_str.contains("EndTimeAlreadyPassed"));
    }

    #[test]
    fn test_invalid_relative_time_format_error() {
        let input = "invalid-relative";
        let error = PbError::InvalidRelativeTimeFormat {
            input: input.to_string(),
        };

        // Test display message
        let expected_msg = format!("Invalid relative time format: {input}");
        assert_eq!(error.to_string(), expected_msg);

        // Test debug formatting
        let debug_str = format!("{error:?}");
        assert!(debug_str.contains("InvalidRelativeTimeFormat"));
        assert!(debug_str.contains(input));
    }

    #[test]
    fn test_missing_required_options_error() {
        let error = PbError::MissingRequiredOptions;

        // Test display message
        assert_eq!(error.to_string(), "--end option is required");

        // Test debug formatting
        let debug_str = format!("{error:?}");
        assert!(debug_str.contains("MissingRequiredOptions"));
    }
}

#[cfg(test)]
mod error_helper_function_tests {
    use super::*;

    #[test]
    fn test_invalid_time_format_helper() {
        let input = "test-input";
        let error = PbError::invalid_time_format(input);

        match error {
            PbError::InvalidTimeFormat {
                input: stored_input,
            } => {
                assert_eq!(stored_input, input);
            }
            _ => panic!("Expected InvalidTimeFormat variant"),
        }
    }

    #[test]
    fn test_invalid_time_format_helper_string_conversion() {
        // Test with &str
        let error = PbError::invalid_time_format("test");
        assert!(matches!(error, PbError::InvalidTimeFormat { .. }));

        // Test with String
        let error = PbError::invalid_time_format("test".to_string());
        assert!(matches!(error, PbError::InvalidTimeFormat { .. }));

        // Test with owned String
        let input = String::from("test");
        let error = PbError::invalid_time_format(input);
        assert!(matches!(error, PbError::InvalidTimeFormat { .. }));
    }

    #[test]
    fn test_invalid_relative_time_format_helper() {
        let input = "test-relative";
        let error = PbError::invalid_relative_time_format(input);

        match error {
            PbError::InvalidRelativeTimeFormat {
                input: stored_input,
            } => {
                assert_eq!(stored_input, input);
            }
            _ => panic!("Expected InvalidRelativeTimeFormat variant"),
        }
    }

    #[test]
    fn test_invalid_relative_time_format_helper_string_conversion() {
        // Test with &str
        let error = PbError::invalid_relative_time_format("test");
        assert!(matches!(error, PbError::InvalidRelativeTimeFormat { .. }));

        // Test with String
        let error = PbError::invalid_relative_time_format("test".to_string());
        assert!(matches!(error, PbError::InvalidRelativeTimeFormat { .. }));
    }
}

#[cfg(test)]
mod pb_result_tests {
    use super::*;

    #[test]
    fn test_pb_result_success() {
        let success: PbResult<i32> = Ok(42);
        assert!(success.is_ok());
        assert_eq!(success.unwrap(), 42);
    }

    #[test]
    fn test_pb_result_error() {
        let error: PbResult<i32> = Err(PbError::StartAfterEnd);
        assert!(error.is_err());

        let err = error.unwrap_err();
        assert_eq!(
            err.to_string(),
            "Start time must be before or equal to end time"
        );
    }

    #[test]
    fn test_pb_result_with_various_error_types() {
        let error_cases: Vec<PbResult<()>> = vec![
            Err(PbError::StartAfterEnd),
            Err(PbError::invalid_time_format("test")),
            Err(PbError::EndTimeAlreadyPassed),
            Err(PbError::invalid_relative_time_format("test")),
            Err(PbError::MissingRequiredOptions),
        ];

        for error_case in error_cases {
            assert!(error_case.is_err());
            let err = error_case.unwrap_err();
            assert!(!err.to_string().is_empty());
        }
    }

    #[test]
    fn test_pb_result_error_propagation() {
        // Test that PbResult works well with the ? operator
        fn test_function() -> PbResult<String> {
            let _value = test_helper()?;
            Ok("success".to_string())
        }

        fn test_helper() -> PbResult<i32> {
            Err(PbError::StartAfterEnd)
        }

        let result = test_function();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Start time must be before or equal to end time"
        );
    }
}

#[cfg(test)]
mod anyhow_integration_tests {
    use super::*;
    use anyhow::{Context, Result as AnyhowResult};

    #[test]
    fn test_pb_error_to_anyhow_conversion() {
        let pb_error = PbError::StartAfterEnd;
        let anyhow_error: anyhow::Error = pb_error.into();

        assert_eq!(
            anyhow_error.to_string(),
            "Start time must be before or equal to end time"
        );
    }

    #[test]
    fn test_pb_error_with_anyhow_context() {
        let result: AnyhowResult<()> =
            Err(PbError::invalid_time_format("test")).context("Failed to parse user input");

        assert!(result.is_err());
        let error = result.unwrap_err();
        let error_str = error.to_string();
        assert!(error_str.contains("Failed to parse user input"));

        // To check the inner error, we need to use the chain
        let has_invalid_time_format = error
            .chain()
            .any(|e| e.to_string().contains("Invalid time format: test"));
        assert!(has_invalid_time_format);
    }

    #[test]
    fn test_pb_result_to_anyhow_result() {
        fn pb_function() -> PbResult<i32> {
            Err(PbError::StartAfterEnd)
        }

        fn anyhow_function() -> AnyhowResult<i32> {
            let value = pb_function()?;
            Ok(value)
        }

        let result = anyhow_function();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Start time must be before or equal to end time"
        );
    }

    #[test]
    fn test_error_chain_with_anyhow() {
        use std::io;

        // Simulate an error chain: io::Error -> PbError -> anyhow::Error
        let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let pb_error = PbError::invalid_time_format("could not read time file");

        let result: AnyhowResult<()> = Err(anyhow::Error::new(pb_error))
            .context("Configuration loading failed")
            .context(anyhow::Error::new(io_error));

        assert!(result.is_err());
        let error = result.unwrap_err();
        let error_str = error.to_string();
        // Check that the outermost context appears in to_string()
        assert!(error_str.contains("file not found"));
    }
}

#[cfg(test)]
mod error_message_tests {
    use super::*;

    #[test]
    fn test_error_messages_are_user_friendly() {
        let error_cases = vec![
            (
                PbError::StartAfterEnd,
                "Start time must be before or equal to end time",
            ),
            (
                PbError::invalid_time_format("2025-13-40"),
                "Invalid time format: 2025-13-40",
            ),
            (
                PbError::EndTimeAlreadyPassed,
                "The specified end time has already passed",
            ),
            (
                PbError::invalid_relative_time_format("+1x"),
                "Invalid relative time format: +1x",
            ),
            (PbError::MissingRequiredOptions, "--end option is required"),
        ];

        for (error, expected_message) in error_cases {
            assert_eq!(error.to_string(), expected_message);

            // Ensure messages are helpful and not too technical
            let msg = error.to_string();
            assert!(!msg.is_empty());
            assert!(!msg.contains("Error")); // Don't redundantly say "Error"
                                             // Most messages start with capital, but some may start with special chars like "--"
            if !msg.starts_with("--") {
                assert!(msg.chars().next().unwrap().is_uppercase()); // Start with capital
            }
        }
    }

    #[test]
    fn test_error_messages_with_special_characters() {
        let special_inputs = vec![
            "input\nwith\nnewlines",
            "input\twith\ttabs",
            "input with spaces",
            "input\"with\"quotes",
            "input\\with\\backslashes",
            "input🎉with🎉emoji",
            "", // Empty string
        ];

        for input in special_inputs {
            let error = PbError::invalid_time_format(input);
            let message = error.to_string();

            // Message should contain the input (properly escaped/displayed)
            assert!(message.contains("Invalid time format:"));

            // Should not cause any formatting issues
            assert!(!message.is_empty());
        }
    }

    #[test]
    fn test_error_messages_length() {
        // Error messages should not be too long (user-friendly)
        let test_cases = vec![
            PbError::StartAfterEnd,
            PbError::invalid_time_format("test"),
            PbError::EndTimeAlreadyPassed,
            PbError::invalid_relative_time_format("test"),
            PbError::MissingRequiredOptions,
        ];

        for error in test_cases {
            let message = error.to_string();
            assert!(message.len() < 200, "Error message too long: {message}");
            assert!(message.len() > 10, "Error message too short: {message}");
        }
    }
}

#[cfg(test)]
mod error_trait_implementation_tests {
    use super::*;

    #[test]
    fn test_error_trait_implementation() {
        let error = PbError::StartAfterEnd;

        // Test that it implements std::error::Error
        let error_trait: &dyn std::error::Error = &error;
        assert_eq!(
            error_trait.to_string(),
            "Start time must be before or equal to end time"
        );

        // Test source() method (should return None for our simple errors)
        assert!(error_trait.source().is_none());
    }

    #[test]
    fn test_error_trait_for_all_variants() {
        let errors: Vec<PbError> = vec![
            PbError::StartAfterEnd,
            PbError::invalid_time_format("test"),
            PbError::EndTimeAlreadyPassed,
            PbError::invalid_relative_time_format("test"),
            PbError::MissingRequiredOptions,
        ];

        for error in errors {
            // Should implement Error trait
            let _: &dyn std::error::Error = &error;

            // Should implement Display
            let _: String = error.to_string();

            // Should implement Debug
            let _: String = format!("{error:?}");
        }
    }

    #[test]
    fn test_send_sync_traits() {
        // Test that PbError implements Send and Sync (important for multithreading)
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}

        assert_send::<PbError>();
        assert_sync::<PbError>();
    }
}

#[cfg(test)]
mod error_pattern_matching_tests {
    use super::*;

    #[test]
    fn test_error_pattern_matching() {
        let errors = vec![
            PbError::StartAfterEnd,
            PbError::invalid_time_format("test"),
            PbError::EndTimeAlreadyPassed,
            PbError::invalid_relative_time_format("test"),
            PbError::MissingRequiredOptions,
        ];

        for error in errors {
            match error {
                PbError::StartAfterEnd => {
                    assert_eq!(
                        error.to_string(),
                        "Start time must be before or equal to end time"
                    );
                }
                PbError::InvalidTimeFormat { input } => {
                    assert_eq!(input, "test");
                }
                PbError::EndTimeAlreadyPassed => {
                    assert_eq!(
                        error.to_string(),
                        "The specified end time has already passed"
                    );
                }
                PbError::InvalidRelativeTimeFormat { input } => {
                    assert_eq!(input, "test");
                }
                PbError::MissingRequiredOptions => {
                    assert_eq!(error.to_string(), "--end option is required");
                }
            }
        }
    }

    #[test]
    fn test_error_exhaustiveness() {
        // This test ensures that if we add new error variants, we update our tests
        let error = PbError::StartAfterEnd;

        let result = match error {
            PbError::StartAfterEnd => "start_after_end",
            PbError::InvalidTimeFormat { .. } => "invalid_time_format",
            PbError::EndTimeAlreadyPassed => "end_time_already_passed",
            PbError::InvalidRelativeTimeFormat { .. } => "invalid_relative_time_format",
            PbError::MissingRequiredOptions => "missing_required_options",
        };

        assert_eq!(result, "start_after_end");
    }
}

#[cfg(test)]
mod error_serialization_tests {
    use super::*;

    #[test]
    fn test_error_debug_format_stability() {
        // Test that debug format is stable and contains expected information
        let test_cases = vec![
            (PbError::StartAfterEnd, "StartAfterEnd"),
            (PbError::invalid_time_format("test"), "InvalidTimeFormat"),
            (PbError::EndTimeAlreadyPassed, "EndTimeAlreadyPassed"),
            (
                PbError::invalid_relative_time_format("test"),
                "InvalidRelativeTimeFormat",
            ),
            (PbError::MissingRequiredOptions, "MissingRequiredOptions"),
        ];

        for (error, expected_variant) in test_cases {
            let debug_str = format!("{error:?}");
            assert!(
                debug_str.contains(expected_variant),
                "Debug format should contain variant name: {debug_str}"
            );
        }
    }

    #[test]
    fn test_error_with_complex_input() {
        // Test errors with complex input strings
        let complex_inputs = vec![
            "a".repeat(1000), // Very long string
            "complex\ninput\nwith\nmultiple\nlines".to_string(),
            "input with unicode: 🦀🔧⚙️".to_string(),
            "input with null byte: \0".to_string(),
            "input with control chars: \x01\x02\x03".to_string(),
        ];

        for input in complex_inputs {
            let error = PbError::invalid_time_format(&input);

            // Should not panic or cause issues
            let _display = error.to_string();
            let _debug = format!("{error:?}");

            // Should preserve the input
            match error {
                PbError::InvalidTimeFormat {
                    input: stored_input,
                } => {
                    assert_eq!(stored_input, input);
                }
                _ => panic!("Expected InvalidTimeFormat variant"),
            }
        }
    }
}

#[cfg(test)]
mod error_usage_patterns_tests {
    use super::*;

    #[test]
    fn test_error_in_result_chains() {
        // Test common error handling patterns
        fn parse_and_validate() -> PbResult<String> {
            validate_input("invalid")?;
            Ok("success".to_string())
        }

        fn validate_input(input: &str) -> PbResult<()> {
            if input == "invalid" {
                return Err(PbError::invalid_time_format(input));
            }
            Ok(())
        }

        let result = parse_and_validate();
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert!(matches!(error, PbError::InvalidTimeFormat { .. }));
    }

    #[test]
    fn test_error_with_context_information() {
        // Test adding context to errors
        fn parse_config_file() -> anyhow::Result<String> {
            parse_time_setting("invalid")
                .context("Failed to parse configuration file")
                .context("Application initialization failed")?;
            Ok("success".to_string())
        }

        fn parse_time_setting(setting: &str) -> PbResult<String> {
            if setting == "invalid" {
                return Err(PbError::invalid_time_format(setting));
            }
            Ok(setting.to_string())
        }

        let result = parse_config_file();
        assert!(result.is_err());

        let error = result.unwrap_err();
        let error_msg = error.to_string();
        assert!(error_msg.contains("Application initialization failed"));

        // Check the error chain for deeper errors
        let has_config_parsing_error = error
            .chain()
            .any(|e| e.to_string().contains("Failed to parse configuration file"));
        assert!(has_config_parsing_error);

        let has_invalid_time_format = error
            .chain()
            .any(|e| e.to_string().contains("Invalid time format: invalid"));
        assert!(has_invalid_time_format);
    }

    #[test]
    fn test_error_recovery_patterns() {
        // Test error recovery patterns
        fn try_parse_with_fallback(input: &str) -> String {
            match try_parse(input) {
                Ok(result) => result,
                Err(PbError::InvalidTimeFormat { .. }) => {
                    // Try a different parsing strategy
                    try_parse_alternative(input).unwrap_or_else(|_| "default".to_string())
                }
                Err(_) => "default".to_string(),
            }
        }

        fn try_parse(input: &str) -> PbResult<String> {
            if input == "invalid" {
                return Err(PbError::invalid_time_format(input));
            }
            Ok(input.to_string())
        }

        fn try_parse_alternative(input: &str) -> PbResult<String> {
            if input == "invalid" {
                return Err(PbError::invalid_relative_time_format(input));
            }
            Ok(input.to_string())
        }

        let result = try_parse_with_fallback("invalid");
        assert_eq!(result, "default");

        let result = try_parse_with_fallback("valid");
        assert_eq!(result, "valid");
    }
}
