# Rust Spinner
A simple spinner for Rust.

## Usage

```rust
use std::sync::{Arc, Mutex};
use std::thread;
mod spinner;
use spinner::spinner::start_spinner; // Import the start_spinner function
use spinner::spinner::spinner_cleanup; // Import the spinner_cleanup function

fn main() {
    let stop_spinner = Arc::new(Mutex::new(false)); // Create a mutex to stop the spinner
    let spinner_thread = start_spinner(vec!["-", "\\", "|", "/"].iter().map(|s| s.to_string()).collect(), "Loading".to_string(), 100 , stop_spinner.clone()); // Start the spinner
    
    thread::sleep(std::time::Duration::from_secs(5)); // Wait for 5 seconds
    
    *stop_spinner.lock().unwrap() = true; // Stop the spinner
    spinner_thread.join().unwrap(); // Join the spinner thread
    
    spinner_cleanup(); // Cleanup the spinner from the terminal
}
```