use to_json_api::ToJsonApi;
use to_json_api::resource_object::ResourceObject;

#[derive(Debug, Serialize, Deserialize)]
pub struct Document<T: ToJsonApi, U: ToJsonApi> {
  pub data: Vec<ResourceObject<T>>,
  pub included: Option<Vec<ResourceObject<U>>>,
}
