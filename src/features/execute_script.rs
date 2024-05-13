use std::time::Duration;

use futures::{stream::StreamExt, SinkExt};
use tokio::time::sleep;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use crate::prelude::DebuggerInfo;

pub async fn execute_script(script: &String, debugger_info: &DebuggerInfo) {
    let (mut socket, _) = connect_async(debugger_info.get_ws_url().unwrap())
        .await
        .expect("Failed to connect");

    let mut js_command = JsCommand::new();
    js_command.add_script(script);

    let message = Message::Text(js_command.to_string());

    println!("Sending message: {:?}", js_command.to_string());

    socket.send(message).await.expect("Failed to send message");

    while let Some(message) = socket.next().await {
        let message = message.expect("Failed to receive a message");
        if let Message::Text(text) = message {
            println!("Response: {}", text);
            break;
        }
    }

    sleep(Duration::from_secs(1)).await;
}

use serde::{Deserialize, Serialize};

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
