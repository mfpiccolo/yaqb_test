use to_json_api::ToJsonApi;
use to_json_api::relationship_object::RelationshipObject;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceObject<T: ToJsonApi> {
  pub _type:         String,
  pub id:            i32,
  pub attributes:    T,
  pub links:         String,
  pub relationships: Vec<Option<RelationshipObject>>,
}
