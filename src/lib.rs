use std::collections::HashMap;
use std::error::Error;

use account::Account;
use csv::Trim;
use serde::Deserialize;

use crate::transaction::{process_transaction, Transaction};
use crate::transaction_type::Type;

mod account;
mod transaction;
mod transaction_type;

/// Represents a single transaction from a csv record
#[derive(Debug, Deserialize)]
pub struct TransactionData {
    #[serde(rename = "type")]
    t_type: Type,
    client: u16,
    #[serde(rename = "tx")]
    id: u32,
    amount: Option<f32>,
}

/// Process csv file one record at a time, updating the transaction log while keeping track of
/// account state, returns and error if the csv reader fails to reader the provided path
pub fn process_csv_file(path: &str) -> Result<HashMap<u16, Account>, Box<dyn Error>> {
    // TODO: replace with database
    let mut transactions: HashMap<u32, Transaction> = HashMap::new();
    let mut accounts: HashMap<u16, Account> = HashMap::new();

    let mut reader = csv::ReaderBuilder::new().trim(Trim::All).from_path(path)?;
    for transaction in reader.deserialize().flatten() {
        process_transaction(&transaction, &mut transactions, &mut accounts);
    }

    Ok(accounts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_csv_file_example_data() {
        let mut expected: HashMap<u16, Account> = HashMap::new();
        let account_1 = Account::new(1.5, 0.0);
        let account_2 = Account::new(2.0, 0.0);
        expected.insert(1, account_1);
        expected.insert(2, account_2);

        assert_eq!(
            expected,
            process_csv_file("./data/inputs/example_no_spacing.csv").unwrap()
        );
    }

    #[test]
    fn test_process_csv_file_all_transactions() {
        let mut expected: HashMap<u16, Account> = HashMap::new();
        let mut account_1 = Account::new(13.1415, 0.0);
        account_1.locked = true;
        expected.insert(1, account_1);

        assert_eq!(
            expected,
            process_csv_file("./data/inputs/test_all_transactions.csv").unwrap()
        );
    }
}
