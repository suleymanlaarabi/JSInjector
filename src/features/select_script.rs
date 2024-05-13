use dialoguer::Input;
use tokio::fs;

use crate::utils::stdout_utils::clear_stdout_up;

pub async fn select_script() -> String {
    loop {
        let path: String = Input::new()
            .with_prompt("Path of your script ? ")
            .interact_text()
            .unwrap();

        let script_content = fs::read_to_string(&path).await;
        clear_stdout_up();
        match script_content {
            Ok(content) => {
                println!("Script content: {}", content);
                return content;
            }
            Err(_) => {
                println!("Unable to read the file");
                continue;
            }
        }
    }
}
