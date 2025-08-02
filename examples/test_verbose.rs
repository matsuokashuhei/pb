use chrono::NaiveDateTime;

fn main() {
    let start = NaiveDateTime::parse_from_str("2025-08-02 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let end = NaiveDateTime::parse_from_str("2025-08-31 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let current = NaiveDateTime::parse_from_str("2025-08-02 18:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    
    use pmon::progress_bar::format_verbose_layout;
    
    let result = format_verbose_layout(2.5, start, end, current);
    println!("Current output:");
    println!("```");
    println!("{}", result);
    println!("```");
    
    // Show the issue more clearly
    println!("\nAnalysis:");
    const BAR_WIDTH: usize = 40;
    println!("BAR_WIDTH = {}", BAR_WIDTH);
    println!("width used for date line = BAR_WIDTH / 2 = {}", BAR_WIDTH / 2);
    
    let start_date = start.format("%Y-%m-%d").to_string();
    let end_date = end.format("%Y-%m-%d").to_string();
    let date_line = format!(
        "{:<width$}{:>width$}",
        start_date,
        end_date,
        width = BAR_WIDTH / 2
    );
    println!("Date line length: {}", date_line.len());
    println!("Date line: '{}'", date_line);
    
    let bar = pmon::progress_bar::format_minimal_only(2.5);
    println!("Bar length: {}", bar.len());
    println!("Bar: '{}'", bar);
}