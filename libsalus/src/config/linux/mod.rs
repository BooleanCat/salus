mod device;
mod id_mapping;
mod intel_rdt;
mod namespace;
pub mod resources;
mod seccomp;
mod syscall;
mod syscall_arg;

pub use self::device::Device;
pub use self::id_mapping::IDMapping;
pub use self::namespace::Namespace;
pub use intel_rdt::IntelRDT;
pub use seccomp::Seccomp;
pub use syscall::Syscall;
pub use syscall_arg::SyscallArg;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Linux {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<Namespace>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid_mappings: Option<Vec<IDMapping>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid_mappings: Option<Vec<IDMapping>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Device>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroups_path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<resources::Resources>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unified: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub intel_rdt: Option<IntelRDT>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sysctl: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub seccomp: Option<Seccomp>,
}

impl Linux {
    pub fn new() -> Self {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::{resources, Device, IDMapping, IntelRDT, Linux, Namespace, Seccomp};
    use serde_json;
    use std::collections::HashMap;

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
            ],
            "devices": [
                {
                    "type": "c",
                    "path": "/dev/fuse",
                    "major": 10,
                    "minor": 229
                }
            ],
            "cgroupsPath": "/cgroups",
            "resources": {},
            "unified": {
                "hugetlb.1GB.max": "1073741824"
            },
            "intelRdt": {},
            "sysctl": {
                "net.ipv4.ip_forward": "1"
            },
            "seccomp": {
                "defaultAction": "SCMP_ACT_ALLOW"
            }
        });

        let mut unified = HashMap::new();
        unified.insert(String::from("hugetlb.1GB.max"), String::from("1073741824"));

        let mut sysctl = HashMap::new();
        sysctl.insert(String::from("net.ipv4.ip_forward"), String::from("1"));

        let got = serde_json::to_value(Linux {
            namespaces: Some(vec![Namespace::new("pid")]),
            uid_mappings: Some(vec![IDMapping::new(0, 2000, 100)]),
            gid_mappings: Some(vec![IDMapping::new(0, 3000, 200)]),
            devices: Some(vec![Device::new("c", "/dev/fuse", 10, 229)]),
            cgroups_path: Some(String::from("/cgroups")),
            resources: Some(resources::Resources::new()),
            unified: Some(unified),
            intel_rdt: Some(IntelRDT::new()),
            sysctl: Some(sysctl),
            seccomp: Some(Seccomp::new("SCMP_ACT_ALLOW")),
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
