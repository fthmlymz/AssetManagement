use serde::Serialize;
use std::error::Error;

#[derive(Debug, Serialize, Clone)]
pub struct HotfixEntry {
    pub hotfix_id: String,
    pub description: Option<String>,
    pub installed_on: Option<String>,
}

#[cfg(target_os = "windows")]
pub fn collect() -> Result<Vec<HotfixEntry>, Box<dyn Error>> {
    use wmi::{COMLibrary, Variant, WMIConnection};
    use std::collections::HashMap;

    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::new(com_con.into())?;

    let results: Vec<HashMap<String, Variant>> =
        wmi_con.raw_query("SELECT HotFixID, Description, InstalledOn FROM Win32_QuickFixEngineering")?;

    let mut hotfixes = Vec::new();

    for result in results {
        let hotfix_id = match result.get("HotFixID") {
            Some(Variant::String(s)) => s.clone(),
            _ => "Unknown".to_string(),
        };

        let description = match result.get("Description") {
            Some(Variant::String(s)) => Some(s.clone()),
            _ => None,
        };

        let installed_on = match result.get("InstalledOn") {
            Some(Variant::String(s)) => Some(s.clone()),
            _ => None,
        };

        hotfixes.push(HotfixEntry {
            hotfix_id,
            description,
            installed_on,
        });
    }

    Ok(hotfixes)
}

#[cfg(target_os = "linux")]
pub fn collect() -> Result<Vec<HotfixEntry>, Box<dyn Error>> {
    use std::process::Command;
    let output = Command::new("bash")
        .arg("-c")
        .arg("dpkg -l 2>/dev/null || rpm -qa 2>/dev/null")
        .output()?;

    if !output.status.success() {
        return Err("dpkg/rpm command failed or not found".into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut hotfixes = Vec::new();

    for line in stdout.lines().skip(5) {
        if line.trim().is_empty() {
            continue;
        }
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let hotfix_id = tokens.get(1).unwrap_or(&"unknown").to_string();

        hotfixes.push(HotfixEntry {
            hotfix_id,
            description: Some("Installed package".to_string()),
            installed_on: None,
        });
    }

    Ok(hotfixes)
}

#[cfg(target_os = "macos")]
pub fn collect() -> Result<Vec<HotfixEntry>, Box<dyn Error>> {
    use std::process::Command;

    let output = Command::new("system_profiler")
        .arg("SPInstallHistoryDataType")
        .output()?;

    if !output.status.success() {
        return Err("system_profiler failed".into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut hotfixes = Vec::new();
    let mut current = HotfixEntry {
        hotfix_id: "Unknown".to_string(),
        description: None,
        installed_on: None,
    };

    for line in stdout.lines() {
        let line = line.trim();
        if line.starts_with("Name:") {
            current.hotfix_id = line.replacen("Name:", "", 1).trim().to_string();
        } else if line.starts_with("Date:") {
            current.installed_on = Some(line.replacen("Date:", "", 1).trim().to_string());
        } else if line.starts_with("Version:") {
            current.description = Some(line.replacen("Version:", "", 1).trim().to_string());
        } else if line.is_empty() {
            if current.hotfix_id != "Unknown" {
                hotfixes.push(current.clone());
                current = HotfixEntry {
                    hotfix_id: "Unknown".to_string(),
                    description: None,
                    installed_on: None,
                };
            }
        }
    }

    Ok(hotfixes)
}

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
pub fn collect() -> Result<Vec<HotfixEntry>, Box<dyn Error>> {
    Ok(vec![])
}
