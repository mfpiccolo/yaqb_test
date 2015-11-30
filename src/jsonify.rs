use rustc_serialize::json;
use rustc_serialize::Encodable;
use models::user::User;
use models::post::Post;

pub trait Jsonable: Encodable {
  fn to_json(&self) -> String;
}

impl Jsonable for User {
  fn to_json(&self) -> String {
    json::encode(self).unwrap()
  }
}

impl Jsonable for Post {
  fn to_json(&self) -> String {
    json::encode(self).unwrap()
  }
}

pub trait Jsonify {
  type Target: IntoIterator;

  fn to_json(&self) -> String;
}

impl<T> Jsonify for Vec<T> where T: Jsonable {
  type Target = Self;

  fn to_json(&self) -> String {
    let vec_strings: Vec<String> = self.into_iter().map(|p| p.to_json()).collect();
    json::encode(&vec_strings).unwrap()
  }
}
