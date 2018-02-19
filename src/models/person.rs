// #[macro_use]
extern crate serde_derive;
// #[macro_use]
extern crate serde_json;
use chrono::DateTime;
use chrono::offset::Utc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
  pub id: Uuid,
  pub name: String,
  pub data: Option<serde_json::Value>,
  pub previous_time: DateTime<Utc>,
}
