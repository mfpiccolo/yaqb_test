#![feature(custom_derive, plugin, custom_attribute)]
#![plugin(yaqb_codegen)]

#[macro_use] extern crate yaqb;
#[macro_use] extern crate nickel;
extern crate dotenv;
extern crate rustc_serialize;
use rustc_serialize::json;

use nickel::{Nickel,
  HttpRouter,
  Request,
  Response,
  MiddlewareResult,
  Continue,
  JsonBody
};

mod models;
use models::user::{User, NewUser, Jsonify};
use models::post::{Post, NewPost};
use yaqb::*;

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
    User::find(user_id).unwrap().to_json()
  });

  router.get("/users/:user_id/posts", middleware! { |request|
    let user_id = get_user_id(&request);
    let user: User = User::find(user_id).unwrap();
    user.posts_vec().to_json()
  });

  router.get("/users", middleware!(User::count().unwrap().to_string()));

  // try it with curl
  // curl 'http://localhost:6767/posts' -H 'Content-Type: application/json;charset=UTF-8'  --data-binary $'{ "name": "John","email": "Connor" }'
  router.post("/users", middleware! { |request, response|
    let new_user = request.json_as::<NewUser>().unwrap();
    let new_users = vec!(new_user);
    let users: Vec<User> = User::insert(new_users);
  });

  // ****** Post Routes
  router.get("/posts/:post_id", middleware! { |request|
    let post_id = request.param("post_id").unwrap().parse::<i32>().unwrap();
    Post::find(post_id).unwrap().to_json()
  });

  router.get("/posts", middleware!(Post::count().unwrap().to_string()));

  // try it with curl
  // curl 'http://localhost:6767/posts' -H 'Content-Type: application/json;charset=UTF-8'  --data-binary $'{ "user_id": 1,"title": "YAQBFTW", "body": "Rust is cool and other interesting stuff" }'
  router.post("/posts", middleware! { |request, response|
    let new_post = request.json_as::<NewPost>().unwrap();
    let new_posts = vec!(new_post);
    let posts: Vec<Post> = Post::insert(new_posts);
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

