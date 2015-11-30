use diesel::*;
use rustc_serialize::json;
use models::user::{ users, User };
use self::posts::dsl::*;
use yaqb_model::modelable::Modelable;

table! {
  posts {
    id -> Serial,
    user_id -> Integer,
    title -> VarChar,
    body -> Nullable<Text>,
  }
}

#[derive(PartialEq, Eq, Debug, Clone, Queriable, RustcEncodable, Modelable)]
#[belongs_to(user)]
pub struct Post {
  pub id: i32,
  pub user_id: i32,
  pub title: String,
  pub body: Option<String>,
}

impl Post {

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

