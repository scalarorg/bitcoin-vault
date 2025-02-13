use clap::{Parser, Subcommand};
use commands::{ConfigCommand, TvlCommand, TxCommands};
use rusqlite::Connection;
use vault::TestSuite;

mod commands;
mod db;
mod executors;

/// Bitcoin Vault Tools
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// database path
    #[arg(long)]
    db_path: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run tx commands
    Tx(TxCommands),

    /// Run config command
    Config(ConfigCommand),
}

struct TvlMaker<'a> {
    suite: TestSuite,
    db_querier: &'a db::Querier,
}

impl<'a> TvlMaker<'a> {
    fn new(db_querier: &'a db::Querier, service_tag: &str, test_env: &str) -> Self {
        unsafe {
            std::env::set_var("TEST_ENV", test_env);
        }
        let suite = TestSuite::new(service_tag);

        Self { suite, db_querier }
    }
}

impl Commands {
    fn execute(&self, db_querier: &db::Querier) -> anyhow::Result<()> {
        match self {
            Commands::Tx(tx_cmd) => {
                let tvl_maker = TvlMaker::new(db_querier, &tx_cmd.service_tag, &tx_cmd.test_env);
                tx_cmd.execute(&tvl_maker)
            }
            Commands::Config(config_cmd) => config_cmd.execute(db_querier),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let db_conn = Connection::open(cli.db_path).expect("Failed to open database");
    let mut db_querier = db::Querier::new(db_conn);
    db_querier.run_migrations();

    cli.command.execute(&db_querier)
}
