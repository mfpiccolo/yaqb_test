use diesel::*;
use models::post::{posts, Post};
use models::connectable::Connectable;
use self::users::dsl::*;
use diesel::query_builder::*;
use diesel::query_builder::debug::DebugQueryBuilder;

infer_table_from_schema!(dotenv!("DATABASE_URL"), "users");

#[derive(PartialEq, Eq, Debug, Clone, Queriable, Serialize, Deserialize, RustcDecodable)]
#[changeset_for(users)]
#[has_many(posts)]
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
    users.count().get_result(&User::conn()).ok().unwrap()
  }

  pub fn insert(new_users: Vec<NewUser>) -> Vec<User> {
    User::conn().insert(&users, &new_users).unwrap().collect()
  }

  pub fn update(_id: i32, changed_user: User) -> User {
    changed_user.save_changes(&User::conn()).ok().unwrap()
  }

  pub fn posts_vec(&self) -> Vec<Post> {
    Post::belonging_to(self).load(&User::conn()).unwrap().collect()
  }

  pub fn users_and_posts() -> Vec<(User, Option<Post>)> {
    users.left_outer_join(posts::table)
      .load(&User::conn()).unwrap().collect()
  }
}

impl Connectable for User {}

#[derive(PartialEq, Eq, Debug, Clone, Queriable, RustcDecodable)]
#[insertable_into(users)]
pub struct NewUser {
  name: String,
  email: Option<String>,
}
