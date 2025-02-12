use clap::{Parser, Subcommand};
use commands::{BridgeCommands, RedeemCommands, SendTokenCommand, TvlCommand};
use rusqlite::Connection;
use vault::TestSuite;

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
    /// Service tag
    #[arg(short = 'x', long)]
    service_tag: String,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Staking related commands
    Bridge(BridgeCommands),
    /// Send token related commands
    SendToken(SendTokenCommand),
    /// Redeem related commands
    Redeem(RedeemCommands),
    // /// Monitoring and status commands
    // Monitor(MonitorCommands),
}

struct TvlMaker {
    suite: TestSuite,
    db_querier: db::Querier,
}

impl TvlMaker {
    fn new(test_env: &str, service_tag: &str) -> Self {
        unsafe {
            std::env::set_var("TEST_ENV", test_env);
        }
        let suite = TestSuite::new(service_tag);

        let db_conn = Connection::open("tvl_maker.db").expect("Failed to open database");
        let db_querier = db::Querier::new(db_conn);

        Self { suite, db_querier }
    }
}

impl Commands {
    fn execute(&self, tvl_maker: &TvlMaker) -> anyhow::Result<()> {
        match self {
            Commands::Bridge(stake_cmd) => stake_cmd.execute(tvl_maker),
            Commands::SendToken(send_token_cmd) => send_token_cmd.execute(tvl_maker),
            Commands::Redeem(redeem_cmd) => redeem_cmd.execute(tvl_maker),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut tvl_maker = TvlMaker::new(cli.test_env.as_str(), cli.service_tag.as_str());

    tvl_maker.db_querier.run_migrations();
    cli.command.execute(&tvl_maker)
}
