use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Solaris {}

impl Solaris {
    pub fn new() -> Self {
        Default::default()
    }
}
