use std::process::{self, Child, Command, Stdio};
use std::sync::{Arc, Mutex};

use tokio::spawn;
use tokio::task::JoinHandle;

pub async fn setup_browser() -> JoinHandle<Arc<Mutex<Child>>> {
    let chrome_thread = spawn(async {
        let child = Command::new("google-chrome")
            .args(["--remote-debugging-port=9222"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to start Chrome");
        println!("Starting Chrome...");
        let child = Arc::new(Mutex::new(child));
        let child_clone = child.clone();
        ctrlc::set_handler(move || {
            println!("Killing Chrome...");
            let mut child = child_clone.lock().unwrap();
            child.kill().unwrap();
            process::exit(0);
        })
        .unwrap();

        child
    });

    chrome_thread
}
