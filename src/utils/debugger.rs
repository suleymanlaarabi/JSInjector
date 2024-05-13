use crate::prelude::DebuggerCollection;

pub async fn get_debugger_info() -> Result<DebuggerCollection, String> {
    let res = reqwest::get("http://localhost:9222/json").await;
    println!("{}", 10 );
    match res {
        Ok(body) => {
            return Ok(
                serde_json::from_str::<DebuggerCollection>(&body.text().await.unwrap()).unwrap(),
            );
        }
        Err(_) => Err(String::from("Chrome is not available")),
    }
}
