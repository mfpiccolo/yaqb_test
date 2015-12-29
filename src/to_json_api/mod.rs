pub mod resource_object;
mod relationship_object;
mod document;
pub mod to_resource_object;

use self::relationship_object::RelationshipObject;
use self::resource_object::ResourceObject;
use self::document::Document;
use self::to_resource_object::ToResourceObject;
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
    serde_json::to_string(&cd).unwrap()
  }
}

impl<T, U> ToJsonApi for Vec<(T, Option<U>)>
where T: ToResourceObject + Default, U: ToResourceObject
{
  fn serialize(&self) -> String {
    let mut current_record: &T = &Default::default();
    let mut json_data: Vec<ResourceObject<&T>> = vec!();
    let mut current_json_data = current_record.to_resource_object();
    let mut included: Vec<ResourceObject<&U>> = vec!();

    for record_tuple in self {
      let top_level_record = &record_tuple.0;
      let nested_record = &record_tuple.1;
      let relationship = match *nested_record {
        Some(ref nr) => {
          included.push(nr.to_resource_object());
          Some(RelationshipObject { _type: nr._type(), id: nr.id() })
        },
        None => None,
      };

      if current_record.id() != top_level_record.id() {
        if current_record.id() != -1 {
          json_data.push(current_json_data.clone());
        }
        current_record = top_level_record;
        current_json_data = top_level_record.to_resource_object();
      }
      current_json_data.relationships.push(relationship);
    }

    let cd = Document {
      data: json_data,
      included: Some(included),
    };
    serde_json::to_string(&cd).unwrap()
  }
}
