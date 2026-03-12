use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub concurrency: usize,
    pub timeout: u64,
    pub auto_sync: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            concurrency: 5,
            timeout: 300,
            auto_sync: false,
        }
    }
}
