use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use pmon::{
    calculate_progress, determine_start_time_for_end, format_minimal_only, format_verbose_layout,
    get_current_time, parse_time, parse_time_with_base, render_colored_progress_bar_with_time,
    validate_times, Cli, DisplayMode,
};
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

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
        println!("pmon - Progress Monitor Tool");
        println!("Start time: {}", start_time.format("%Y-%m-%d %H:%M:%S"));
        println!("End time: {}", end_time.format("%Y-%m-%d %H:%M:%S"));
        println!("Update interval: {} seconds", cli.interval());
        println!("Press Ctrl+C to exit\n");
    }

    // Check if we're in a TTY environment and if the environment is truly interactive
    let is_tty = crossterm::tty::IsTty::is_tty(&std::io::stdout());
    let is_interactive =
        is_tty && std::env::var("CI").is_err() && std::env::var("GITHUB_ACTIONS").is_err();

    if is_interactive {
        // Run in interactive mode with keyboard controls
        run_interactive_mode(start_time, end_time, cli.interval())
    } else {
        // Run in pipe mode (backward compatibility)
        run_pipe_mode(start_time, end_time, cli.interval())
    }
}

/// Run the interactive mode with keyboard controls and display mode switching
fn run_interactive_mode(
    start_time: chrono::NaiveDateTime,
    end_time: chrono::NaiveDateTime,
    interval_seconds: u64,
) -> Result<()> {
    // Set up signal handler for graceful exit
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    // Enable raw mode and alternate screen
    terminal::enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;

    // Set up cleanup function
    let cleanup = || {
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
    };

    // Set up panic hook for cleanup
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
        original_hook(panic_info);
    }));

    let mut display_mode = DisplayMode::Minimal;
    let mut show_instructions = true;
    let instruction_start = Instant::now();
    let interval_duration = Duration::from_secs(interval_seconds);
    let poll_duration = Duration::from_millis(100);

    let result = 'main_loop: loop {
        // Check for signals
        if !running.load(Ordering::SeqCst) {
            break Ok(());
        }

        // Get current time and calculate progress
        let current_time = get_current_time();
        let progress = calculate_progress(start_time, end_time, current_time);

        // Check if we've completed
        if progress >= 100.0 {
            break Ok(());
        }

        // Hide instructions after 3 seconds in minimal mode
        if show_instructions && display_mode == DisplayMode::Minimal {
            if instruction_start.elapsed() >= Duration::from_secs(3) {
                show_instructions = false;
            }
        }

        // Clear screen and render display
        print!("\r\x1b[2J\x1b[1;1H"); // Clear screen and move cursor to top-left

        match display_mode {
            DisplayMode::Minimal => {
                let bar = format_minimal_only(progress);
                println!("{}", bar);

                if show_instructions {
                    println!("Press 'v' for details, 'q' to quit");
                }
            }
            DisplayMode::Verbose => {
                let layout = format_verbose_layout(progress, start_time, end_time, current_time);
                println!("{}", layout);
                println!();
                println!("Press 'v' to toggle view, 'q' to quit");
            }
        }

        io::stdout().flush()?;

        // Handle input and sleep
        let mut remaining_sleep = interval_duration;
        while remaining_sleep > Duration::ZERO && running.load(Ordering::SeqCst) {
            let sleep_chunk = remaining_sleep.min(poll_duration);

            // Check for input
            if event::poll(sleep_chunk)? {
                match event::read()? {
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('v') | KeyCode::Char('V'),
                        ..
                    }) => {
                        // Toggle display mode
                        display_mode = match display_mode {
                            DisplayMode::Minimal => DisplayMode::Verbose,
                            DisplayMode::Verbose => DisplayMode::Minimal,
                        };

                        // Reset instruction timer if switching to minimal
                        if display_mode == DisplayMode::Minimal {
                            show_instructions = true;
                        }

                        break; // Redraw immediately
                    }
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('q') | KeyCode::Char('Q'),
                        ..
                    })
                    | Event::Key(KeyEvent {
                        code: KeyCode::Esc, ..
                    }) => {
                        break 'main_loop Ok(());
                    }
                    Event::Key(KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    }) => {
                        break 'main_loop Ok(());
                    }
                    _ => {
                        // Ignore other key events
                    }
                }
            }

            remaining_sleep = remaining_sleep.saturating_sub(sleep_chunk);
        }
    };

    // Cleanup
    cleanup();

    result
}

/// Run the pipe mode for backward compatibility (non-interactive)
fn run_pipe_mode(
    start_time: chrono::NaiveDateTime,
    end_time: chrono::NaiveDateTime,
    interval_seconds: u64,
) -> Result<()> {
    let interval_duration = Duration::from_secs(interval_seconds);

    loop {
        // Get current time and calculate progress
        let current_time = get_current_time();
        let progress = calculate_progress(start_time, end_time, current_time);

        // Render progress bar with time information (original format)
        let bar =
            render_colored_progress_bar_with_time(progress, start_time, end_time, current_time);

        // Print the progress bar (each on a new line for pipe mode)
        println!("{bar}");

        // Check if we've completed (progress >= 100%)
        if progress >= 100.0 {
            println!("Progress completed! Time range has elapsed.");
            break;
        }

        // Sleep for the full interval in pipe mode
        std::thread::sleep(interval_duration);
    }

    Ok(())
}
