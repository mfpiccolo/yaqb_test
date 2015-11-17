extern crate dotenv;
#[macro_use] extern crate yaqb;

use yaqb::*;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub hair_color: Option<String>,
}

table! {
    users {
        id -> Serial,
        name -> VarChar,
        hair_color -> Nullable<VarChar>,
    }
}

queriable! {
    User {
        id -> i32,
        name -> String,
        hair_color -> Option<String>,
    }
}

fn main() {
  use self::users::table as users;
  dotenv::dotenv().ok();
  let connection_url = ::std::env::var("DATABASE_URL").ok()
      .expect("DATABASE_URL must be set in order to run tests");
  let conn = Connection::establish(&connection_url).unwrap();

  conn.execute("INSERT INTO users (name) VALUES ('Sean'), ('Tess')")
      .unwrap();

  let result: Option<User> = conn.find(users, 1).unwrap();

  println!("{:?}", result);
}

