use yaqb::*;
use rustc_serialize::json;
use models::post::{ posts, Post };
use self::users::dsl::*;
use yaqb::query_builder::*;

table! {
  users {
    id -> Serial,
    name -> VarChar,
    email -> Nullable<VarChar>,
  }
}

#[derive(PartialEq, Eq, Debug, Clone, Queriable, RustcEncodable)]
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

  pub fn find(_id: i32) -> Option<User> {
    User::conn().find(users, _id).unwrap()
  }

  pub fn count() -> Option<i64> {
    let select_count = users.select_sql::<types::BigInt>("COUNT(*)");
    User::conn().query_one::<_, i64>(select_count.clone()).unwrap()
  }

  pub fn insert(new_users: Vec<NewUser>) -> Vec<User> {
    User::conn().insert(&users, &new_users).unwrap().collect()
  }

  pub fn to_json(&self) -> String {
    json::encode(self).unwrap()
  }

  pub fn posts_vec(&self) -> Vec<Post> {
    Post::belonging_to(self).load(&User::conn()).unwrap().collect()
  }

  pub fn update(_id: i32, changed_user: NewUser) {
    let command = update(users::table.filter(id.eq(_id))).set(changed_user);
    User::conn().execute_returning_count(&command).unwrap();
  }

}

#[derive(PartialEq, Eq, Debug, Clone, Queriable, RustcDecodable)]
#[insertable_into(users)]
pub struct NewUser {
  pub name: String,
  pub email: Option<String>,
}

impl AsChangeset for NewUser {
  type Changeset = Vec<Box<Changeset<Target=users::table>>>;

  fn as_changeset(self) -> Self::Changeset {
    let mut changes: Vec<Box<Changeset<Target=users::table>>> = Vec::new();

    let _name = self.name;
    changes.push(Box::new(
        users::name.eq(_name).as_changeset()
    ));

    if let Some(_email) = self.email {
      changes.push(Box::new(
          users::email.eq(_email).as_changeset()
      ))
    }

    changes
  }
}

impl NewUser {
  pub fn new(_name: &str, _email: Option<&str>) -> Self {
    NewUser {
      name: _name.to_string(),
      email: _email.map(|s| s.to_string()),
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
