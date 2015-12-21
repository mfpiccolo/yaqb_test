use jsonable::Jsonable;
use jsonable::json_api_data::JsonApiData;

#[derive(Debug)]
pub struct CompoundDocument<T: Jsonable, U: Jsonable> {
  pub data: Vec<JsonApiData<T>>,
  pub included: Vec<JsonApiData<U>>,
}
