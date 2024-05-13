//! Crate prelude

use serde::{Deserialize, Serialize};

pub use crate::error::Error;
pub type Result<T> = core::result::Result<T, Error>;

pub struct W<T>(pub T);

use std::fmt::Display;
// Personal preference.
pub use std::format as f;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DebuggerInfo {
    description: String,
    #[serde(rename = "devtoolsFrontendUrl")]
    devtools_frontend_url: Option<String>,
    id: String,
    title: String,
    #[serde(rename = "type")]
    page_type: String,
    url: Option<String>,
    #[serde(rename = "webSocketDebuggerUrl")]
    web_socket_debugger_url: Option<String>,
}

pub type DebuggerCollection = Vec<DebuggerInfo>;

impl DebuggerInfo {
    pub fn new(
        description: String,
        devtools_frontend_url: Option<String>,
        id: String,
        title: String,
        page_type: String,
        url: Option<String>,
        web_socket_debugger_url: Option<String>,
    ) -> Self {
        Self {
            description,
            devtools_frontend_url,
            id,
            title,
            page_type,
            url,
            web_socket_debugger_url,
        }
    }
    pub fn get_ws_url(&self) -> Option<&str> {
        self.web_socket_debugger_url.as_deref()
    }

    pub fn get_devtools_url(&self) -> Option<&str> {
        self.devtools_frontend_url.as_deref()
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_url(&self) -> Option<&str> {
        self.url.as_deref()
    }

    pub fn get_type(&self) -> &str {
        &self.page_type
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }
}

impl Display for DebuggerInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}
