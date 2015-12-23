use jsonable::Jsonable;
use jsonable::relationship_data::RelationshipData;

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct JsonApiData<T: Jsonable> {
  pub _type:         String,
  pub id:            i32,
  pub attributes:    T,
  pub links:         String,
  pub relationships: Vec<Option<RelationshipData>>,
}
