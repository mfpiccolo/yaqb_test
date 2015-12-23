mod json_api_data;
mod relationship_data;
mod compound_document;
mod json_apiable;

use rustc_serialize::json;
use self::relationship_data::RelationshipData;
use self::json_api_data::JsonApiData;
use self::compound_document::CompoundDocument;
use self::json_apiable::JsonApiable;
use models::user::User;
use models::post::Post;
use std::fmt::Debug;
use rustc_serialize::Encodable;

pub trait Jsonable {
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

impl<'a> Jsonable for &'a User {
  fn to_json(&self) -> String {
    json::encode(self).unwrap()
  }
}

impl<'a> Jsonable for &'a Post {
  fn to_json(&self) -> String {
    json::encode(self).unwrap()
  }
}

impl<T: Jsonable + Encodable> Jsonable for JsonApiData<T> {
  fn to_json(&self) -> String {
    json::encode(self).unwrap()
  }
}

impl Jsonable for RelationshipData {
  fn to_json(&self) -> String {
    json::encode(self).unwrap()
  }
}

impl<'a> Jsonable for CompoundDocument<&'a User, &'a Post> {
  fn to_json(&self) -> String {
    json::encode(self).unwrap()
  }
}

impl<T> Jsonable for Vec<T> where T: Jsonable {
  fn to_json(&self) -> String {
    let vec_strings: Vec<String> = self.into_iter().map(|p| p.to_json()).collect();
    json::encode(&vec_strings).unwrap()
  }
}

impl Jsonable for Vec<(User, Option<Post>)> {
  fn to_json(&self) -> String {
    let mut current_user = &User::new();
    let mut relationships: Vec<RelationshipData> = vec!();
    let mut json_data: Vec<JsonApiData<&User>> = vec!();
    let mut current_json_data = current_user.to_json_api();
    let mut included: Vec<JsonApiData<&Post>> = vec!();

    for user_post in self {
      let user = &user_post.0;
      let post = &user_post.1;
      let relationship = match *post {
        Some(ref p) => {
          included.push(p.to_json_api());
          Some(RelationshipData { _type: "posts".to_string(), id: p.id })
        },
        None => None,
      };

      if current_user.id != user.id {
        if current_user.id != -1 {
          json_data.push(current_json_data.clone());
        }
        current_user = user;
        current_json_data = user.to_json_api();
      }
      current_json_data.relationships.push(relationship);
    }

    let cd = CompoundDocument {
      data: json_data,
      included: included,
    };
    cd.to_json()
  }
}
