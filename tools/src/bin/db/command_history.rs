use chrono::Utc;
use serde::{Deserialize, Serialize};

use super::DbEntity;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandHistory {
    pub id: Option<u64>,
    pub name: String,
    /// unix timestamp, seconds since epoch
    pub time: i64,
    pub suite_env: Option<String>,
    pub params: Option<String>,
    pub result: Option<String>,
}

impl CommandHistory {
    pub fn new(
        name: String,
        suite_env: Option<String>,
        params: Option<String>,
        result: Option<String>,
    ) -> Self {
        Self {
            id: None,
            name,
            time: Utc::now().timestamp_nanos_opt().unwrap(),
            suite_env,
            params,
            result,
        }
    }
}

impl Default for CommandHistory {
    fn default() -> Self {
        Self {
            id: None,
            name: "".to_string(),
            time: Utc::now().timestamp_nanos_opt().unwrap(),
            suite_env: None,
            params: None,
            result: None,
        }
    }
}

impl DbEntity for CommandHistory {
    fn table_name() -> &'static str {
        "command_histories"
    }

    fn columns() -> Vec<&'static str> {
        vec!["name", "time", "suite_env", "params", "result"]
    }

    fn values(&self) -> Vec<Box<dyn rusqlite::ToSql>> {
        vec![
            Box::new(self.name.clone()),
            Box::new(self.time),
            Box::new(self.suite_env.clone()),
            Box::new(self.params.clone()),
            Box::new(self.result.clone()),
        ]
    }

    fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(CommandHistory {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            time: row.get(2)?,
            suite_env: row.get(3)?,
            params: row.get(4)?,
            result: row.get(5)?,
        })
    }
}
