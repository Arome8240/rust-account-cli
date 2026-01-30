# Rust Wallet Ledger CLI

A simple command-line wallet ledger application built in Rust that allows you to manage multiple wallets with transaction tracking.

## Features

- ✅ Create multiple wallets
- ✅ Credit funds to wallets
- ✅ Debit funds from wallets
- ✅ View wallet balances
- ✅ View transaction history
- ✅ Persistent data storage (JSON)
- ✅ Input validation and error handling

## Installation

```bash
cargo build --release
```

## Usage

### Create a wallet

```bash
cargo run -- create "my-wallet"
```

### Credit funds

```bash
cargo run -- credit my-wallet 100.50 --description "Initial deposit"
```

### Debit funds

```bash
cargo run -- debit my-wallet 25.75 --description "Coffee purchase"
```

### View balance

```bash
cargo run -- balance my-wallet
```

### View transaction history

```bash
cargo run -- history my-wallet
```

### List all wallets

```bash
cargo run -- list
```

### Get help

```bash
cargo run -- --help
```

## Data Storage

Wallet data is automatically saved to `wallets.json` in the current directory. The file includes:

- Wallet names and balances
- Complete transaction history with timestamps
- Transaction descriptions and unique IDs

## Error Handling

The application handles various error cases:

- Insufficient funds for debit operations
- Negative amounts
- Non-existent wallets
- File I/O errors

## Example Session

```bash
cargo run -- create "savings"
```

```
✅ Wallet 'savings' created successfully
```

```bash
cargo run -- credit savings 1000.00 --description "Salary"
```

```
✅ Credited $1000.00 to wallet 'savings'. New balance: $1000.00
```

```bash
cargo run -- debit savings 50.00 --description "Groceries"
```

```
✅ Debited $50.00 from wallet 'savings'. New balance: $950.00
```

```bash
cargo run -- balance savings
```

```
💰 Wallet 'savings' balance: $950.00
```

```bash
cargo run -- history savings
```

```
📝 Transaction history for wallet 'savings':
Date                 Type     Amount       Description          Balance
--------------------------------------------------------------------------------
2026-01-30 08:04:20  CREDIT   $1000.00     Salary               $1000.00
2026-01-30 08:04:31  DEBIT    $50.00       Groceries            $950.00
```
