mod block_io;
mod cpu;
mod device;
mod huge_page_limit;
mod memory;
mod network;
mod priority;
mod throttle_device;
mod weight_device;

pub use block_io::BlockIO;
pub use cpu::CPU;
pub use device::Device;
pub use huge_page_limit::HugePageLimit;
pub use memory::Memory;
pub use network::Network;
pub use priority::Priority;
pub use throttle_device::ThrottleDevice;
pub use weight_device::WeightDevice;

use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Resources {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Device>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<Memory>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<CPU>,

    #[serde(rename = "blockIO", skip_serializing_if = "Option::is_none")]
    pub block_io: Option<BlockIO>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub huge_page_limits: Option<Vec<HugePageLimit>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<Network>,
}

impl Resources {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::{BlockIO, Device, HugePageLimit, Memory, Network, Resources, CPU};
    use serde_json;

    #[test]
    fn serialize() {
        let want = serde_json::json!({});

        let got = serde_json::to_value(Resources::new()).unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn serialize_optional_fields() {
        let want = serde_json::json!({
            "devices": [
                {
                    "allow": true
                }
            ],
            "memory": {},
            "cpu": {},
            "blockIO": {},
            "hugePageLimits": [
                {
                    "pageSize": "2MB",
                    "limit": 209715200
                }
            ],
            "network": {}
        });

        let got = serde_json::to_value(Resources {
            devices: Some(vec![Device::new(true)]),
            memory: Some(Memory::new()),
            cpu: Some(CPU::new()),
            block_io: Some(BlockIO::new()),
            huge_page_limits: Some(vec![HugePageLimit::new("2MB", 209715200)]),
            network: Some(Network::new()),
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
