use chrono::{Local, Utc};

fn main() {
    println!("Hello world!");

    // Get current local date and time
    let local_time = Local::now();
    println!("Local date and time: {}", local_time);
    
    // Get current UTC date and time
    let utc_time = Utc::now();
    println!("UTC date and time: {}", utc_time);
    
    // Format the date and time in a custom format
    let formatted = local_time.format("%Y-%m-%d %H:%M:%S");
    println!("Formatted local time: {}", formatted);
    
    // Another format example
    let custom_format = local_time.format("%A, %B %d, %Y at %I:%M %p");
    println!("Custom format: {}", custom_format);
}