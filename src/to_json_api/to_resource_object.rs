use to_json_api::ToJsonApi;
use to_json_api::resource_object::ResourceObject;
use models::user::User;
use models::post::Post;

pub trait ToResourceObject where Self: ToJsonApi + Sized {
  fn to_resource_object(&self) -> ResourceObject<&Self>;
}

impl ToResourceObject for User {
  fn to_resource_object(&self) -> ResourceObject<&Self> {
    ResourceObject {
      _type: "users".to_string(),
      id:   self.id,
      attributes: self,
      links: "https://somewhere.com/".to_string(),
      relationships: vec!(),
    }
  }
}

impl ToResourceObject for Post {
  fn to_resource_object(&self) -> ResourceObject<&Post> {
    ResourceObject {
      _type: "posts".to_string(),
      id:   self.id,
      attributes: self,
      links: "https://somewhere.com/".to_string(),
      relationships: vec!(),
    }
  }
}

