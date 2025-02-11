pub mod command_history;
pub mod querier;
pub mod schema;

pub use command_history::*;
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

pub trait QueryBuilder {
    fn build(&self) -> String;
}

pub trait DbOperations<T> {
    fn create(&self, item: &T) -> DbResult<i64>;
    fn read(&self, id: i64) -> DbResult<T>;
    fn update(&self, id: i64, item: &T) -> DbResult<()>;
    fn delete(&self, id: i64) -> DbResult<()>;
    fn list(&self, limit: Option<i64>, offset: Option<i64>) -> DbResult<Vec<T>>;
}
