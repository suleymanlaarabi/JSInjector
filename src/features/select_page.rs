use std::time::Duration;

use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Select};
use tokio::time::sleep;

use crate::{
    prelude::{DebuggerCollection, DebuggerInfo},
    utils::{debugger::get_debugger_info, stdout_utils::clear_stdout},
};

pub async fn select_page(term: &Term, theme: &ColorfulTheme) -> DebuggerInfo {
    loop {
        let mut menu: DebuggerCollection = vec![];

        loop {
            sleep(Duration::from_millis(300)).await;
            match get_debugger_info().await {
                Ok(value) => {
                    menu.extend(value);
                    break;
                }
                Err(_) => continue,
            }
        }

        clear_stdout();
        menu.insert(
            0,
            DebuggerInfo::new(
                "Refresh list".to_string(),
                None,
                "Refresh list".to_string(),
                "Refresh list".to_string(),
                "Refresh list".to_string(),
                None,
                None,
            ),
        );

        let app_title: console::StyledObject<&str> =
            style("JSInjector").bold().underlined().green();
        println!("\n{}", app_title);

        let selection = Select::with_theme(theme)
            .with_prompt("Options: ")
            .default(0)
            .items(&menu)
            .interact_on_opt(&term);

        match selection {
            Ok(Some(0)) => continue,
            Ok(Some(index)) => {
                let selected = &menu[index];
                return selected.clone();
            }
            Ok(None) => {
                println!("No selection made");
                continue;
            }
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        }
    }
}
