use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: String,
    pub transaction_type: TransactionType,
    pub amount: f64,
    pub description: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum TransactionType {
    Credit,
    Debit,
}

#[derive(Serialize, Deserialize)]
pub struct Wallet {
    pub name: String,
    pub balance: f64,
    pub transactions: Vec<Transaction>,
}

impl Wallet {
    pub fn new(name: String) -> Self {
        Self {
            name,
            balance: 0.0,
            transactions: Vec::new(),
        }
    }

    pub fn credit(&mut self, amount: f64, description: Option<String>) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Amount must be positive".to_string());
        }

        let transaction = Transaction {
            id: format!("{}", Utc::now().timestamp_nanos_opt().unwrap_or(0)),
            transaction_type: TransactionType::Credit,
            amount,
            description: description.unwrap_or_else(|| "Credit transaction".to_string()),
            timestamp: Utc::now(),
        };

        self.balance += amount;
        self.transactions.push(transaction);
        Ok(())
    }

    pub fn debit(&mut self, amount: f64, description: Option<String>) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Amount must be positive".to_string());
        }

        if self.balance < amount {
            return Err("Insufficient funds".to_string());
        }

        let transaction = Transaction {
            id: format!("{}", Utc::now().timestamp_nanos_opt().unwrap_or(0)),
            transaction_type: TransactionType::Debit,
            amount,
            description: description.unwrap_or_else(|| "Debit transaction".to_string()),
            timestamp: Utc::now(),
        };

        self.balance -= amount;
        self.transactions.push(transaction);
        Ok(())
    }
}