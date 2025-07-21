//! pb - A CLI progress bar tool for time-based visualization
//!
//! This library provides the core functionality for the pb CLI tool,
//! including time parsing, progress calculation, and error handling.

pub mod cli;
pub mod error;
pub mod progress_bar;
pub mod time_parser;

// Re-export commonly used types
pub use anyhow::{Context, Result as AnyhowResult};
pub use error::{PbError, PbResult};
