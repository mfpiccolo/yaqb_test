use diesel::*;
use models::post::Post;
use self::users::dsl::*;
use diesel::query_builder::*;

infer_schema!(dotenv!("DATABASE_URL"));

#[derive(PartialEq, Eq, Debug, Clone, Queriable, RustcEncodable, Modelable)]
pub struct User {
  pub id: i32,
  pub name: String,
  pub email: Option<String>,
}

impl User {

  #[inline]
  fn conn() -> Connection {
    let connection_url = ::std::env::var("DATABASE_URL").ok()
      .expect("DATABASE_URL must be set in order to run tests");
    Connection::establish(&connection_url).unwrap()
  }

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
    changed_user.save_changes(User::conn())

    // let command = update(users::table.filter(id.eq(_id))).set(changed_user);
    // User::conn().query_one(command).unwrap()
  }

  pub fn posts_vec(&self) -> Vec<Post> {
    Post::belonging_to(self).load(&User::conn()).unwrap().collect()
  }

}

#[derive(PartialEq, Eq, Debug, Clone, Queriable, RustcDecodable)]
#[insertable_into(users)]
#[changeset_for(users)]
pub struct NewUser {
  name: String,
  email: Option<String>,
}
