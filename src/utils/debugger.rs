use crate::prelude::DebuggerCollection;

pub async fn get_debugger_info() -> DebuggerCollection {
    let body = reqwest::get("http://localhost:9222/json")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    serde_json::from_str::<DebuggerCollection>(&body).unwrap()
}
