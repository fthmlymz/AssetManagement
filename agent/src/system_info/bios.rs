use serde::Serialize;
use std::error::Error;

#[derive(Debug, Serialize, Clone)]
pub struct BiosInfo {
    pub vendor: Option<String>,
    pub version: Option<String>,
    pub release_date: Option<String>,
}

#[cfg(target_os = "windows")]
pub fn collect() -> Result<BiosInfo, Box<dyn Error>> {
    use wmi::{WMIConnection, COMLibrary};
    use std::collections::HashMap;

    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::new(com_con.into())?;

    let results: Vec<HashMap<String, String>> = wmi_con.raw_query("SELECT Manufacturer, SMBIOSBIOSVersion, ReleaseDate FROM Win32_BIOS")?;

    let first = results.get(0).cloned().unwrap_or_default();

    Ok(BiosInfo {
        vendor: first.get("Manufacturer").cloned(),
        version: first.get("SMBIOSBIOSVersion").cloned(),
        release_date: first.get("ReleaseDate").cloned(),
    })
}

#[cfg(target_os = "linux")]
pub fn collect() -> Result<BiosInfo, Box<dyn Error>> {
    use std::fs;

    let vendor = fs::read_to_string("/sys/class/dmi/id/bios_vendor").ok();
    let version = fs::read_to_string("/sys/class/dmi/id/bios_version").ok();
    let release_date = fs::read_to_string("/sys/class/dmi/id/bios_date").ok();

    Ok(BiosInfo {
        vendor: vendor.map(|s| s.trim().to_string()),
        version: version.map(|s| s.trim().to_string()),
        release_date: release_date.map(|s| s.trim().to_string()),
    })
}

#[cfg(target_os = "macos")]
pub fn collect() -> Result<BiosInfo, Box<dyn Error>> {
    use std::process::Command;

    let output = Command::new("system_profiler")
        .arg("SPHardwareDataType")
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    let vendor = Some("Apple".to_string());
    let version = stdout.lines().find(|l| l.contains("Boot ROM Version"))
        .map(|l| l.split(':').nth(1).unwrap_or("").trim().to_string());
    let release_date = None;

    Ok(BiosInfo {
        vendor,
        version,
        release_date,
    })
}
