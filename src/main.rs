use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use pmon::{
    calculate_progress, determine_start_time_for_end, get_current_time, parse_time,
    parse_time_with_base, render_colored_progress_bar_with_time, validate_times, Cli,
};
use std::io::{self, Write};
use std::time::Duration;

fn main() -> Result<()> {
    // Parse command line arguments
    let cli = match Cli::parse_args() {
        Ok(cli) => cli,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };

    // Parse start and end times
    let start_time = match cli.start() {
        Some(start_str) => {
            // Start time provided - parse it normally
            match parse_time(start_str) {
                Ok(time) => time,
                Err(e) => {
                    eprintln!("Error parsing start time '{start_str}': {e}");
                    std::process::exit(1);
                }
            }
        }
        None => {
            // No start time provided - determine it based on end time format
            determine_start_time_for_end(cli.end())
        }
    };

    // Parse end time using start time as base for relative calculations
    let end_time = match parse_time_with_base(cli.end(), Some(start_time)) {
        Ok(time) => time,
        Err(e) => {
            eprintln!("Error parsing end time '{}': {e}", cli.end());
            std::process::exit(1);
        }
    };

    // Validate time relationship
    if let Err(e) = validate_times(start_time, end_time) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }

    // Display header information only if verbose flag is set
    if cli.verbose() {
        println!("pb - Progress Bar Tool");
        println!("Start time: {}", start_time.format("%Y-%m-%d %H:%M:%S"));
        println!("End time: {}", end_time.format("%Y-%m-%d %H:%M:%S"));
        println!("Update interval: {} seconds", cli.interval());
        println!("Press Ctrl+C to exit\n");
    }

    // Check if we're in a TTY environment and if the environment is truly interactive
    let is_tty = crossterm::tty::IsTty::is_tty(&std::io::stdout());
    let is_interactive =
        is_tty && std::env::var("CI").is_err() && std::env::var("GITHUB_ACTIONS").is_err();

    // Enable raw mode for signal detection only if we're in an interactive TTY
    if is_interactive {
        crossterm::terminal::enable_raw_mode()?;
    }

    // Ensure terminal cleanup on exit
    let cleanup = move || {
        if is_interactive {
            let _ = crossterm::terminal::disable_raw_mode();
        }
        println!(); // New line before exit
    };

    // Set up panic hook for cleanup
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        if is_interactive {
            let _ = crossterm::terminal::disable_raw_mode();
        }
        println!(); // New line before exit
        original_hook(panic_info);
    }));

    // Main application loop
    let result = run_progress_loop(start_time, end_time, cli.interval(), is_interactive);

    // Cleanup and handle result
    cleanup();

    match result {
        Ok(_) => {
            println!("Progress monitoring completed successfully.");
            Ok(())
        }
        Err(e) => {
            eprintln!("Error during progress monitoring: {e}");
            std::process::exit(1);
        }
    }
}

/// Run the main progress monitoring loop
fn run_progress_loop(
    start_time: chrono::NaiveDateTime,
    end_time: chrono::NaiveDateTime,
    interval_seconds: u64,
    is_interactive: bool,
) -> Result<()> {
    let interval_duration = Duration::from_secs(interval_seconds);
    let poll_duration = Duration::from_millis(100); // Check for Ctrl+C every 100ms

    loop {
        // Get current time and calculate progress (using centralized time function)
        let current_time = get_current_time();
        let progress = calculate_progress(start_time, end_time, current_time);

        // Render progress bar with time information
        let bar =
            render_colored_progress_bar_with_time(progress, start_time, end_time, current_time);

        // Update display
        if is_interactive {
            // In interactive TTY mode, use carriage return to overwrite the current line
            print!("\r{bar}");
            io::stdout().flush()?;
        } else {
            // In non-interactive mode, just print the progress bar
            println!("{bar}");
        }

        // Check if we've completed (progress >= 100%)
        if progress >= 100.0 {
            if !is_interactive {
                println!("Progress completed! Time range has elapsed.");
            } else {
                println!("\nProgress completed! Time range has elapsed.");
            }
            break;
        }

        // Sleep with periodic Ctrl+C checking (only in interactive mode)
        if is_interactive {
            let mut remaining_sleep = interval_duration;
            while remaining_sleep > Duration::ZERO {
                let sleep_chunk = remaining_sleep.min(poll_duration);

                // Check for Ctrl+C
                if event::poll(sleep_chunk)? {
                    if let Event::Key(KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    }) = event::read()?
                    {
                        println!("\nReceived Ctrl+C, exiting gracefully...");
                        return Ok(());
                    }
                    // Ignore other key events
                }

                remaining_sleep = remaining_sleep.saturating_sub(sleep_chunk);
            }
        } else {
            // In non-interactive mode, just sleep for the full interval
            std::thread::sleep(interval_duration);
        }
    }

    Ok(())
}
