use chrono::Local;
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

fn main() {
    loop {
        // Get the current time
        let now = Local::now();
        
        // Format the time as a string
        let time_str = now.format("%H:%M:%S").to_string();
        
        // Clear the terminal (this is a simple way, might not work on all terminals)
        print!("\x1B[2J\x1B[1;1H");
        
        // Print the time
        println!("Current time: {}", time_str);
        
        // Flush stdout to ensure the output is displayed immediately
        stdout().flush().unwrap();
        
        // Sleep for 1 second
        thread::sleep(Duration::from_secs(1));
    }
}