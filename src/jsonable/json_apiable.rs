use jsonable::Jsonable;
use jsonable::resource_object::ResourceObject;
use models::user::User;
use models::post::Post;

pub trait JsonApiable where Self: Jsonable + Sized {
  fn new() -> Self;

  fn to_json_api(&self) -> ResourceObject<&Self>;
}

impl JsonApiable for User {
  fn new() -> Self {
    User {id: -1, name: "".to_string(), email: None}
  }

  fn to_json_api(&self) -> ResourceObject<&Self> {
    ResourceObject {
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

  fn to_json_api(&self) -> ResourceObject<&Post> {
    ResourceObject {
      _type: "posts".to_string(),
      id:   self.id,
      attributes: self,
      links: "https://somewhere.com/".to_string(),
      relationships: vec!(),
    }
  }
}
