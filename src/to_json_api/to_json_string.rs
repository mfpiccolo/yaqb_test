use serde_json;
use serde::ser::Serialize;
use models::user::User;
use models::post::Post;
use to_json_api::relationship_object::RelationshipObject;
use to_json_api::resource_object::ResourceObject;
use to_json_api::document::Document;


pub trait ToJsonString {
  fn to_json_string(&self) -> String;
}

impl ToJsonString for User {
  fn to_json_string(&self) -> String {
    serde_json::to_string(self).unwrap()
  }
}

impl ToJsonString for Post {
  fn to_json_string(&self) -> String {
    serde_json::to_string(self).unwrap()
  }
}

impl<'a> ToJsonString for &'a User {
  fn to_json_string(&self) -> String {
    serde_json::to_string(self).unwrap()
  }
}

impl<'a> ToJsonString for &'a Post {
  fn to_json_string(&self) -> String {
    serde_json::to_string(self).unwrap()
  }
}

impl<T: Serialize> ToJsonString for ResourceObject<T> {
  fn to_json_string(&self) -> String {
    serde_json::to_string(self).unwrap()
  }
}

impl ToJsonString for RelationshipObject {
  fn to_json_string(&self) -> String {
    serde_json::to_string(self).unwrap()
  }
}

impl<'a, T, U> ToJsonString for Document<&'a T, &'a U> where T: Serialize, U: Serialize {
  fn to_json_string(&self) -> String {
    serde_json::to_string(self).unwrap()
  }
}
