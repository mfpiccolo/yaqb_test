use jsonable::Jsonable;
use jsonable::resource_object::ResourceObject;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonApiData<T: Jsonable> {
  pub _type:         String,
  pub id:            i32,
  pub attributes:    T,
  pub links:         String,
  pub relationships: Vec<Option<ResourceObject>>,
}
