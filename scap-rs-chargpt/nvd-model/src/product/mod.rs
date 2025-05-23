use chrono::NaiveDateTime;
#[cfg(feature = "db")]
use diesel::{Associations, Identifiable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
#[cfg(feature = "openapi")]
use utoipa::{IntoParams, ToSchema};
#[cfg(feature = "yew")]
use yew::Properties;

#[cfg(feature = "db")]
use crate::schema::products;
use crate::types::{uuid_serde, AnyValue, MetaData};
use crate::vendor::Vendor;

#[cfg(feature = "db")]
pub mod db;

#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[cfg_attr(feature = "db",
derive(Queryable, Identifiable, Associations, Selectable),
diesel(table_name = products, belongs_to(Vendor)))]
#[cfg_attr(feature = "yew", derive(Properties))]
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Product {
  #[serde(with = "uuid_serde")]
  pub id: Vec<u8>,
  #[serde(with = "uuid_serde")]
  pub vendor_id: Vec<u8>,
  pub official: i16,
  pub part: String,
  pub name: String,
  pub description: String,
  pub meta: AnyValue<MetaData>,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

#[cfg_attr(feature = "yew", derive(Properties))]
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProductWithVendor {
  pub product: Product,
  pub vendor: Vendor,
}

#[cfg_attr(feature = "openapi", derive(IntoParams))]
// 产品查询参数
#[cfg_attr(feature = "yew", derive(Properties))]
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct QueryProduct {
  pub vendor_id: Option<String>,
  pub vendor_name: Option<String>,
  pub name: Option<String>,
  pub part: Option<String>,
  pub official: Option<i16>,
  pub size: Option<i64>,
  pub page: Option<i64>,
}
