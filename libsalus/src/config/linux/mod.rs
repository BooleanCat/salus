mod id_mapping;
mod namespace;

pub use self::id_mapping::IDMapping;
pub use self::namespace::Namespace;

use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Linux {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<Namespace>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid_mappings: Option<Vec<IDMapping>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid_mappings: Option<Vec<IDMapping>>,
}

impl Linux {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::{IDMapping, Linux, Namespace};
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({});

        let got = serde_json::to_value(Linux::new()).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "namespaces": [
                {
                    "type": "pid"
                }
            ],
            "uidMappings": [
                {
                    "containerID": 0,
                    "hostID": 2000,
                    "size": 100
                }
            ],
            "gidMappings": [
                {
                    "containerID": 0,
                    "hostID": 3000,
                    "size": 200
                }
            ]
        });

        let got = serde_json::to_value(Linux {
            namespaces: Some(vec![Namespace::new("pid")]),
            uid_mappings: Some(vec![IDMapping::new(0, 2000, 100)]),
            gid_mappings: Some(vec![IDMapping::new(0, 3000, 200)]),
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
