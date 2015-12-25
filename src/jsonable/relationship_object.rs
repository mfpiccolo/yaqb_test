#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipObject {
  pub _type: String,
  pub id:   i32,
}

impl RelationshipObject {
  fn new() -> RelationshipObject {
    RelationshipObject {_type: "".to_string(), id: 0}
  }
}
