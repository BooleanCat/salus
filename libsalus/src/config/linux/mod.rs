mod namespace;

pub use self::namespace::Namespace;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Linux {
    #[serde(skip_serializing_if = "Option::is_none")]
    namespaces: Option<Vec<Namespace>>,
}

impl Linux {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::{Linux, Namespace};
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
            ]
        });

        let got = serde_json::to_value(Linux {
            namespaces: Some(vec![Namespace::new("pid")]),
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
