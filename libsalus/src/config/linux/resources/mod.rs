mod block_io;
mod cpu;
mod device;
mod memory;
mod throttle_device;
mod weight_device;

pub use block_io::BlockIO;
pub use cpu::CPU;
pub use device::Device;
pub use memory::Memory;
pub use throttle_device::ThrottleDevice;
pub use weight_device::WeightDevice;

use serde::{Deserialize, Serialize};

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
}

impl Resources {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::{BlockIO, Device, Memory, Resources, CPU};
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
            "blockIO": {}
        });

        let got = serde_json::to_value(Resources {
            devices: Some(vec![Device::new(true)]),
            memory: Some(Memory::new()),
            cpu: Some(CPU::new()),
            block_io: Some(BlockIO::new()),
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
