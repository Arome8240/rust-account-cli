mod cli;
mod models;
mod wallet_manager;

use clap::Parser;
use cli::{Cli, Commands};
use wallet_manager::WalletManager;

fn main() {
    let cli = Cli::parse();
    let mut wallet_manager = WalletManager::new();

    let result = match cli.command {
        Commands::Create { name } => wallet_manager.create_wallet(name),
        Commands::Credit { wallet, amount, description } => {
            wallet_manager.credit_wallet(&wallet, amount, description)
        }
        Commands::Debit { wallet, amount, description } => {
            wallet_manager.debit_wallet(&wallet, amount, description)
        }
        Commands::Balance { wallet } => wallet_manager.show_balance(&wallet),
        Commands::History { wallet } => wallet_manager.show_history(&wallet),
        Commands::List => {
            wallet_manager.list_wallets();
            Ok(())
        }
    };

    if let Err(error) = result {
        eprintln!("❌ Error: {}", error);
        std::process::exit(1);
    }
}
