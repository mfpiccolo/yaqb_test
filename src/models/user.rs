use diesel::*;
use models::post::{posts, Post};
use models::connectable::Connectable;
use self::users::dsl::*;
pub use diesel::connection::PgConnection;

infer_table_from_schema!(dotenv!("DATABASE_URL"), "users");

#[derive(PartialEq, Eq, Debug, Clone, Queryable, Serialize, Deserialize, RustcDecodable, Default)]
#[changeset_for(users)]
#[has_many(posts)]
pub struct User {
  pub id: i32,
  pub name: String,
  pub email: Option<String>,
}

impl User {

  pub fn find(_id: i32) -> User {
    users.filter(id.eq(_id)).get_result(&User::conn()).unwrap()
  }

  pub fn count() -> i64 {
    users.count().get_result(&User::conn()).ok().unwrap()
  }

  pub fn insert(new_users: Vec<NewUser>) -> Vec<User> {
    insert(&new_users).into(users).get_results(&User::conn()).unwrap().collect::<Vec<User>>()
  }

  pub fn update(_id: i32, changed_user: User) -> User {
    changed_user.save_changes(&User::conn()).ok().unwrap()
  }

  pub fn posts_vec(&self) -> Vec<Post> {
    Post::belonging_to(self).load(&User::conn()).unwrap().collect()
  }

  pub fn users_and_posts() -> Vec<(User, Option<Post>)> {
    users.left_outer_join(posts::table).load(&User::conn()).unwrap().collect()
  }
}

impl Connectable for User {}

#[derive(PartialEq, Eq, Debug, Clone, Queryable, RustcDecodable)]
#[insertable_into(users)]
pub struct NewUser {
  name: String,
  email: Option<String>,
}
