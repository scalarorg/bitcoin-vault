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

pub const CREATE_CONFIG_TABLE: &str = "
    CREATE TABLE IF NOT EXISTS configs (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name VARCHAR NOT NULL,
        btc_node_address VARCHAR NOT NULL,
        btc_node_user VARCHAR NOT NULL,
        btc_node_password VARCHAR NOT NULL,
        btc_node_wallet VARCHAR NOT NULL,
        protocol_private_key VARCHAR NOT NULL,
        custodian_private_keys VARCHAR NOT NULL,
        custodian_quorum INTEGER NOT NULL,
        network VARCHAR NOT NULL,
        tag VARCHAR NOT NULL,
        version INTEGER NOT NULL
    );

    CREATE UNIQUE INDEX IF NOT EXISTS idx_config_name ON configs (name);
";
