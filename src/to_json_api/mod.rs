mod resource_object;
mod relationship_object;
mod document;
pub mod to_resource_object;

use self::relationship_object::RelationshipObject;
use self::resource_object::ResourceObject;
use self::document::Document;
use self::to_resource_object::ToResourceObject;
use models::user::User;
use models::post::Post;
use serde_json;
use serde::ser::Serialize;

pub trait ToJsonApi {
  fn to_json(&self) -> String;
}

impl ToJsonApi for User {
  fn to_json(&self) -> String {
    serde_json::to_string(self).unwrap()
  }
}

impl ToJsonApi for Post {
  fn to_json(&self) -> String {
    serde_json::to_string(self).unwrap()
  }
}

impl<'a> ToJsonApi for &'a User {
  fn to_json(&self) -> String {
    serde_json::to_string(self).unwrap()
  }
}

impl<'a> ToJsonApi for &'a Post {
  fn to_json(&self) -> String {
    serde_json::to_string(self).unwrap()
  }
}

impl<T: ToJsonApi + Serialize> ToJsonApi for ResourceObject<T> {
  fn to_json(&self) -> String {
    serde_json::to_string(self).unwrap()
  }
}

impl ToJsonApi for RelationshipObject {
  fn to_json(&self) -> String {
    serde_json::to_string(self).unwrap()
  }
}

impl<'a> ToJsonApi for Document<&'a User, &'a Post> {
  fn to_json(&self) -> String {
    serde_json::to_string(self).unwrap()
  }
}

impl<T> ToJsonApi for Vec<T> where T: ToJsonApi {
  fn to_json(&self) -> String {
    let vec_strings: Vec<String> = self.into_iter().map(|p| p.to_json()).collect();
    serde_json::to_string(&vec_strings).unwrap()
  }
}

impl ToJsonApi for Vec<(User, Option<Post>)> {
  fn to_json(&self) -> String {
    let mut current_user = &User {id: -1, name: "".to_string(), email: None};
    let mut json_data: Vec<ResourceObject<&User>> = vec!();
    let mut current_json_data = current_user.to_resource_object();
    let mut included: Vec<ResourceObject<&Post>> = vec!();

    for user_post in self {
      let user = &user_post.0;
      let post = &user_post.1;
      let relationship = match *post {
        Some(ref p) => {
          included.push(p.to_resource_object());
          Some(RelationshipObject { _type: "posts".to_string(), id: p.id })
        },
        None => None,
      };

      if current_user.id != user.id {
        if current_user.id != -1 {
          json_data.push(current_json_data.clone());
        }
        current_user = user;
        current_json_data = user.to_resource_object();
      }
      current_json_data.relationships.push(relationship);
    }

    let cd = Document {
      data: json_data,
      included: Some(included),
    };
    cd.to_json()
  }
}
