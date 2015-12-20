use rustc_serialize::json;
use models::user::User;
use models::post::Post;
use std::fmt::Debug;

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
    let mut json_data: Vec<JsonApiData> = vec!();
    let mut current_json_data = JsonApiData::new();

    for user_post in self {
      let user = &user_post.0;
      let post = &user_post.1;
      let relationship = match *post {
        Some(ref p) => Some(RelationshipData { _type: "posts".to_string(), id: p.id }),
        None => None,
      };
      if current_user.id != user.id {
        if current_user.id != -1 {
          json_data.push(current_json_data.clone());
        }
        current_user = user;
        current_json_data = JsonApiData::build(user);
      }
      current_json_data.relationships.push(relationship);
    }

    println!("{:?}", json_data);
    "slkjf".to_string()
  }
}

#[derive(Debug, Clone)]
pub struct RelationshipData {
  pub _type: String,
  pub id:   i32,
}

impl RelationshipData {
  fn new() -> RelationshipData {
    RelationshipData {_type: "".to_string(), id: 0}
  }
}

#[derive(Debug, Clone)]
pub struct JsonApiData {
  pub _type:         String,
  pub id:            i32,
  pub attributes:    String,
  pub links:         String,
  pub relationships: Vec<Option<RelationshipData>>,
}

impl JsonApiData {
  fn new() -> JsonApiData {
    JsonApiData {
      _type:         "".to_string(),
      id:            0,
      attributes:    "".to_string(),
      links:         "".to_string(),
      relationships: vec!(None),
    }
  }

  fn build(u: &User) -> JsonApiData {
    JsonApiData {
      _type: "users".to_string(),
      id:   u.id,
      attributes: u.to_json(),
      links: "https://somewhere.com/".to_string(),
      relationships: vec!(),
    }
  }
}

trait JsonApiable {
  fn new() -> Self;
}

impl JsonApiable for User {
  fn new() -> User {
    User {id: -1, name: "".to_string(), email: None}
  }
}

impl JsonApiable for Post {
  fn new() -> Post {
    Post {
      id: -1,
      user_id: -1,
      title: "".to_string(),
      body: None,
    }
  }
}
