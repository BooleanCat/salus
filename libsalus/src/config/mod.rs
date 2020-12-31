mod capabilities;
mod console_size;
pub mod linux;
mod mount;
mod process;
mod rlimit;
mod root;
pub mod solaris;
mod user;
pub mod windows;

pub use capabilities::Capabilities;
pub use console_size::ConsoleSize;
pub use mount::Mount;
pub use process::Process;
pub use rlimit::Rlimit;
pub use root::Root;
pub use user::User;

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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub linux: Option<linux::Linux>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows: Option<windows::Windows>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub solaris: Option<solaris::Solaris>,
}

impl Config {
    pub fn new(oci_version: &str) -> Self {
        Self {
            oci_version: String::from(oci_version),
            hostname: None,
            root: None,
            mounts: None,
            process: None,
            user: None,
            linux: None,
            windows: None,
            solaris: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        linux::Linux, solaris::Solaris, windows::Windows, Config, Mount, Process, Root, User,
    };
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
            },
            "user": {
                "username": "pikachu"
            },
            "linux": {},
            "windows": {},
            "solaris": {}
        });

        let got = serde_json::to_value(Config {
            root: Some(Root::new("/foo/bar")),
            hostname: Some(String::from("baz")),
            mounts: Some(vec![Mount::new("/bar/baz")]),
            process: Some(Process::new("/here")),
            user: Some(User::Windows {
                username: Some(String::from("pikachu")),
            }),
            linux: Some(Linux::new()),
            windows: Some(Windows::new()),
            solaris: Some(Solaris::new()),
            ..Config::new("0.1.0")
        })
        .unwrap();

        assert_eq!(want, got);
    }
}
