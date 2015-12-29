mod resource_object;
mod relationship_object;
mod document;
pub mod to_resource_object;
pub mod to_json_string;

use self::relationship_object::RelationshipObject;
use self::resource_object::ResourceObject;
use self::document::Document;
use self::to_resource_object::ToResourceObject;
use self::to_json_string::ToJsonString;
use models::user::User;
use models::post::Post;
use serde_json;

pub trait ToJsonApi {
  fn serialize(&self) -> String;
}

impl<T> ToJsonApi for Vec<T> where T: ToResourceObject {
  fn serialize(&self) -> String {
    let json_data: Vec<ResourceObject<&T>> = self.into_iter().map(|p| p.to_resource_object()).collect();
    let cd = Document {
      data: json_data,
      included: None::<Vec<ResourceObject<&T>>>,
    };
    cd.to_json_string()
  }
}

impl ToJsonApi for Vec<(User, Option<Post>)> {
  fn serialize(&self) -> String {
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
    cd.to_json_string()
  }
}
