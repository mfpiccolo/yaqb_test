use diesel::*;
use models::user::{users, User};
use self::posts::dsl::*;

infer_table_from_schema!(dotenv!("DATABASE_URL"), "posts");

#[derive(PartialEq, Eq, Debug, Clone, Queriable, RustcEncodable)]
#[belongs_to(user)]
pub struct Post {
  id: i32,
  user_id: i32,
  title: String,
  body: Option<String>,
}

impl Post {

  #[inline]
  fn conn() -> Connection {
    let connection_url = ::std::env::var("DATABASE_URL").ok()
      .expect("DATABASE_URL must be set in order to run tests");
    Connection::establish(&connection_url).unwrap()
  }

  pub fn find(_id: i32) -> Post {
    Post::conn().find(posts, _id).unwrap()
  }

  pub fn count() -> i64 {
    let select_count = posts.select_sql::<types::BigInt>("COUNT(*)");
    Post::conn().query_one::<_, i64>(select_count.clone()).unwrap()
  }

  pub fn insert(new_posts: Vec<NewPost>) -> Vec<Post> {
    Post::conn().insert(&posts, &new_posts).unwrap().collect()
  }

  pub fn new_post(&self, _title: &str, _body: Option<&str>) -> NewPost {
    NewPost::new(self.id, _title, _body)
  }

}

#[derive(RustcDecodable, PartialEq, Eq, Debug, Clone, Queriable)]
#[insertable_into(posts)]
#[changeset_for(posts)]
#[allow(dead_code)]
pub struct NewPost {
  pub user_id: i32,
  pub title: String,
  pub body: Option<String>,
}

impl NewPost {
  pub fn new(_user_id: i32, _title: &str, _body: Option<&str>) -> Self {
    NewPost {
      user_id: _user_id,
      title: _title.into(),
      body: _body.map(|b| b.into()),
    }
  }
}

