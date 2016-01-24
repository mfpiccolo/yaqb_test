use diesel::connection::PgConnection;
use diesel::Connection;

pub trait Connectable {
  #[inline]
  fn conn() -> PgConnection {
    let connection_url = ::std::env::var("DATABASE_URL").ok()
      .expect("DATABASE_URL must be set in order to run tests");
    let result = PgConnection::establish(&connection_url).unwrap();
    result.begin_test_transaction().unwrap();
    result
  }
}
