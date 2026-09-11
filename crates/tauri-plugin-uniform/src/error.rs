use serde::{Serialize, Serializer};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Screen capture error: {0}")]
    Capture(String),

    #[error("No display monitor found")]
    NoMonitor,

    #[error("Coordinate ({0}, {1}) is out of monitor bounds")]
    OutOfBounds(i32, i32),

    #[error("Dialog cancelled by user")]
    DialogCancelled,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

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
