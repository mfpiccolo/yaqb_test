use serde::ser::Serialize;
use to_json_api::resource_object::ResourceObject;

#[derive(Debug, Serialize, Deserialize)]
pub struct Document<T: Serialize, U: Serialize> {
  pub data: Vec<ResourceObject<T>>,
  pub included: Option<Vec<ResourceObject<U>>>,
}
