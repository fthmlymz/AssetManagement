//mod config;
mod nist;
mod models;
mod elastic;

use dotenv::dotenv;
use log::{info, error};

use crate::nist::{NistApi, NistApiVersion, NistDataType};
use crate::models::{CveItem, CpeItem};
use crate::elastic::{create_client, bulk_save_cves, bulk_save_cpes};

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    info!("🚀 NIST CVE/CPE Collector başlatıldı");

    let api = NistApi::new();
    let es_client = create_client().await;

    let versions = vec![
        NistApiVersion::V2,
        NistApiVersion::V31,
        NistApiVersion::V4,
    ];

    let data_types = vec![
        NistDataType::CVE,
        NistDataType::CPE,
    ];

    for version in versions {
        for data_type in &data_types {
            let label = format!("{:?}-{}", data_type, version.as_str());
            info!("📡 Veri alınıyor: {}", label);

            match api.fetch_data((*data_type).clone(), version.clone(), 10, 0).await {
                Ok(entries) => {
                    info!("✔ {} kayıt alındı [{}]", entries.len(), label);

                    match data_type {
                        NistDataType::CVE => {
                            let mut cve_models = Vec::new();

                            for entry in entries {
                                match serde_json::from_value::<CveItem>(entry.clone()) {
                                    Ok(model) => cve_models.push(model),
                                    Err(e) => error!("CVE parse hatası: {}", e),
                                }
                            }

                            if let Err(e) = bulk_save_cves(&es_client, &cve_models).await {
                                error!("❌ CVE bulk kayıt hatası: {}", e);
                            }
                        }

                        NistDataType::CPE => {
                            let mut cpe_models = Vec::new();

                            for entry in entries {
                                match serde_json::from_value::<CpeItem>(entry.clone()) {
                                    Ok(model) => cpe_models.push(model),
                                    Err(e) => error!("CPE parse hatası: {}", e),
                                }
                            }

                            if let Err(e) = bulk_save_cpes(&es_client, &cpe_models).await {
                                error!("❌ CPE bulk kayıt hatası: {}", e);
                            }
                        }
                    }
                }

                Err(e) => {
                    error!("❌ {} verisi alınamadı: {}", label, e);
                }
            }
        }
    }
}
