use serde::Serialize;
use std::error::Error;

#[derive(Debug, Serialize, Clone)]
pub struct MonitorInfo {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub refresh_rate: Option<u32>,
    pub primary: bool,
}

#[cfg(target_os = "windows")]
pub fn collect() -> Result<Vec<MonitorInfo>, Box<dyn Error>> {
    use std::mem::{size_of, zeroed};
    use std::ptr::null_mut;

    use winapi::shared::windef::{HMONITOR, HDC, LPRECT};
    use winapi::um::winuser::{
        EnumDisplayMonitors, GetMonitorInfoW, MONITORINFOEXW, 
    };

    extern "system" fn monitor_enum_proc(
        hmonitor: HMONITOR,
        _: HDC,
        _: LPRECT,
        lparam: isize,
    ) -> i32 {
        unsafe {
            let monitors = &mut *(lparam as *mut Vec<MonitorInfo>);
            let mut info: MONITORINFOEXW = zeroed();
            info.cbSize = size_of::<MONITORINFOEXW>() as u32;

            if GetMonitorInfoW(hmonitor, &mut info as *mut _ as *mut _) != 0 {
                let rect = info.rcMonitor;
                let width = (rect.right - rect.left) as u32;
                let height = (rect.bottom - rect.top) as u32;
                let name = "Generic Monitor".to_string(); // Model is not easily accessible without EDID
                let is_primary = info.dwFlags & 1 != 0;

                monitors.push(MonitorInfo {
                    name,
                    width,
                    height,
                    refresh_rate: None,
                    primary: is_primary,
                });
            }

            1
        }
    }

    let mut monitors = Vec::new();
    let monitors_ptr = &mut monitors as *mut _ as isize;

    unsafe {
        EnumDisplayMonitors(null_mut(), null_mut(), Some(monitor_enum_proc), monitors_ptr);
    }

    Ok(monitors)
}

#[cfg(target_os = "linux")]
pub fn collect() -> Result<Vec<MonitorInfo>, Box<dyn Error>> {
    use std::process::Command;

    let output = Command::new("xrandr")
        .arg("--current")
        .output()?;

    if !output.status.success() {
        return Err("xrandr komutu başarısız oldu".into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut monitors = Vec::new();

    for line in stdout.lines() {
        if line.contains(" connected") {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            let name = tokens[0].to_string();
            let primary = tokens.contains(&"primary");

            let resolution = tokens.iter()
                .find(|s| s.contains('+') && s.contains('x'))
                .unwrap_or(&"0x0+0+0")
                .split('+')
                .next()
                .unwrap_or("0x0");

            let parts: Vec<&str> = resolution.split('x').collect();
            let width = parts.get(0).and_then(|s| s.parse().ok()).unwrap_or(0);
            let height = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);

            monitors.push(MonitorInfo {
                name,
                width,
                height,
                refresh_rate: None,
                primary,
            });
        }
    }

    Ok(monitors)
}

#[cfg(target_os = "macos")]
pub fn collect() -> Result<Vec<MonitorInfo>, Box<dyn Error>> {
    use core_graphics::display::{CGDisplay, CGDisplayMode};

    let active_displays = CGDisplay::active_displays()?;
    let mut monitors = Vec::new();

    for display_id in active_displays {
        let mode: CGDisplayMode = CGDisplay::display_mode(display_id)
            .ok_or("Ekran modu alınamadı")?;

        monitors.push(MonitorInfo {
            name: format!("Display-{}", display_id),
            width: mode.width() as u32,
            height: mode.height() as u32,
            refresh_rate: Some(mode.refresh_rate() as u32),
            primary: display_id == CGDisplay::main().id(),
        });
    }

    Ok(monitors)
}

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
pub fn collect() -> Result<Vec<MonitorInfo>, Box<dyn Error>> {
    Ok(vec![])
}
