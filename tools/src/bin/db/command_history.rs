use super::QueryBuilder;
use chrono::Utc;
use serde::{Deserialize, Serialize};

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

pub struct CommandHistoryQuery {
    pub name: Option<String>,
    pub suite_env: Option<String>,
    pub from_date: Option<i64>,
    pub to_date: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl QueryBuilder for CommandHistoryQuery {
    fn build(&self) -> String {
        let mut conditions = Vec::new();
        let mut query = String::from("SELECT * FROM command_histories WHERE 1=1");

        if let Some(name) = &self.name {
            conditions.push(format!(" AND name = '{}'", name));
        }
        if let Some(suite_env) = &self.suite_env {
            conditions.push(format!(" AND suite_env = '{}'", suite_env));
        }
        if let Some(from_date) = self.from_date {
            conditions.push(format!(" AND time >= {}", from_date));
        }
        if let Some(to_date) = self.to_date {
            conditions.push(format!(" AND time <= {}", to_date));
        }

        query.push_str(&conditions.join(""));
        query.push_str(" ORDER BY time DESC");

        if let Some(limit) = self.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }
        if let Some(offset) = self.offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }

        query
    }
}
