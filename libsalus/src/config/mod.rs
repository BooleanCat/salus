mod root;

pub use root::Root;

use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Config {
    pub oci_version: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<Root>,
}

impl Config {
    pub fn new(oci_version: &str) -> Self {
        Config {
            oci_version: String::from(oci_version),
            hostname: None,
            root: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, Root};
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
                "path": String::from("/foo/bar"),
            }
        });

        let got = serde_json::to_value(Config {
            root: Some(Root::new("/foo/bar")),
            hostname: Some(String::from("baz")),
            ..Config::new("0.1.0")
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
