extern crate serde;
extern crate serde_json;
use rustc_serialize::json;

use yaqb::*;
use models::post::{posts, Post};

table! {
  users {
    id -> Serial,
    name -> VarChar,
    email -> Nullable<VarChar>,
  }
}

one_to_many!(users (User) -> posts (Post) on (user_id = id));

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Queriable, RustcEncodable)]
pub struct User {
  pub id: i32,
  pub name: String,
  pub email: Option<String>,
}

impl User {

  #[inline]
  pub fn conn() -> Connection {
    let connection_url = ::std::env::var("DATABASE_URL").ok()
      .expect("DATABASE_URL must be set in order to run tests");
    Connection::establish(&connection_url).unwrap()
  }

  pub fn find(id: i32) -> Option<User> {
    User::conn().find(self::users::table, id).unwrap()
  }

  pub fn count() -> Option<i64> {
    let select_count = users::table.select_sql::<types::BigInt>("COUNT(*)");
    User::conn().query_one::<_, i64>(select_count.clone()).unwrap()
  }

  pub fn insert(new_users: Vec<NewUser>) -> Vec<User> {
    User::conn().insert(&self::users::table, &new_users).unwrap().collect()
  }

  pub fn to_json(&self) -> String {
    serde_json::to_string(&self).unwrap()
  }

  pub fn posts_vec(&self) -> Vec<Post> {
    self.posts().load(&User::conn()).unwrap().collect()
  }

}

#[derive(RustcDecodable, Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Queriable)]
#[insertable_into(users)]
pub struct NewUser {
  pub name: String,
  pub email: Option<String>,
}

impl NewUser {
  pub fn new(name: &str, email: Option<&str>) -> Self {
    NewUser {
      name: name.to_string(),
      email: email.map(|s| s.to_string()),
    }
  }
}

pub trait Jsonify {
  fn to_json(&self) -> String;
}

impl Jsonify for Vec<Post> {
  fn to_json(&self) -> String {
    let vec_strings: Vec<String> = self.into_iter().map(|p| p.to_json()).collect();
    json::encode(&vec_strings).unwrap()
  }
}
