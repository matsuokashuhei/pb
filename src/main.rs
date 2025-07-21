use anyhow::Result;
use pb::Cli;

fn main() -> Result<()> {
    // Parse command line arguments
    let cli = Cli::parse_args()?;

    println!("pb CLI tool starting...");
    println!("Start time: {}", cli.start());
    println!("End time: {}", cli.end());
    println!("Update interval: {} seconds", cli.interval());

    // TODO: Implement actual progress bar functionality
    println!("Progress bar functionality not yet implemented");

    Ok(())
}
