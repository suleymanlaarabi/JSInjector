use std::io;

use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeError;

use thiserror::Error;

#[derive(Debug, Error)]
enum PageInfoError {
    #[error("error reading file: {0}")]
    Io(#[from] io::Error),
    #[error("error parsing JSON: {0}")]
    Json(#[from] SerdeError),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PageType {
    Page,
    Iframe,
    BackgroundPage,
    ServiceWorker,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageInfo {
    description: String,
    #[serde(rename = "devtoolsFrontendUrl")]
    devtools_frontend_url: String,
    id: String,
    title: String,
    #[serde(rename = "type")]
    page_type: PageType,
    url: String,
    #[serde(rename = "webSocketDebuggerUrl")]
    web_socket_debugger_url: String,
}

impl PageInfo {
    pub fn to_json(&self) -> Result<String, SerdeError> {
        serde_json::to_string(&self)
    }

    pub fn from_json(json: &str) -> Result<Self, SerdeError> {
        serde_json::from_str(json)
    }
    pub fn is_secure(&self) -> bool {
        self.url.starts_with("https://") || self.url.starts_with("chrome-extension://")
    }
    pub fn summary(&self) -> String {
        format!(
            "Title: '{}', Type: {:?}, URL: '{}', WebSocket: '{}'",
            self.title, self.page_type, self.url, self.web_socket_debugger_url
        )
    }
    pub fn can_debug(&self) -> bool {
        !self.web_socket_debugger_url.is_empty()
    }
    pub fn page_type_string(&self) -> String {
        match self.page_type {
            PageType::Page => "Standard Page".to_string(),
            PageType::Iframe => "IFrame".to_string(),
            PageType::BackgroundPage => "Background Page".to_string(),
            PageType::ServiceWorker => "Service Worker".to_string(),
        }
    }
}
