mod select_page;

use console::Term;
use dialoguer::theme::ColorfulTheme;
use select_page::select_page;

mod setup_browser;
use setup_browser::setup_browser;

use crate::{prelude::DebuggerInfo, utils::stdout_utils::clear_stdout_up};

mod select_script;

mod execute_script;

pub async fn setup_features() {
    let term = Term::stdout();
    let theme = &ColorfulTheme::default();
    let _ = setup_browser().await;

    loop {
        clear_stdout_up();
        let selected_page: DebuggerInfo = select_page(&term, theme).await;
        let script_content = select_script::select_script().await;
        execute_script::execute_script(&script_content, &selected_page, &theme, &term).await;
        break;
    }
}
