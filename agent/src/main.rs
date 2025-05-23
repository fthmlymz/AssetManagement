mod config;
mod system_info;

use system_info::{base, network, bios, users, hotfix, monitor, programs};
use log::info;
use tokio::signal;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    info!("🟢 Agent başlatılıyor...");

    let config = config::load_config().await?;
    info!("✅ Konfigürasyon yüklendi: {:?}", config);

    let sys_info = base::collect().await?;
    let net_info = network::collect()?;
    let bios_info = bios::collect()?;
    let user_info = users::collect()?;
    let hotfixes = hotfix::collect()?;
    let monitors = monitor::collect()?;
    let installed_programs = programs::collect()?;

    println!("📋 Sistem Bilgisi:\n{:#?}", sys_info);
    println!("🌐 Ağ Arayüzleri:\n{:#?}", net_info);
    println!("🧬 BIOS Bilgisi:\n{:#?}", bios_info);
    println!("👤 Oturum Açmış Kullanıcılar:\n{:#?}", user_info);
    println!("🩹 Yüklü Hotfix Güncellemeleri:\n{:#?}", hotfixes);
    println!("🖥️ Monitörler:\n{:#?}", monitors);
    println!("📦 Yüklü Programlar:\n{:#?}", installed_programs);

    signal::ctrl_c().await?;
    info!("🛑 Agent kapatılıyor...");
    Ok(())
}
