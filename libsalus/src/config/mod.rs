mod capabilities;
mod console_size;
mod mount;
mod process;
mod rlimit;
mod root;

pub use capabilities::Capabilities;
pub use console_size::ConsoleSize;
pub use mount::Mount;
pub use process::Process;
pub use rlimit::Rlimit;
pub use root::Root;

use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub oci_version: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<Root>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<Mount>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub process: Option<Process>,
}

impl Config {
    pub fn new(oci_version: &str) -> Self {
        Self {
            oci_version: String::from(oci_version),
            hostname: None,
            root: None,
            mounts: None,
            process: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, Mount, Process, Root};
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({
            "ociVersion": "0.1.0"
        });

        let got = serde_json::to_value(Config::new("0.1.0")).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "ociVersion": "0.1.0",
            "hostname": "baz",
            "root": {
                "path": "/foo/bar",
            },
            "mounts": [
                {
                    "destination": "/bar/baz"
                }
            ],
            "process": {
                "cwd": "/here"
            }
        });

        let got = serde_json::to_value(Config {
            root: Some(Root::new("/foo/bar")),
            hostname: Some(String::from("baz")),
            mounts: Some(vec![Mount::new("/bar/baz")]),
            process: Some(Process::new("/here")),
            ..Config::new("0.1.0")
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
