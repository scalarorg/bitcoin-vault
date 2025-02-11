use bitcoin_vault::TestSuite;
use clap::{Parser, Subcommand};
use commands::{StakeCommands, TvlCommand};
use rusqlite::Connection;

mod commands;
mod db;
mod executors;

/// Bitcoin Vault Tools
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Test environment to use (regtest, testnet4 or custom to the env file)
    #[arg(short, long, default_value = ".env")]
    test_env: String,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Staking related commands
    Stake(StakeCommands),
    // /// Unstaking related commands
    // Unstake(UnstakeCommands),

    // /// Monitoring and status commands
    // Monitor(MonitorCommands),
}

struct TvlMaker {
    suite: TestSuite,
    db_querier: db::Querier,
}

impl TvlMaker {
    fn new(test_env: &str) -> Self {
        unsafe {
            std::env::set_var("TEST_ENV", test_env);
        }
        let suite = TestSuite::new();

        let db_conn = Connection::open("tvl_maker.db").expect("Failed to open database");
        let db_querier = db::Querier::new(db_conn);

        Self { suite, db_querier }
    }
}

impl Commands {
    fn execute(&self, tvl_maker: &TvlMaker) -> anyhow::Result<()> {
        match self {
            Commands::Stake(stake_cmd) => stake_cmd.execute(tvl_maker),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut tvl_maker = TvlMaker::new(cli.test_env.as_str());

    tvl_maker.db_querier.run_migrations();
    cli.command.execute(&tvl_maker)
}
