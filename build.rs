extern crate diesel;
extern crate dotenv;
use diesel::*;
use dotenv::dotenv;

fn main() {
  dotenv().ok();
  let database_url = ::std::env::var("DATABASE_URL")
      .expect("DATABASE_URL must be set to run tests");
  let connection = Connection::establish(&database_url).unwrap();
  migrations::run_pending_migrations(&connection).unwrap();
}
