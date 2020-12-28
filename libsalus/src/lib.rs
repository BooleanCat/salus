use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Config {
    pub oci_version: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<ConfigRoot>,
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
    use jsonschema::JSONSchema;
    use serde_json;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn validate_schema() {
        let config = Config {
            oci_version: String::from("0.1.0"),
            root: Some(ConfigRoot {
                path: String::from("/foo/bar"),
                read_only: Some(true),
            }),
            hostname: Some(String::from("baz")),
        };

        if let Err(errs) = JSONSchema::compile(
            &serde_json::from_reader(BufReader::new(
                File::open("src/schema/config-schema.json").unwrap(),
            ))
            .unwrap(),
        )
        .unwrap()
        .validate(&serde_json::to_value(config).unwrap())
        {
            for err in errs {
                println!("{}", err);
            }

            panic!("failed validating config schema");
        }
    }
}
