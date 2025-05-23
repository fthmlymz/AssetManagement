use std::env;
use reqwest::Client;
use serde_json::Value;
use log::{info, warn, error};

#[derive(Debug, Clone)]
pub enum NistApiVersion {
    V2,
    V31,
    V4,
}

impl NistApiVersion {
    pub fn as_str(&self) -> &'static str {
        match self {
            NistApiVersion::V2 => "2.0",
            NistApiVersion::V31 => "3.1",
            NistApiVersion::V4 => "4.0",
        }
    }
}

#[derive(Debug, Clone)]
pub enum NistDataType {
    CVE,
    CPE,
}

impl NistDataType {
    pub fn base_url(&self) -> &'static str {
        match self {
            NistDataType::CVE => "https://services.nvd.nist.gov/rest/json/cves",
            NistDataType::CPE => "https://services.nvd.nist.gov/rest/json/cpes",
        }
    }

    pub fn json_key(&self) -> &'static str {
        match self {
            NistDataType::CVE => "vulnerabilities",
            NistDataType::CPE => "products",
        }
    }
}

pub struct NistApi {
    client: Client,
    api_key: Option<String>,
}

impl NistApi {
    pub fn new() -> Self {
        let api_key = env::var("NIST_API_KEY").ok();
        NistApi {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn fetch_data(
        &self,
        data_type: NistDataType,
        version: NistApiVersion,
        results_per_page: usize,
        start_index: usize,
    ) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
        let version_str = version.as_str();

        let mut query: Vec<(&str, String)> = vec![
            ("resultsPerPage", results_per_page.to_string()),
            ("startIndex", start_index.to_string()),
        ];

        if let Some(ref key) = self.api_key {
            query.push(("apiKey", key.to_string()));
        }

        let full_url = format!("{}/{}", data_type.base_url(), version_str);

        info!("İstek gönderiliyor: {}?resultsPerPage={}&startIndex={}", full_url, results_per_page, start_index);

        let response = self.client.get(&full_url)
            .query(&query)
            .send()
            .await?;

        if !response.status().is_success() {
            error!("NIST API başarısız yanıt: {:?}", response.status());
            return Err(format!("API yanıtı başarısız: {}", response.status()).into());
        }

        let json: Value = response.json().await?;
        let key = data_type.json_key();

        match json.get(key) {
            Some(Value::Array(items)) => Ok(items.clone()),
            _ => {
                warn!("Beklenen alan bulunamadı: {}", key);
                Ok(vec![])
            }
        }
    }
}
