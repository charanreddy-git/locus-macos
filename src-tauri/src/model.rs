use serde::Serialize;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::Mutex;

pub struct StreamState {
    pub cancel_flag: Arc<Mutex<bool>>,
}

// macOS-only error types
#[derive(Error, Debug)]
pub enum MacOSError {
    #[error("No active window found")]
    NoActiveWindow,
    #[error("CoreGraphics error: {0}")]
    CoreGraphicsError(String),
}

#[derive(Serialize, Debug, Clone)]
pub struct ActiveWindow {
    pub class: String,
    pub title: String,
}

impl ActiveWindow {
    pub fn none() -> Self {
        ActiveWindow {
            title: "none".to_string(),
            class: "none".to_string(),
        }
    }
}

impl PartialEq for ActiveWindow {
    fn eq(&self, other: &Self) -> bool {
        self.class == other.class && self.title == other.title
    }
}
