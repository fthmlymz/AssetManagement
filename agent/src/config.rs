use serde::Deserialize;
use std::path::Path;
use std::error::Error;

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct Config {
    pub server_url: String,
    //pub grpc_url: String,
    //pub websocket_url: String,
    pub agent_id: String,
    pub interval_seconds: u64,
    pub auth_token: Option<String>,
}

pub async fn load_config() -> Result<Config, Box<dyn Error>> {
    let path = Path::new("config/agent_config.yaml");

    if !path.exists() {
        return Err("❌ config/agent_config.yaml bulunamadı.".into());
    }

    let contents = tokio::fs::read_to_string(path).await?;
    let config: Config = serde_yaml::from_str(&contents)?;

    Ok(config)
}
