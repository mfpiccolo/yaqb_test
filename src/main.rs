#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
#![plugin(yaqb_codegen)]

#[macro_use] extern crate yaqb;
#[macro_use] extern crate nickel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
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
use models::user::{User, NewUser};
use yaqb::*;

fn main() {
  dotenv::dotenv().ok();
  let mut server = Nickel::new();

  server.utilize(middleware! { |request|
      println!("logging request from middleware! macro: {:?}", request.origin.uri);
  });

  server.utilize(logger);

  let mut router = Nickel::router();

  router.get("/users/:userid", middleware! { |request|
    let user_id = request.param("userid").unwrap().parse::<i32>().unwrap();
    User::find(user_id).unwrap().to_json()
  });
  router.get("/users", middleware!(User::count().unwrap().to_string()));

  // try it with curl
  // curl 'http://localhost:6767/users' -H 'Content-Type: application/json;charset=UTF-8'  --data-binary $'{ "name": "John","email": "Connor" }'
  router.post("/users", middleware! { |request, response|
    let new_user = request.json_as::<NewUser>().unwrap();
    let new_users = vec!(new_user);
    User::insert(new_users);
  });

  server.utilize(router);
  server.listen("127.0.0.1:6767");
}

fn logger<'mw>(req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
  println!("logging request from logger fn: {:?}", req.origin.uri);
  Ok(Continue(res))
}

#[test]
fn find_user_test() {
  let conn = before_each();
  let mike: User = User::find(1).unwrap();
  assert_eq!("Mike", mike.name);
  after_each(&conn);
}

#[test]
fn count_users_test() {
  let conn = before_each();
  assert_eq!(Some(2), User::count());
  after_each(&conn);
}

fn setup_users_table(conn: &Connection) {
  conn.execute("CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    email VARCHAR
  )").unwrap();
}

fn tear_down_users_table(conn: &Connection) {
  conn.execute("DROP TABLE IF EXISTS users").unwrap();
}

fn before_each() -> Connection {
  dotenv::dotenv().ok();
  let conn = User::conn();
  setup_users_table(&conn);
  conn.execute("INSERT INTO users (name) VALUES ('Mike'), ('Joe')").unwrap();
  User::conn()
}

fn after_each(conn: &Connection) {
  // tear_down_users_table(&conn);
}


