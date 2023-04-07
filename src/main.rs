use std::sync::{Arc, Mutex};
use std::thread;
mod spinner;
use spinner::spinner::start_spinner;
use spinner::spinner::spinner_cleanup;

fn main() {
    let stop_spinner = Arc::new(Mutex::new(false));
    let spinner_thread = start_spinner(vec!["-", "\\", "|", "/"].iter().map(|s| s.to_string()).collect(), "Loading".to_string(), stop_spinner.clone());
    
    thread::sleep(std::time::Duration::from_secs(5));
    
    *stop_spinner.lock().unwrap() = true;
    spinner_thread.join().unwrap();
    
    spinner_cleanup();
}
