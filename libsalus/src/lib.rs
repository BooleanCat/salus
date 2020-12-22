use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Config {
    pub oci_version: String,
    pub root: ConfigRoot,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ConfigRoot {
    pub path: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::{Config, ConfigRoot};
    use serde_json;

    #[test]
    fn config_deserialize() {
        let config: Config = serde_json::from_str(
            r#"
            {
                "ociVersion": "0.1.0",
                "root": {
                    "path": "/foo/bar",
                    "readOnly": true
                }
            }
        "#,
        )
        .unwrap();

        assert_eq!(
            config,
            Config {
                oci_version: String::from("0.1.0"),
                root: ConfigRoot {
                    path: String::from("/foo/bar"),
                    read_only: Some(true),
                }
            }
        );
    }
}
