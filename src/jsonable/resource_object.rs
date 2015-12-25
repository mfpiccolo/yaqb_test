use jsonable::Jsonable;
use jsonable::relationship_object::RelationshipObject;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceObject<T: Jsonable> {
  pub _type:         String,
  pub id:            i32,
  pub attributes:    T,
  pub links:         String,
  pub relationships: Vec<Option<RelationshipObject>>,
}
