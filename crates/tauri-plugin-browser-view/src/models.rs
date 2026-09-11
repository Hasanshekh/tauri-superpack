use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TabBounds {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTabOptions {
    pub label: String,
    pub url: String,
    pub bounds: TabBounds,
    pub window_label: Option<String>,
    pub partition: Option<String>,
    pub user_agent: Option<String>,
    pub devtools: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TabInfo {
    pub label: String,
    pub url: String,
    pub bounds: TabBounds,
    pub is_active: bool,
    pub partition: Option<String>,
}
