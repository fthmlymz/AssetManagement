use elasticsearch::BulkParts;
use elasticsearch::{Elasticsearch, http::transport::Transport};
use elasticsearch::http::request::JsonBody;
use crate::models::{CveItem, CpeItem};
use serde_json::json;
use log::{info, error};

pub async fn bulk_save_cves(client: &Elasticsearch, items: &[CveItem]) -> Result<(), Box<dyn std::error::Error>> {
    if items.is_empty() {
        return Ok(());
    }

    let mut body: Vec<JsonBody<_>> = Vec::new();
    for item in items {
        let id = &item.cve.cve_id;
        body.push(json!({ "index": { "_id": id } }).into());
        body.push(serde_json::to_value(item)?.into());
    }

    let response = client
        .bulk(BulkParts::Index("nist-cves"))
        .body(body)
        .send()
        .await?;

    if response.status_code().is_success() {
        info!("✔ Toplu CVE kaydı tamamlandı: {} kayıt", items.len());
    } else {
        error!("❌ CVE bulk insert başarısız.");
    }

    Ok(())
}

pub async fn bulk_save_cpes(client: &Elasticsearch, items: &[CpeItem]) -> Result<(), Box<dyn std::error::Error>> {
    if items.is_empty() {
        return Ok(());
    }

    let mut body: Vec<JsonBody<_>> = Vec::new();
    for item in items {
        let id = &item.cpe_name;
        body.push(json!({ "index": { "_id": id } }).into());
        body.push(serde_json::to_value(item)?.into());
    }

    let response = client
        .bulk(BulkParts::Index("nist-cpes"))
        .body(body)
        .send()
        .await?;

    if response.status_code().is_success() {
        info!("✔ Toplu CPE kaydı tamamlandı: {} kayıt", items.len());
    } else {
        error!("❌ CPE bulk insert başarısız.");
    }

    Ok(())
}

pub async fn create_client() -> Elasticsearch {
    let es_url = std::env::var("ELASTICSEARCH_URL").unwrap_or_else(|_| "http://localhost:9200".to_string());
    let transport = Transport::single_node(&es_url)
        .expect("Elasticsearch URL geçersiz.");
    Elasticsearch::new(transport)
}