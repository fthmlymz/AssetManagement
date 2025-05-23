use serde::Serialize;
use std::error::Error;

#[derive(Debug, Serialize, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub mac_address: Option<String>, // Şimdilik None
    pub ipv4: Vec<String>,
    pub ipv6: Vec<String>,
}

#[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
pub fn collect() -> Result<Vec<NetworkInterface>, Box<dyn Error>> {
    use if_addrs::{get_if_addrs, IfAddr};

    let mut interfaces: Vec<NetworkInterface> = Vec::new();

    let if_addrs = get_if_addrs()?;
    for iface in if_addrs {
        let name = iface.name;
        let ip = match iface.addr {
            IfAddr::V4(v4) => v4.ip.to_string(),
            IfAddr::V6(v6) => v6.ip.to_string(),
        };

        let is_ipv6 = ip.contains(':');

        if let Some(pos) = interfaces.iter_mut().position(|i| i.name == name) {
            if is_ipv6 {
                interfaces[pos].ipv6.push(ip);
            } else {
                interfaces[pos].ipv4.push(ip);
            }
        } else {
            let mut new_iface = NetworkInterface {
                name,
                mac_address: None, // if-addrs 0.13.4'te desteklenmiyor
                ipv4: Vec::new(),
                ipv6: Vec::new(),
            };

            if is_ipv6 {
                new_iface.ipv6.push(ip);
            } else {
                new_iface.ipv4.push(ip);
            }

            interfaces.push(new_iface);
        }
    }

    Ok(interfaces)
}

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
pub fn collect() -> Result<Vec<NetworkInterface>, Box<dyn Error>> {
    Ok(vec![]) // Diğer OS'lerde boş veri döner
}
