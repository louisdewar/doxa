#[macro_use]
pub extern crate diesel;

pub mod action;
pub mod model;
pub mod schema;

use diesel::Connection;
pub use diesel::{Insertable, Queryable};

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, Pool};

pub type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub use diesel::result::Error as DieselError;

pub fn was_unique_key_violation(error: &DieselError) -> bool {
    matches!(
        error,
        DieselError::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _),
    )
}

pub fn establish_connection(database_url: &str) -> PgPool {
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create DB pool")
}
