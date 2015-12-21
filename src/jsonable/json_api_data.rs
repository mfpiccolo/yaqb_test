use jsonable::Jsonable;
use jsonable::relationship_data::RelationshipData;

#[derive(Debug, Clone)]
pub struct JsonApiData<T: Jsonable> {
  pub record:        T,
  pub _type:         String,
  pub id:            i32,
  pub attributes:    String,
  pub links:         String,
  pub relationships: Vec<Option<RelationshipData>>,
}
