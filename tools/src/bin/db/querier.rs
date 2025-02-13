use rusqlite::Connection;
use rusqlite_migration::{Migrations, M};

use super::{DbEntity, DbError, DbResult, CREATE_COMMAND_HISTORY_TABLE, CREATE_CONFIG_TABLE};

pub struct Querier {
    pub db_conn: Connection,
}

// Generic implementation for database operations
impl Querier {
    pub fn save<T: DbEntity>(&self, item: &T) -> DbResult<i64> {
        let placeholders = (1..=item.values().len())
            .map(|i| format!("?{}", i))
            .collect::<Vec<_>>()
            .join(", ");

        let query = format!(
            "INSERT OR REPLACE INTO {} ({}) VALUES ({}) RETURNING id",
            T::table_name(),
            T::columns().join(", "),
            placeholders
        );

        let mut stmt = self.db_conn.prepare(&query)?;
        stmt.query_row(rusqlite::params_from_iter(item.values()), |row| row.get(0))
            .map_err(DbError::from)
    }

    pub fn read<T: DbEntity>(&self, id: i64) -> DbResult<T> {
        let query = format!("SELECT * FROM {} WHERE id = ?1", T::table_name());

        let mut stmt = self.db_conn.prepare(&query)?;
        stmt.query_row([id], T::from_row).map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => DbError::NotFound,
            e => DbError::from(e),
        })
    }

    pub fn list<T: DbEntity>(&self, limit: Option<i64>, offset: Option<i64>) -> DbResult<Vec<T>> {
        let mut query = format!("SELECT * FROM {}", T::table_name());

        if let Some(limit) = limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }
        if let Some(offset) = offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }

        let mut stmt = self.db_conn.prepare(&query)?;
        let rows = stmt.query_map([], T::from_row)?;

        rows.collect::<Result<Vec<_>, _>>().map_err(DbError::from)
    }
}

impl Querier {
    pub fn new(db_conn: Connection) -> Self {
        Self { db_conn }
    }

    pub fn run_migrations(&mut self) {
        let migrations = Migrations::new(vec![
            M::up(CREATE_COMMAND_HISTORY_TABLE),
            M::up(CREATE_CONFIG_TABLE),
        ]);

        migrations
            .to_latest(&mut self.db_conn)
            .expect("Failed to run migrations");
    }
}
