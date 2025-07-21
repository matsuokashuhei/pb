use chrono::NaiveDateTime;

fn main() {
    // Test 24:00:00
    let result = NaiveDateTime::parse_from_str("2025-07-21 24:00:00", "%Y-%m-%d %H:%M:%S");
    println!("24:00:00 result: {:?}", result);
    
    // Test negative hour
    let result = NaiveDateTime::parse_from_str("2025-07-21 -1:30:45", "%Y-%m-%d %H:%M:%S");
    println!("-1:30:45 result: {:?}", result);
    
    // Test 25:00:00
    let result = NaiveDateTime::parse_from_str("2025-07-21 25:00:00", "%Y-%m-%d %H:%M:%S");
    println!("25:00:00 result: {:?}", result);
}
