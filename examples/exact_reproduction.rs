use chrono::NaiveDateTime;
use pmon::progress_bar::format_verbose_layout;

fn main() {
    // Use exact same parameters as user's example
    let start = NaiveDateTime::parse_from_str("2025-08-02 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let end = NaiveDateTime::parse_from_str("2025-08-31 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let current = NaiveDateTime::parse_from_str("2025-08-02 18:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    
    let layout = format_verbose_layout(2.5, start, end, current);
    
    println!("=== EXACT REPRODUCTION ===");
    println!("Raw layout string with escapes:");
    println!("{:?}", layout);
    
    println!("\n=== ACTUAL OUTPUT ===");
    println!("{}", layout);
    
    println!("\n=== LINE BY LINE ANALYSIS ===");
    let lines: Vec<&str> = layout.split('\n').collect();
    for (i, line) in lines.iter().enumerate() {
        println!("Line {}: '{}' (length: {})", i + 1, line, line.len());
        
        // Show first 10 characters in detail
        print!("  First 10 chars: ");
        for (j, c) in line.chars().take(10).enumerate() {
            print!("'{}' ", c);
        }
        println!();
        
        // Show if line starts with spaces
        let leading_spaces = line.chars().take_while(|&c| c == ' ').count();
        println!("  Leading spaces: {}", leading_spaces);
    }
    
    println!("\n=== Compare to user's desired output ===");
    println!("User wants:");
    println!("2025-08-02                    2025-08-31");
    println!("█░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░");
    println!("2.5% elapsed | 28d 6h remaining");
    
    println!("\nOur output:");
    println!("{}", layout);
    
    println!("\nAre they identical? {}", layout == "2025-08-02                    2025-08-31\n█░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░\n2.5% elapsed | 28d 6h remaining");
}