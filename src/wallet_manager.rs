use crate::models::{TransactionType, Wallet};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct WalletManager {
    wallets: HashMap<String, Wallet>,
    data_file: String,
}

impl WalletManager {
    pub fn new() -> Self {
        let data_file = "wallets.json".to_string();
        let mut manager = Self {
            wallets: HashMap::new(),
            data_file,
        };
        manager.load_wallets();
        manager
    }

    fn load_wallets(&mut self) {
        if Path::new(&self.data_file).exists() {
            match fs::read_to_string(&self.data_file) {
                Ok(content) => {
                    if let Ok(wallets) = serde_json::from_str::<HashMap<String, Wallet>>(&content) {
                        self.wallets = wallets;
                    }
                }
                Err(_) => {
                    eprintln!("Warning: Could not read wallets file");
                }
            }
        }
    }

    fn save_wallets(&self) -> Result<(), String> {
        let json = serde_json::to_string_pretty(&self.wallets)
            .map_err(|e| format!("Failed to serialize wallets: {}", e))?;

        fs::write(&self.data_file, json)
            .map_err(|e| format!("Failed to write wallets file: {}", e))?;

        Ok(())
    }

    pub fn create_wallet(&mut self, name: String) -> Result<(), String> {
        if self.wallets.contains_key(&name) {
            return Err(format!("Wallet '{}' already exists", name));
        }

        let wallet = Wallet::new(name.clone());
        self.wallets.insert(name.clone(), wallet);
        self.save_wallets()?;

        println!("Wallet '{}' created successfully", name);
        Ok(())
    }

    pub fn credit_wallet(
        &mut self,
        name: &str,
        amount: f64,
        description: Option<String>,
    ) -> Result<(), String> {
        let wallet = self
            .wallets
            .get_mut(name)
            .ok_or_else(|| format!("Wallet '{}' not found", name))?;

        wallet.credit(amount, description)?;
        let balance = wallet.balance;
        self.save_wallets()?;

        println!(
            "Credited ${:.2} to wallet '{}'. New balance: ${:.2}",
            amount, name, balance
        );
        Ok(())
    }

    pub fn debit_wallet(
        &mut self,
        name: &str,
        amount: f64,
        description: Option<String>,
    ) -> Result<(), String> {
        let wallet = self
            .wallets
            .get_mut(name)
            .ok_or_else(|| format!("Wallet '{}' not found", name))?;

        wallet.debit(amount, description)?;
        let balance = wallet.balance;
        self.save_wallets()?;

        println!(
            "Debited ${:.2} from wallet '{}'. New balance: ${:.2}",
            amount, name, balance
        );
        Ok(())
    }

    pub fn show_balance(&self, name: &str) -> Result<(), String> {
        let wallet = self
            .wallets
            .get(name)
            .ok_or_else(|| format!("Wallet '{}' not found", name))?;

        println!("Wallet '{}' balance: ${:.2}", name, wallet.balance);
        Ok(())
    }

    pub fn show_history(&self, name: &str) -> Result<(), String> {
        let wallet = self
            .wallets
            .get(name)
            .ok_or_else(|| format!("Wallet '{}' not found", name))?;

        if wallet.transactions.is_empty() {
            println!("No transactions found for wallet '{}'", name);
            return Ok(());
        }

        println!("Transaction history for wallet '{}':", name);
        println!(
            "{:<20} {:<8} {:<12} {:<20} {}",
            "Date", "Type", "Amount", "Description", "Balance"
        );
        println!("{}", "-".repeat(80));

        let mut running_balance = 0.0;
        for transaction in &wallet.transactions {
            match transaction.transaction_type {
                TransactionType::Credit => running_balance += transaction.amount,
                TransactionType::Debit => running_balance -= transaction.amount,
            }

            let type_str = match transaction.transaction_type {
                TransactionType::Credit => "CREDIT",
                TransactionType::Debit => "DEBIT",
            };

            println!(
                "{:<20} {:<8} ${:<11.2} {:<20} ${:.2}",
                transaction.timestamp.format("%Y-%m-%d %H:%M:%S"),
                type_str,
                transaction.amount,
                transaction.description,
                running_balance
            );
        }

        Ok(())
    }

    pub fn list_wallets(&self) {
        if self.wallets.is_empty() {
            println!("No wallets found. Create one with 'wallet create <name>'");
            return;
        }

        println!("Available wallets:");
        for (name, wallet) in &self.wallets {
            println!("  • {} (Balance: ${:.2})", name, wallet.balance);
        }
    }
}
