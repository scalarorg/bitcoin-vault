pub mod command_history;
pub mod config;
pub mod querier;
pub mod schema;

pub use command_history::*;
pub use config::*;
pub use querier::*;
pub use schema::*;

#[derive(Debug)]
pub enum DbError {
    SqliteError(rusqlite::Error),
    NotFound,
    InvalidData(String),
}

impl From<rusqlite::Error> for DbError {
    fn from(err: rusqlite::Error) -> Self {
        DbError::SqliteError(err)
    }
}

pub type DbResult<T> = Result<T, DbError>;

// First, create a trait for database entities
pub trait DbEntity {
    fn table_name() -> &'static str;
    fn columns() -> Vec<&'static str>;
    fn values(&self) -> Vec<Box<dyn rusqlite::ToSql>>;
    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self>
    where
        Self: Sized;
}
