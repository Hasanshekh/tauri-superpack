use serde::{Serialize, Serializer};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Plugin '{0}' not found")]
    PluginNotFound(String),

    #[error("Plugin with ID '{0}' already registered")]
    PluginAlreadyRegistered(String),

    #[error("JavaScript execution error: {0}")]
    JsExecution(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error(transparent)]
    Tauri(#[from] tauri::Error),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
