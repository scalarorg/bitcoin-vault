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
    #[arg(long)]
    db_path: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Tx(TxCommands),
}

#[derive(Parser, Debug)]
struct TxCommands {
    /// Test environment to use (regtest, testnet4 or custom to the env file)
    #[arg(short, long, default_value = ".env")]
    test_env: String,
    /// Service tag
    #[arg(short = 'x', long)]
    service_tag: String,

    #[command(subcommand)]
    command: SubTxCommands,
}

#[derive(Subcommand, Debug)]
enum SubTxCommands {
    /// Staking related commands
    Bridge(BridgeCommands),
    /// Send token related commands
    SendToken(SendTokenCommand),
    /// Redeem related commands
    Redeem(RedeemCommands),
    // /// Monitoring and status commands
    // Monitor(MonitorCommands),
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
                match &tx_cmd.command {
                    SubTxCommands::Bridge(bridge_cmd) => bridge_cmd.execute(&tvl_maker),
                    SubTxCommands::SendToken(send_token_cmd) => send_token_cmd.execute(&tvl_maker),
                    SubTxCommands::Redeem(redeem_cmd) => redeem_cmd.execute(&tvl_maker),
                }
            }
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
