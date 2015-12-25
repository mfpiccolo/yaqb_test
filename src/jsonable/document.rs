use jsonable::Jsonable;
use jsonable::resource_object::ResourceObject;

#[derive(Debug, Serialize, Deserialize)]
pub struct Document<T: Jsonable, U: Jsonable> {
  pub data: Vec<ResourceObject<T>>,
  pub included: Option<Vec<ResourceObject<U>>>,
}
