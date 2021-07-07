#[macro_use]
pub extern crate diesel;

pub mod schema;

pub use diesel::{Insertable, Queryable};

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

pub type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn was_unique_key_violation(error: &diesel::result::Error) -> bool {
    matches!(
        error,
        diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _),
    )
}
