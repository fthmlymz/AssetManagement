use serde::{Deserialize, Serialize};

//
// CVE VERİ MODELİ
//

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CveItem {
    pub cve: CveData,
    #[serde(rename = "published")]
    pub published_date: Option<String>,
    #[serde(rename = "lastModified")]
    pub last_modified_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CveData {
    #[serde(rename = "id")]
    pub cve_id: String,
    #[serde(default)]
    pub source_identifier: Option<String>,
    #[serde(default)]
    pub descriptions: Vec<CveDescription>,
    #[serde(default)]
    pub metrics: Option<serde_json::Value>, // CVSS detayları
    #[serde(default)]
    pub weaknesses: Option<serde_json::Value>, // CWE listesi
    #[serde(default)]
    pub references: Option<serde_json::Value>,
    #[serde(default)]
    pub configurations: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CveDescription {
    #[serde(rename = "lang")]
    pub lang: String,
    #[serde(rename = "value")]
    pub value: String,
}


//
// CPE VERİ MODELİ
//

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpeItem {
    #[serde(rename = "cpeName")]
    pub cpe_name: String,

    #[serde(default)]
    pub titles: Vec<CpeTitle>,

    #[serde(default)]
    pub deprecated: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CpeTitle {
    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "lang")]
    pub lang: String,
}
