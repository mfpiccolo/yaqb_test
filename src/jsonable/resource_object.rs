#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceObject {
  pub _type: String,
  pub id:   i32,
}

impl ResourceObject {
  fn new() -> ResourceObject {
    ResourceObject {_type: "".to_string(), id: 0}
  }
}
