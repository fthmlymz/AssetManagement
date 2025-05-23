use sysinfo::{System, RefreshKind, CpuRefreshKind, MemoryRefreshKind, Disks};
use serde::Serialize;
use std::error::Error;

#[derive(Debug, Serialize)]
pub struct SystemInfo {
    pub os_name: String,
    pub hostname: String,
    pub total_memory_mb: u64,
    pub used_memory_mb: u64,
    pub cpu_brand: String,
    pub cpu_usage_percent: f32,
    pub total_disk_gb: u64,
    pub used_disk_gb: u64,
}

pub async fn collect() -> Result<SystemInfo, Box<dyn Error>> {
    let refresh_kind = RefreshKind::everything()
        .with_memory(MemoryRefreshKind::everything())
        .with_cpu(CpuRefreshKind::everything());

    let mut sys = System::new_with_specifics(refresh_kind);
    sys.refresh_all();

    let os_name = get_os_name();
    let hostname = get_hostname();

    let total_memory_mb = sys.total_memory() / 1024;
    let used_memory_mb = sys.used_memory() / 1024;

    let cpus = sys.cpus();
    let cpu_brand = if !cpus.is_empty() {
        cpus[0].brand().to_string()
    } else {
        "Unknown".to_string()
    };
    let cpu_usage_percent = if !cpus.is_empty() {
        cpus[0].cpu_usage()
    } else {
        0.0
    };

    let mut total_disk_gb = 0;
    let mut used_disk_gb = 0;

    let disks = Disks::new_with_refreshed_list();
    for disk in disks.list() {
        total_disk_gb += disk.total_space() / 1024 / 1024 / 1024;
        used_disk_gb += (disk.total_space() - disk.available_space()) / 1024 / 1024 / 1024;
    }

    Ok(SystemInfo {
        os_name,
        hostname,
        total_memory_mb,
        used_memory_mb,
        cpu_brand,
        cpu_usage_percent,
        total_disk_gb,
        used_disk_gb,
    })
}

fn get_os_name() -> String {
    match System::long_os_version() {
        Some(v) => v,
        None => {
            #[cfg(target_os = "windows")]
            { "Windows".to_string() }

            #[cfg(target_os = "linux")]
            { "Linux".to_string() }

            #[cfg(target_os = "macos")]
            { "macOS".to_string() }

            #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
            { "Unknown OS".to_string() }
        }
    }
}

fn get_hostname() -> String {
    match System::host_name() {
        Some(name) => name,
        None => {
            #[cfg(target_os = "windows")]
            { std::env::var("COMPUTERNAME").unwrap_or("Unknown".to_string()) }

            #[cfg(target_os = "linux")]
            { std::fs::read_to_string("/etc/hostname").unwrap_or("Unknown".to_string()).trim().to_string() }

            #[cfg(target_os = "macos")]
            {
                use std::process::Command;
                let output = Command::new("scutil").arg("--get").arg("ComputerName").output();
                match output {
                    Ok(o) => String::from_utf8_lossy(&o.stdout).trim().to_string(),
                    Err(_) => "Unknown".to_string(),
                }
            }

            #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
            { "Unknown".to_string() }
        }
    }
}
