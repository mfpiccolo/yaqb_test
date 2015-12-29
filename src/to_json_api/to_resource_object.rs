use serde::ser::Serialize;
use to_json_api::resource_object::ResourceObject;
use models::user::User;
use models::post::Post;

pub trait ToResourceObject where Self: Serialize + Sized {
  fn to_resource_object(&self) -> ResourceObject<&Self> {
    ResourceObject {
      _type: self._type(),
      id:   self.id(),
      attributes: self,
      links: "https://somewhere.com/".to_string(),
      relationships: vec!(),
    }
  }

  fn id(&self) -> i32;
  fn _type(&self) -> String;
}

impl ToResourceObject for User {
  fn id(&self) -> i32 {
    self.id
  }

  fn _type(&self) -> String {
    "users".to_string()
  }
}

impl ToResourceObject for Post {
  fn id(&self) -> i32 {
    self.id
  }

  fn _type(&self) -> String {
    "posts".to_string()
  }
}

