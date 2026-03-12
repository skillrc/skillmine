use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}

impl Skill {
    pub fn new(name: String, version: String) -> Self {
        Self {
            name,
            version,
            description: None,
        }
    }
}
