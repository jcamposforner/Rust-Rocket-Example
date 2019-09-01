use r2d2;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::ops::Deref;

pub type Pool = r2d2::Pool<r2d2_postgres::PostgresConnectionManager>;

pub fn init_pool(db_url: String) -> Pool {
    let manager = PostgresConnectionManager::new(db_url, TlsMode::None).unwrap();
    let pool = r2d2::Pool::new(manager).unwrap();

    pool
}

pub struct Conn(pub r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>);

impl<'a, 'r> FromRequest<'a, 'r> for Conn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Conn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Conn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for Conn {
    type Target = r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
