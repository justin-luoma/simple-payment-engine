use std::collections::HashMap;
use std::error::Error;

use account::Account;
use csv::Trim;
use serde::Deserialize;

use crate::transaction::{Transaction, TransactionState};
use crate::transaction_type::TransactionType;

mod account;
mod transaction;
mod transaction_type;

/// Represents a single transaction from a csv record
#[derive(Debug, Deserialize)]
pub struct TransactionData {
    #[serde(rename = "type")]
    transaction_type: TransactionType,
    client: u16,
    #[serde(rename = "tx")]
    id: u32,
    amount: Option<f32>,
}

impl TransactionData {
    /// Process a single transaction record, updating the passed in account
    pub fn process(&self, account: &mut Account) -> Result<(), Box<dyn Error>> {
        match self.transaction_type {
            TransactionType::Deposit => {
                account.available += self.amount.unwrap();

                account.transactions.insert(
                    self.id,
                    Transaction::new(
                        self.transaction_type,
                        self.client,
                        self.amount.ok_or("Deposits should have an amount")?,
                    ),
                );
            }

            TransactionType::Withdrawal => {
                if account.available >= self.amount.unwrap() {
                    account.available -= self.amount.unwrap();
                    account.transactions.insert(
                        self.id,
                        Transaction::new(
                            self.transaction_type,
                            self.client,
                            self.amount.ok_or("Withdrawals should have an amount")?,
                        ),
                    );
                }
            }

            TransactionType::Dispute => {
                if let Some(transaction) = account.transactions.get_mut(&self.id) {
                    if transaction.state.is_none()
                        || transaction.state == Some(TransactionState::Resolved)
                    {
                        transaction.state = Some(TransactionState::Disputed);
                        account.available -= transaction.amount;
                        account.held += transaction.amount;
                    }
                }
            }

            TransactionType::Resolve => {
                if let Some(transaction) = account.transactions.get_mut(&self.id) {
                    if transaction.state == Some(TransactionState::Disputed) {
                        transaction.state = Some(TransactionState::Resolved);
                        account.held -= transaction.amount;
                        account.available += transaction.amount;
                    }
                }
            }

            TransactionType::Chargeback => {
                if let Some(transaction) = account.transactions.get_mut(&self.id) {
                    if transaction.state == Some(TransactionState::Disputed) {
                        transaction.state = Some(TransactionState::Chargeback);
                        account.held -= transaction.amount;
                        account.locked = true;
                    }
                }
            }
        }
        Ok(())
    }
}

/// Process csv file one record at a time, updating the transaction log while keeping track of
/// account state, returns and error if the csv reader fails to reader the provided path
pub fn process_csv_file(path: &str) -> Result<HashMap<u16, Account>, Box<dyn Error>> {
    // TODO: replace with database
    let mut accounts: HashMap<u16, Account> = HashMap::new();

    let mut reader = csv::ReaderBuilder::new().trim(Trim::All).from_path(path)?;

    for transaction_data in reader.deserialize::<TransactionData>().flatten() {
        if let Some(account) = accounts.get_mut(&transaction_data.client) {
            account.update(transaction_data)?;
        } else if transaction_data.transaction_type == TransactionType::Deposit {
            let mut account = Account::new(transaction_data.client);
            let client = transaction_data.client;
            account.update(transaction_data)?;
            accounts.insert(client, account);
        }
    }

    Ok(accounts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_csv_file_example_data() {
        let mut expected_account_1 = Account::new(1);
        expected_account_1.available = 1.5;
        let transaction_1 = Transaction::new(TransactionType::Deposit, 1, 1.0);
        let transaction_2 = Transaction::new(TransactionType::Deposit, 1, 2.0);
        let transaction_3 = Transaction::new(TransactionType::Withdrawal, 1, 1.5);
        expected_account_1.transactions.insert(1, transaction_1);
        expected_account_1.transactions.insert(3, transaction_2);
        expected_account_1.transactions.insert(4, transaction_3);

        let mut expected_account_2 = Account::new(2);
        expected_account_2.available = 2.0;
        let transaction_1 = Transaction::new(TransactionType::Deposit, 2, 2.0);
        expected_account_2.transactions.insert(2, transaction_1);

        let actual = process_csv_file("./data/inputs/example_no_spacing.csv").unwrap();

        assert_eq!(&expected_account_1, actual.get(&1).unwrap());
        assert_eq!(&expected_account_2, actual.get(&2).unwrap());
    }

    #[test]
    fn test_process_csv_file_all_transactions() {
        let mut expected_account_1 = Account::new(1);
        expected_account_1.available = 16.66;
        expected_account_1.locked = true;
        let transaction_1 = Transaction::new(TransactionType::Deposit, 1, 6.66);
        let transaction_2 = Transaction::new(TransactionType::Withdrawal, 1, 1.0);
        let transaction_3 = Transaction::new(TransactionType::Deposit, 1, 1.0);
        let mut transaction_4 = Transaction::new(TransactionType::Deposit, 1, 10.0);
        transaction_4.state = Some(TransactionState::Resolved);
        let mut transaction_5 = Transaction::new(TransactionType::Deposit, 1, 13.0);
        transaction_5.state = Some(TransactionState::Chargeback);
        expected_account_1.transactions.insert(1, transaction_1);
        expected_account_1.transactions.insert(2, transaction_2);
        expected_account_1.transactions.insert(3, transaction_3);
        expected_account_1.transactions.insert(4, transaction_4);
        expected_account_1.transactions.insert(5, transaction_5);

        let actual = process_csv_file("./data/inputs/test_all_transactions.csv").unwrap();
        assert_eq!(expected_account_1.id, actual.get(&1).unwrap().id);
        assert_eq!(
            expected_account_1.available,
            actual.get(&1).unwrap().available
        );
        assert_eq!(expected_account_1.locked, actual.get(&1).unwrap().locked);
        assert_eq!(expected_account_1.held, actual.get(&1).unwrap().held);
        assert_eq!(
            expected_account_1.transactions.get(&1),
            actual.get(&1).unwrap().transactions.get(&1)
        );
        assert_eq!(
            expected_account_1.transactions.get(&2),
            actual.get(&1).unwrap().transactions.get(&2)
        );
        assert_eq!(
            expected_account_1.transactions.get(&3),
            actual.get(&1).unwrap().transactions.get(&3)
        );
        assert_eq!(
            expected_account_1.transactions.get(&4),
            actual.get(&1).unwrap().transactions.get(&4)
        );
        assert_eq!(
            expected_account_1.transactions.get(&5),
            actual.get(&1).unwrap().transactions.get(&5)
        );
    }
}
