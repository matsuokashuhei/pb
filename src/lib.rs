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
pub use cli::Cli;
pub use error::{PbError, PbResult};
pub use progress_bar::{calculate_progress, render_colored_progress_bar, render_progress_bar};
pub use time_parser::{
    get_current_time, parse_date, parse_datetime, parse_relative_time, parse_time, parse_time_with_base, validate_times,
};
