#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(diesel_codegen, dotenv_macros, json_macros)]
#![plugin(json_macros)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate nickel;
#[macro_use] extern crate dotenv;
extern crate rustc_serialize;

use nickel::{Nickel,
  HttpRouter,
  Request,
  Response,
  MiddlewareResult,
  Continue,
  JsonBody
};

mod models;
mod jsonable;

use models::user::{users, User, NewUser};
use models::post::{posts, Post, NewPost};
use jsonable::*;
use diesel::*;
pub use diesel::data_types::*;
use std::time::*;

fn main() {
  dotenv::dotenv().ok();
  let mut server = Nickel::new();

  server.utilize(middleware! { |request|
      println!("logging request from middleware! macro: {:?}", request.origin.uri);
  });

  server.utilize(logger);

  let mut router = Nickel::router();

  // ****** User Routes
  router.get("/users/:user_id", middleware! { |request|
    let user_id = get_user_id(request);
    User::find(user_id).to_json()
  });

  router.get("/users/:user_id/posts", middleware! { |request|
    let user_id = get_user_id(&request);
    let user: User = User::find(user_id);
    user.posts_vec().to_json()
  });

  // try it with curl
  // curl --request PATCH 'http://localhost:6767/users/1' -H 'Content-Type: application/json;charset=UTF-8' --data-binary $'{ "name": "Change","email": "new@email.com" }'
  router.patch("/users/:user_id", middleware! { |request, response|
    let user_id = get_user_id(request);
    let changed_user = request.json_as::<User>().unwrap();
    let user: User = User::update(user_id, changed_user);
    user.to_json()
  });

  router.get("/users", middleware!(User::count().to_string()));

  // try it with curl
  // curl --request POST 'http://localhost:6767/users' -H 'Content-Type: application/json;charset=UTF-8'  --data-binary $'{ "name": "Mike", "email": "mike@email.com" }'
  router.post("/users", middleware! { |request, response|
    let new_user = request.json_as::<NewUser>().unwrap();
    let new_users = vec!(new_user);
    let users: Vec<User> = User::insert(new_users);
    users.to_json()
  });

  // ****** Post Routes
  router.get("/posts/:post_id", middleware! { |request|
    let post_id = request.param("post_id").unwrap().parse::<i32>().unwrap();
    Post::find(post_id).to_json()
  });

  router.get("/posts", middleware!(Post::count().to_string()));

  // try it with curl
  // curl --request POST 'http://localhost:6767/posts' -H 'Content-Type: application/json;charset=UTF-8'  --data-binary $'{ "user_id": 1,"title": "diesel FTW", "body": "Rust is cool and other interesting stuff" }'
  router.post("/posts", middleware! { |request, response|
    let new_post = request.json_as::<NewPost>().unwrap();
    let new_posts = vec!(new_post);
    let posts: Vec<Post> = Post::insert(new_posts);
    posts.to_json()
  });


  router.get("/users_and_posts", middleware! { |request|
    let users_and_posts = User::users_and_posts();
    users_and_posts.to_json()
  });

  // ******* End Routes

  server.utilize(router);
  server.listen("127.0.0.1:6767");
}

fn logger<'mw>(req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
  println!("logging request from logger fn: {:?}", req.origin.uri);
  Ok(Continue(res))
}

fn get_user_id(request: &Request) -> i32 {
  request.param("user_id").unwrap().parse::<i32>().unwrap()
}

