extern crate serde;
extern crate serde_json;

use yaqb::*;

table! {
  users {
    id -> Serial,
    name -> VarChar,
    email -> Nullable<VarChar>,
  }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Queriable)]
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

  pub fn insert(new_users: Vec<NewUser>) {
    User::conn().insert_returning_count(&self::users::table, &new_users);
  }

  pub fn to_json(&self) -> String {
    serde_json::to_string(&self).unwrap()
  }

}

#[derive(RustcDecodable, Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Queriable)]
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

insertable! {
  NewUser => users {
    name -> String,
    email -> Option<String>,
  }
}

changeset! {
  NewUser => users {
    name -> String,
    email -> Option<String>,
  }
}

