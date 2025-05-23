use serde::Serialize;
use std::error::Error;

#[derive(Debug, Serialize, Clone)]
pub struct InstalledProgram {
    pub name: String,
    pub version: Option<String>,
    pub vendor: Option<String>,
}

#[cfg(target_os = "windows")]
pub fn collect() -> Result<Vec<InstalledProgram>, Box<dyn Error>> {
    use winreg::enums::*;
    use winreg::RegKey;

    let mut programs = Vec::new();

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    let uninstall_keys = vec![
        hklm.open_subkey_with_flags(
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
            KEY_READ,
        ),
        hklm.open_subkey_with_flags(
            "SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
            KEY_READ,
        ),
        hkcu.open_subkey_with_flags(
            "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
            KEY_READ,
        ),
    ];

    for key_result in uninstall_keys {
        if let Ok(key) = key_result {
            for subkey_result in key.enum_keys() {
                if let Ok(subkey_name) = subkey_result {
                    if let Ok(subkey) = key.open_subkey(&subkey_name) {
                        let name: Result<String, _> = subkey.get_value("DisplayName");
                        if let Ok(name) = name {
                            let version: Option<String> = subkey.get_value("DisplayVersion").ok();
                            let vendor: Option<String> = subkey.get_value("Publisher").ok();

                            programs.push(InstalledProgram {
                                name,
                                version,
                                vendor,
                            });
                        }
                    }
                }
            }
        }
    }

    Ok(programs)
}


#[cfg(target_os = "linux")]
pub fn collect() -> Result<Vec<InstalledProgram>, Box<dyn Error>> {
    use std::process::Command;

    let output = Command::new("bash")
        .arg("-c")
        .arg("dpkg-query -W -f='${Package}\t${Version}\t${Maintainer}\n' 2>/dev/null || rpm -qa --qf '%{NAME}\t%{VERSION}\t%{VENDOR}\n'")
        .output()?;

    if !output.status.success() {
        return Err("dpkg-query veya rpm başarısız oldu".into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut programs = Vec::new();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.trim().split('\t').collect();
        if parts.len() >= 1 {
            programs.push(InstalledProgram {
                name: parts.get(0).unwrap_or(&"unknown").to_string(),
                version: parts.get(1).map(|s| s.to_string()),
                vendor: parts.get(2).map(|s| s.to_string()),
            });
        }
    }

    Ok(programs)
}

#[cfg(target_os = "macos")]
pub fn collect() -> Result<Vec<InstalledProgram>, Box<dyn Error>> {
    use std::process::Command;

    let output = Command::new("system_profiler")
        .arg("SPApplicationsDataType")
        .output()?;

    if !output.status.success() {
        return Err("system_profiler başarısız oldu".into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut programs = Vec::new();
    let mut current = InstalledProgram {
        name: "".to_string(),
        version: None,
        vendor: None,
    };

    for line in stdout.lines() {
        let line = line.trim();

        if line.starts_with("Name:") {
            current.name = line.replacen("Name:", "", 1).trim().to_string();
        } else if line.starts_with("Version:") {
            current.version = Some(line.replacen("Version:", "", 1).trim().to_string());
        } else if line.starts_with("Obtained from:") {
            current.vendor = Some(line.replacen("Obtained from:", "", 1).trim().to_string());
        } else if line.is_empty() && !current.name.is_empty() {
            programs.push(current.clone());
            current = InstalledProgram {
                name: "".to_string(),
                version: None,
                vendor: None,
            };
        }
    }

    Ok(programs)
}

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
pub fn collect() -> Result<Vec<InstalledProgram>, Box<dyn Error>> {
    Ok(vec![])
}
