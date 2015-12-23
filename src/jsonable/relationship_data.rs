#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct RelationshipData {
  pub _type: String,
  pub id:   i32,
}

impl RelationshipData {
  fn new() -> RelationshipData {
    RelationshipData {_type: "".to_string(), id: 0}
  }
}
