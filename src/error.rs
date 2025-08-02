//! Error handling system for pb CLI tool
//!
//! This module provides comprehensive error types using `thiserror` for custom error definitions
//! and integrates with `anyhow` for error propagation throughout the application.

use thiserror::Error;

/// Custom error types for the pb CLI tool
///
/// These errors cover all the main failure modes that can occur during
/// time parsing, validation, and progress bar operation.
#[derive(Error, Debug)]
pub enum PbError {
    /// Start time must be before or equal to end time
    #[error("Start time must be before or equal to end time")]
    StartAfterEnd,

    /// Invalid time format with the problematic input
    #[error("Invalid time format: {input}")]
    InvalidTimeFormat { input: String },

    /// The specified end time has already passed
    #[error("The specified end time has already passed")]
    EndTimeAlreadyPassed,

    /// Invalid relative time format with the problematic input
    #[error("Invalid relative time format: {input}")]
    InvalidRelativeTimeFormat { input: String },

    /// Required CLI options are missing (only --end is required now)
    #[error("--end option is required")]
    MissingRequiredOptions,
}

/// Result type alias for operations that can fail with a PbError
pub type PbResult<T> = Result<T, PbError>;

impl PbError {
    /// Create an InvalidTimeFormat error with the given input
    pub fn invalid_time_format(input: impl Into<String>) -> Self {
        Self::InvalidTimeFormat {
            input: input.into(),
        }
    }

    /// Create an InvalidRelativeTimeFormat error with the given input
    pub fn invalid_relative_time_format(input: impl Into<String>) -> Self {
        Self::InvalidRelativeTimeFormat {
            input: input.into(),
        }
    }
}

// Note: anyhow automatically provides From<PbError> for anyhow::Error
// since PbError implements std::error::Error through thiserror::Error

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_after_end_error_message() {
        let error = PbError::StartAfterEnd;
        assert_eq!(
            error.to_string(),
            "Start time must be before or equal to end time"
        );
    }

    #[test]
    fn test_invalid_time_format_error_message() {
        let input = "invalid-time";
        let error = PbError::invalid_time_format(input);
        assert_eq!(error.to_string(), "Invalid time format: invalid-time");
    }

    #[test]
    fn test_end_time_already_passed_error_message() {
        let error = PbError::EndTimeAlreadyPassed;
        assert_eq!(
            error.to_string(),
            "The specified end time has already passed"
        );
    }

    #[test]
    fn test_invalid_relative_time_format_error_message() {
        let input = "invalid-relative";
        let error = PbError::invalid_relative_time_format(input);
        assert_eq!(
            error.to_string(),
            "Invalid relative time format: invalid-relative"
        );
    }

    #[test]
    fn test_missing_required_options_error_message() {
        let error = PbError::MissingRequiredOptions;
        assert_eq!(error.to_string(), "--end option is required");
    }

    #[test]
    fn test_error_conversion_to_anyhow() {
        let pb_error = PbError::StartAfterEnd;
        let anyhow_error: anyhow::Error = pb_error.into();
        assert_eq!(
            anyhow_error.to_string(),
            "Start time must be before or equal to end time"
        );
    }

    #[test]
    fn test_error_debug_format() {
        let error = PbError::invalid_time_format("test");
        let debug_str = format!("{error:?}");
        assert!(debug_str.contains("InvalidTimeFormat"));
        assert!(debug_str.contains("test"));
    }

    #[test]
    fn test_pb_result_type() {
        let success: PbResult<i32> = Ok(42);
        let failure: PbResult<i32> = Err(PbError::StartAfterEnd);

        assert!(success.is_ok());
        assert!(failure.is_err());
        if let Ok(val) = success {
            assert_eq!(val, 42);
        }
        if let Err(err) = failure {
            assert_eq!(
                err.to_string(),
                "Start time must be before or equal to end time"
            );
        }
    }

    #[test]
    fn test_helper_function_invalid_time_format() {
        let error = PbError::invalid_time_format("2023-13-45");
        match error {
            PbError::InvalidTimeFormat { input } => {
                assert_eq!(input, "2023-13-45");
            }
            _ => panic!("Expected InvalidTimeFormat variant"),
        }
    }

    #[test]
    fn test_helper_function_invalid_relative_time_format() {
        let error = PbError::invalid_relative_time_format("5xyz");
        match error {
            PbError::InvalidRelativeTimeFormat { input } => {
                assert_eq!(input, "5xyz");
            }
            _ => panic!("Expected InvalidRelativeTimeFormat variant"),
        }
    }

    #[test]
    fn test_anyhow_integration() {
        use anyhow::Context;

        let result: anyhow::Result<()> =
            Err(PbError::StartAfterEnd).context("Failed to validate time range");

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Failed to validate time range"));
        // The root cause error message appears in the error chain,
        // but not necessarily in the top-level display
    }

    #[test]
    fn test_error_chain() {
        use anyhow::Context;

        // Simulate a chain of errors
        let root_error = PbError::invalid_time_format("malformed");
        let chain_result: anyhow::Result<()> = Err(root_error)
            .context("Time parsing failed")
            .context("CLI argument processing failed");

        assert!(chain_result.is_err());
        let error_msg = chain_result.unwrap_err().to_string();
        assert!(error_msg.contains("CLI argument processing failed"));
    }
}
