use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub mac_address: Option<String>,
    pub ipv4: Vec<String>,
    pub ipv6: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ExtendedSystemInfo {
    pub base: super::system_info::base::SystemInfo,
    pub interfaces: Vec<NetworkInterface>,
    // gelecekte: bios, programs, hotfix, vb.
}
