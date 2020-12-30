use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RDMA {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hca_handles: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hca_objects: Option<u32>,
}

impl RDMA {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::RDMA;
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({});

        let got = serde_json::to_value(RDMA::new()).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "hcaHandles": 3,
            "hcaObjects": 10000
        });

        let got = serde_json::to_value(RDMA {
            hca_handles: Some(3),
            hca_objects: Some(10000),
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
