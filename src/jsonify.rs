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
    println!("{:?}", cd);
    "slkjf".to_string()
  }
}
#[derive(Debug)]
pub struct CompoundDocument<T: Jsonable, U: Jsonable> {
  data: Vec<JsonApiData<T>>,
  included: Vec<JsonApiData<U>>,
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
pub struct JsonApiData<T: Jsonable> {
  pub record:        T,
  pub _type:         String,
  pub id:            i32,
  pub attributes:    String,
  pub links:         String,
  pub relationships: Vec<Option<RelationshipData>>,
}

trait JsonApiable where Self: Jsonable + Sized {
  fn new() -> Self;

  fn to_json_api(&self) -> JsonApiData<&Self>;
}

impl JsonApiable for User {
  fn new() -> Self {
    User {id: -1, name: "".to_string(), email: None}
  }

  fn to_json_api(&self) -> JsonApiData<&Self> {
    JsonApiData {
      record: self,
      _type: "users".to_string(),
      id:   self.id,
      attributes: self.to_json(),
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
      record: self,
      _type: "posts".to_string(),
      id:   self.id,
      attributes: self.to_json(),
      links: "https://somewhere.com/".to_string(),
      relationships: vec!(),
    }
  }
}
