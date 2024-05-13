use std::path::{Path, PathBuf};

use dialoguer::Input;

use crate::utils::stdout_utils::clear_stdout_up;

pub async fn select_script() -> PathBuf {
    loop {
        let path: String = Input::new()
            .with_prompt("Path of your script ? ")
            .interact_text()
            .unwrap();

        let script_path = Path::new(&path).to_owned();
        let exist_file = script_path.is_file();

        clear_stdout_up();
        if exist_file {
            return script_path;
        }
        println!("Unable to read the scrip");
    }
}
