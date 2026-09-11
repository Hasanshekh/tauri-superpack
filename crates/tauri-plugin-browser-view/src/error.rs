use serde::{Serialize, Serializer};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Tab '{0}' not found")]
    TabNotFound(String),

    #[error("Tab with label '{0}' already exists")]
    TabAlreadyExists(String),

    #[error("Window '{0}' not found")]
    WindowNotFound(String),

    #[error("Failed to parse URL: {0}")]
    InvalidUrl(String),

    #[error("Webview operation failed: {0}")]
    Webview(String),

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
