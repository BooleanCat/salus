use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Linux {}

impl Linux {
    pub fn new() -> Self {
        Default::default()
    }
}
