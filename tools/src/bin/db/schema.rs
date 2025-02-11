pub const CREATE_COMMAND_HISTORY_TABLE: &str = "
    CREATE TABLE IF NOT EXISTS command_histories (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name VARCHAR NOT NULL,
        time INTEGER NOT NULL,
        suite_env VARCHAR NOT NULL,
        params VARCHAR,
        result VARCHAR
    );

    CREATE UNIQUE INDEX IF NOT EXISTS idx_command_history_name_time ON command_histories (name, time);
";
