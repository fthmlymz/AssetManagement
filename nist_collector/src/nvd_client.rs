use serde_json::Value;
use std::env;
use reqwest::Client;

pub struct NvdClient {
    api_key: String,
    client: Client,
}

impl NvdClient {
    pub fn new() -> Self {
        let api_key = env::var("NIST_API_KEY").expect("NIST_API_KEY eksik!");
        NvdClient {
            api_key,
            client: Client::new(),
        }
    }

    pub async fn get_cves(&self, results_per_page: usize, start_index: usize) -> Result<Value, Box<dyn std::error::Error>> {
        let url = "https://services.nvd.nist.gov/rest/json/cves/2.0";

        let response = self.client
            .get(url)
            .query(&[
                ("resultsPerPage", results_per_page.to_string()),
                ("startIndex", start_index.to_string()),
            ])
            .header("apiKey", &self.api_key)
            .send()
            .await?;

        let data = response.json::<Value>().await?;
        Ok(data)
    }
}
