use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "wallet")]
#[command(about = "A simple wallet ledger CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Create {
        name: String,
    },
    Credit {
        wallet: String,
        amount: f64,
        #[arg(short, long)]
        description: Option<String>,
    },
    Debit {
        wallet: String,
        amount: f64,
        #[arg(short, long)]
        description: Option<String>,
    },
    Balance {
        wallet: String,
    },
    History {
        wallet: String,
    },
    List,
}