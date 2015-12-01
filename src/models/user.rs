use diesel::*;
use models::post::Post;
use self::users::dsl::*;
use diesel::query_builder::*;
use diesel_model::modelable::Modelable;

table! {
  users {
    id -> Serial,
    name -> VarChar,
    email -> Nullable<VarChar>,
  }
}

#[derive(PartialEq, Eq, Debug, Clone, Queriable, RustcEncodable, Modelable)]
pub struct User {
  pub id: i32,
  pub name: String,
  pub email: Option<String>,
}

impl User {

  pub fn find(_id: i32) -> User {
    User::conn().find(users, _id).unwrap()
  }

  pub fn count() -> i64 {
    let select_count = users.select_sql::<types::BigInt>("COUNT(*)");
    User::conn().query_one::<_, i64>(select_count.clone()).unwrap()
  }

  pub fn insert(new_users: Vec<NewUser>) -> Vec<User> {
    User::conn().insert(&users, &new_users).unwrap().collect()
  }

  pub fn update(_id: i32, changed_user: NewUser) -> User {
    let command = update(users::table.filter(id.eq(_id))).set(changed_user);
    User::conn().query_one(command).unwrap()
  }

  pub fn posts_vec(&self) -> Vec<Post> {
    Post::belonging_to(self).load(&User::conn()).unwrap().collect()
  }

}

#[derive(PartialEq, Eq, Debug, Clone, Queriable, RustcDecodable)]
#[insertable_into(users)]
#[allow(dead_code)]
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

