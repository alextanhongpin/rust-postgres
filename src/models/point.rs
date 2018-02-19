// #[macro_use]
extern crate serde_derive;
// #[macro_use]
extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
  pub x: i32,
  pub y: i32,
}
