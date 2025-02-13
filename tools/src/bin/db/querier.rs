use rusqlite::{params, Connection};
use rusqlite_migration::{Migrations, M};

use super::{
    CommandHistory, CommandHistoryQuery, DbError, DbOperations, DbResult, QueryBuilder,
    CREATE_COMMAND_HISTORY_TABLE,
};

pub struct Querier {
    db_conn: Connection,
}

impl Querier {
    pub fn new(db_conn: Connection) -> Self {
        Self { db_conn }
    }

    pub fn run_migrations(&mut self) {
        let migrations = Migrations::new(vec![M::up(CREATE_COMMAND_HISTORY_TABLE)]);

        migrations
            .to_latest(&mut self.db_conn)
            .expect("Failed to run migrations");
    }

    pub fn query_command_history(
        &self,
        query: CommandHistoryQuery,
    ) -> DbResult<Vec<CommandHistory>> {
        let sql = <CommandHistoryQuery as QueryBuilder>::build(&query);
        let mut stmt = self.db_conn.prepare(&sql)?;

        let histories = stmt.query_map([], |row| {
            Ok(CommandHistory {
                id: row.get(0)?,
                name: row.get(1)?,
                time: row.get(2)?,
                suite_env: row.get(3)?,
                params: row.get(4)?,
                result: row.get(5)?,
            })
        })?;

        histories
            .collect::<Result<Vec<_>, _>>()
            .map_err(DbError::from)
    }
}

impl DbOperations<CommandHistory> for Querier {
    fn create(&self, item: &CommandHistory) -> DbResult<i64> {
        let mut stmt = self.db_conn.prepare(
            "INSERT INTO command_histories (name, time, suite_env, params, result) 
             VALUES (?1, ?2, ?3, ?4, ?5) RETURNING id",
        )?;

        let id = stmt.query_row(
            params![
                item.name,
                item.time,
                item.suite_env,
                item.params,
                item.result,
            ],
            |row| row.get(0),
        )?;
        Ok(id)
    }

    fn read(&self, id: i64) -> DbResult<CommandHistory> {
        let mut stmt = self.db_conn.prepare(
            "SELECT id, name, time, suite_env, params, result 
             FROM command_histories 
             WHERE id = ?1",
        )?;

        stmt.query_row([id], |row| {
            Ok(CommandHistory {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                time: row.get(2)?,
                suite_env: row.get(3)?,
                params: row.get(4)?,
                result: row.get(5)?,
            })
        })
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => DbError::NotFound,
            e => DbError::from(e),
        })
    }

    fn update(&self, id: i64, item: &CommandHistory) -> DbResult<()> {
        let mut stmt = self.db_conn.prepare(
            "UPDATE command_histories 
             SET name = ?1, time = ?2, suite_env = ?3, params = ?4, result = ?5
             WHERE id = ?6",
        )?;

        stmt.execute(params![
            item.name,
            item.time,
            item.suite_env,
            item.params,
            item.result,
            id
        ])?;

        Ok(())
    }

    fn delete(&self, id: i64) -> DbResult<()> {
        let mut stmt = self
            .db_conn
            .prepare("DELETE FROM command_histories WHERE id = ?1")?;
        stmt.execute([id])?;
        Ok(())
    }

    fn list(&self, limit: Option<i64>, offset: Option<i64>) -> DbResult<Vec<CommandHistory>> {
        let query = CommandHistoryQuery {
            name: None,
            suite_env: None,
            from_date: None,
            to_date: None,
            limit,
            offset,
        };
        self.query_command_history(query)
    }
}
