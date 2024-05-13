use std::{
    fs,
    path::PathBuf,
    time::{Duration, Instant},
};

use crate::{prelude::DebuggerInfo, utils::stdout_utils::clear_stdout};
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use futures::{
    channel::mpsc::{channel, Receiver},
    stream::StreamExt,
    SinkExt,
};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

pub async fn execute_script(
    script_path: &PathBuf,
    debugger_info: &DebuggerInfo,
    theme: &ColorfulTheme,
    term: &Term,
) {
    let menu: Vec<&str> = vec!["Hot refresh", "Just inject"];

    let selection = Select::with_theme(theme)
        .with_prompt("Options: ")
        .default(0)
        .items(&menu)
        .interact_on_opt(&term);

    let (mut socket, _) = connect_async(debugger_info.get_ws_url().unwrap())
        .await
        .expect("Failed to connect");

    async fn execute(
        script_path: &PathBuf,
        socket: &mut (impl SinkExt<Message> + Unpin + StreamExt),
    ) {
        clear_stdout();
        println!("script injected");
        let binding = fs::read_to_string(script_path).unwrap();
        let script = binding.as_str();

        let mut js_command = JsCommand::new();
        js_command.add_script(script);

        let message = Message::Text(js_command.to_string());

        let _ = socket.send(message).await;
    }

    match selection {
        Ok(Some(value)) => match value {
            0 => {
                let (mut watcher, mut rx) = async_watcher().unwrap();

                watcher
                    .watch(&script_path, RecursiveMode::Recursive)
                    .unwrap();

                let mut last_exec = Instant::now();
                execute(&script_path, &mut socket).await;
                while let Some(_) = rx.next().await {
                    let now = Instant::now();

                    if now.duration_since(last_exec) > Duration::from_secs(1)
                        || now.duration_since(last_exec) < Duration::from_secs(0)
                    {
                        execute(&script_path, &mut socket).await;
                        last_exec = now;
                    }
                }
            }
            1 => {
                execute(&script_path, &mut socket).await;
            }
            _ => {
                eprintln!("Invalid option");
            }
        },
        Ok(None) => {
            eprintln!("No option selected");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    sleep(Duration::from_secs(1)).await;
}

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);

    let watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}

#[derive(Serialize, Deserialize, Debug)]
struct JsCommand {
    id: u32,
    method: String,
    params: Params,
}

#[derive(Serialize, Deserialize, Debug)]
struct Params {
    expression: String,
}

impl JsCommand {
    fn new() -> JsCommand {
        JsCommand {
            id: 1,
            method: "Runtime.evaluate".to_string(),
            params: Params {
                expression: "".to_string(),
            },
        }
    }

    fn add_script(&mut self, script: &str) {
        self.params.expression = script.to_string();
    }

    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
