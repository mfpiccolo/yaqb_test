use jsonable::Jsonable;
use jsonable::resource_object::ResourceObject;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompoundDocument<T: Jsonable, U: Jsonable> {
  pub data: Vec<ResourceObject<T>>,
  pub included: Vec<ResourceObject<U>>,
}
