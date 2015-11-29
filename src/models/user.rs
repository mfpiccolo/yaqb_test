use yaqb::*;
use rustc_serialize::json;
use models::post::{ posts, Post };

table! {
  users {
    id -> Serial,
    name -> VarChar,
    email -> Nullable<VarChar>,
  }
}

#[derive(PartialEq, Eq, Debug, Clone, Queriable, RustcEncodable)]
#[has_many(posts)]
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
    json::encode(self).unwrap()
  }

  pub fn posts_vec(&self) -> Vec<Post> {
    Post::belonging_to(self).load(&User::conn()).unwrap().collect()
  }

}

#[derive(PartialEq, Eq, Debug, Clone, Queriable, RustcDecodable)]
#[insertable_into(users)]
#[changeset_for(users)]
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
