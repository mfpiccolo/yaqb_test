extern crate serde;
extern crate serde_json;

use yaqb::*;

table! {
  posts {
    id -> Serial,
    user_id -> Integer,
    title -> VarChar,
    body -> Nullable<Text>,
  }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Queriable)]
pub struct Post {
  pub id: i32,
  pub user_id: i32,
  pub title: String,
  pub body: Option<String>,
}

impl Post {

  #[inline]
  pub fn conn() -> Connection {
    let connection_url = ::std::env::var("DATABASE_URL").ok()
      .expect("DATABASE_URL must be set in order to run tests");
    Connection::establish(&connection_url).unwrap()
  }

  pub fn find(id: i32) -> Option<Post> {
    Post::conn().find(self::posts::table, id).unwrap()
  }

  pub fn count() -> Option<i64> {
    let select_count = posts::table.select_sql::<types::BigInt>("COUNT(*)");
    Post::conn().query_one::<_, i64>(select_count.clone()).unwrap()
  }

  pub fn insert(new_posts: Vec<NewPost>) -> Vec<Post> {
    Post::conn().insert(&self::posts::table, &new_posts).unwrap().collect()
  }

  pub fn to_json(&self) -> String {
    serde_json::to_string(&self).unwrap()
  }

  pub fn new_post(&self, title: &str, body: Option<&str>) -> NewPost {
    NewPost::new(self.id, title, body)
  }

}

#[derive(RustcDecodable, Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Queriable)]
#[insertable_into(posts)]
pub struct NewPost {
  pub user_id: i32,
  pub title: String,
  pub body: Option<String>,
}

impl NewPost {
  pub fn new(user_id: i32, title: &str, body: Option<&str>) -> Self {
    NewPost {
      user_id: user_id,
      title: title.into(),
      body: body.map(|b| b.into()),
    }
  }
}

