#[macro_use]
pub extern crate diesel;
#[macro_use]
pub extern crate diesel_migrations;

pub mod action;
pub mod model;
pub mod schema;
pub mod view;

use diesel::Connection;
pub use diesel::{Insertable, Queryable};

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, Pool};

pub type PgPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub use diesel::result::Error as DieselError;

pub use serde_json;

embed_migrations!();

pub fn run_migrations(connection: &PgConnection) {
    embedded_migrations::run_with_output(connection, &mut std::io::stdout())
        .expect("Failed to run migrations");
}

pub fn was_unique_key_violation(error: &DieselError) -> bool {
    matches!(
        error,
        DieselError::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _),
    )
}

pub fn establish_connection(database_url: &str) -> PgConnection {
    PgConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn establish_pool(database_url: &str) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create DB pool")
}
