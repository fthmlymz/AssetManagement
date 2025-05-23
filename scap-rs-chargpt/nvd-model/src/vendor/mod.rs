use chrono::NaiveDateTime;
#[cfg(feature = "db")]
use diesel::{Identifiable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
#[cfg(feature = "openapi")]
use utoipa::ToSchema;
#[cfg(feature = "yew")]
use yew::Properties;

#[cfg(feature = "db")]
use crate::schema::vendors;
use crate::types::{uuid_serde, AnyValue, MetaData};

#[cfg(feature = "db")]
pub mod db;

#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "db", derive(Queryable, Identifiable, Selectable), diesel(table_name = vendors))]
#[cfg_attr(feature = "yew", derive(Properties))]
pub struct Vendor {
  #[serde(with = "uuid_serde")]
  pub id: Vec<u8>,
  pub official: i16,
  pub name: String,
  pub description: String,
  pub meta: AnyValue<MetaData>,
  pub updated_at: NaiveDateTime,
  pub created_at: NaiveDateTime,
}
