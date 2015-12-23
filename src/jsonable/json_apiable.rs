use jsonable::Jsonable;
use jsonable::json_api_data::JsonApiData;
use models::user::User;
use models::post::Post;

pub trait JsonApiable where Self: Jsonable + Sized {
  fn new() -> Self;

  fn to_json_api(&self) -> JsonApiData<&Self>;
}

impl JsonApiable for User {
  fn new() -> Self {
    User {id: -1, name: "".to_string(), email: None}
  }

  fn to_json_api(&self) -> JsonApiData<&Self> {
    JsonApiData {
      _type: "users".to_string(),
      id:   self.id,
      attributes: self,
      links: "https://somewhere.com/".to_string(),
      relationships: vec!(),
    }
  }
}

impl JsonApiable for Post {
  fn new() -> Self {
    Post {
      id: -1,
      user_id: -1,
      title: "".to_string(),
      body: None,
    }
  }

  fn to_json_api(&self) -> JsonApiData<&Post> {
    JsonApiData {
      _type: "posts".to_string(),
      id:   self.id,
      attributes: self,
      links: "https://somewhere.com/".to_string(),
      relationships: vec!(),
    }
  }
}
