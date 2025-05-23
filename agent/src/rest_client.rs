use crate::config::Config;
use crate::system_info::SystemInfo;
use reqwest::Client;
use std::error::Error;
use log::{info, error};

pub async fn send_data(config: &Config, sys_info: &SystemInfo) -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let request_url = format!("{}/api/agent/data", config.server_url);

    let mut request = client
        .post(&request_url)
        .json(&sys_info);

    // Authorization header (opsiyonel)
    if let Some(token) = &config.auth_token {
        request = request.bearer_auth(token);
    }

    let response = request.send().await?;

    if response.status().is_success() {
        info!("✅ Sistem bilgisi başarıyla gönderildi.");
    } else {
        error!("❌ Veri gönderimi başarısız. Durum: {}", response.status());
    }

    Ok(())
}
