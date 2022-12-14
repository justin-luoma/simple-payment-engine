use crate::{Account, TransactionData, Type};
use core::option::Option;
use core::option::Option::None;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Disputed,
    Resolved,
    Chargeback,
}

/// Represents a single transaction log entry
#[derive(Debug)]
pub struct Transaction {
    pub t_type: Type,
    pub client: u16,
    pub amount: f32,
    pub state: Option<State>,
}

impl Transaction {
    pub fn new(t_type: Type, client: u16, amount: f32) -> Self {
        Self {
            t_type,
            client,
            amount,
            state: None,
        }
    }
}

/// Process a single transaction record, updating the passed in transaction log and account information
pub fn process_transaction(
    transaction_data: &TransactionData,
    transactions: &mut HashMap<u32, Transaction>,
    accounts: &mut HashMap<u16, Account>,
) {
    match transaction_data.t_type {
        Type::Deposit => {
            if let Some(amount) = transaction_data.amount {
                if let Some(account) = accounts.get_mut(&transaction_data.client) {
                    account.available += amount;
                } else {
                    accounts.insert(transaction_data.client, Account::new(amount, 0.0));
                }
                transactions.insert(
                    transaction_data.id,
                    Transaction::new(transaction_data.t_type, transaction_data.client, amount),
                );
            }
        }
        Type::Withdrawal => {
            if let Some(amount) = transaction_data.amount {
                if let Some(account) = accounts.get_mut(&transaction_data.client) {
                    if account.available >= amount {
                        account.available -= amount;
                    }
                }
                transactions.insert(
                    transaction_data.id,
                    Transaction::new(transaction_data.t_type, transaction_data.client, amount),
                );
            }
        }
        Type::Dispute => {
            if let Some(transaction) = transactions.get_mut(&transaction_data.id) {
                if transaction.state.is_none() || transaction.state == Some(State::Resolved) {
                    transaction.state = Some(State::Disputed);
                    if let Some(account) = accounts.get_mut(&transaction.client) {
                        account.available -= transaction.amount;
                        account.held += transaction.amount;
                    }
                }
            }
        }
        Type::Resolve => {
            if let Some(transaction) = transactions.get_mut(&transaction_data.id) {
                if transaction.state == Some(State::Disputed) {
                    transaction.state = Some(State::Resolved);
                    if let Some(account) = accounts.get_mut(&transaction.client) {
                        account.held -= transaction.amount;
                        account.available += transaction.amount;
                    }
                }
            }
        }
        Type::Chargeback => {
            if let Some(transaction) = transactions.get_mut(&transaction_data.id) {
                if transaction.state == Some(State::Disputed) {
                    transaction.state = Some(State::Chargeback);
                    if let Some(account) = accounts.get_mut(&transaction.client) {
                        account.held -= transaction.amount;
                        account.locked = true;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_transaction_deposit_creates_account_and_updates_available() {
        let expected = HashMap::from([(1, Account::new(1.1234, 0.0))]);

        let mut transactions: HashMap<u32, Transaction> = HashMap::new();
        let mut actual: HashMap<u16, Account> = HashMap::new();

        process_transaction(
            &TransactionData {
                t_type: Type::Deposit,
                id: 1,
                client: 1,
                amount: Some(1.1234),
            },
            &mut transactions,
            &mut actual,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_process_transaction_withdrawal_decreases_available_when_available_greater_or_equal_amount(
    ) {
        let expected = HashMap::from([(1, Account::new(0.0, 0.0))]);

        let mut transactions: HashMap<u32, Transaction> = HashMap::new();
        let mut actual: HashMap<u16, Account> = HashMap::new();

        actual.insert(1, Account::new(1.1234, 0.0));

        process_transaction(
            &TransactionData {
                t_type: Type::Withdrawal,
                id: 1,
                client: 1,
                amount: Some(1.1234),
            },
            &mut transactions,
            &mut actual,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_process_transaction_dispute_existing_transaction_decreases_available_increases_held() {
        let expected = HashMap::from([(1, Account::new(0.0, 1.1234))]);

        let mut transactions: HashMap<u32, Transaction> = HashMap::new();
        transactions.insert(1, Transaction::new(Type::Deposit, 1, 1.1234));

        let mut actual: HashMap<u16, Account> = HashMap::new();
        actual.insert(1, Account::new(1.1234, 0.0));

        process_transaction(
            &TransactionData {
                t_type: Type::Dispute,
                id: 1,
                client: 1,
                amount: None,
            },
            &mut transactions,
            &mut actual,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_process_transaction_resolve_existing_transaction_increases_available_decreases_held() {
        let expected = HashMap::from([(1, Account::new(1.1234, 0.0))]);

        let mut transactions: HashMap<u32, Transaction> = HashMap::new();
        transactions.insert(1, Transaction::new(Type::Deposit, 1, 1.1234));
        let transaction = transactions.get_mut(&1).unwrap();
        transaction.state = Some(State::Disputed);

        let mut actual: HashMap<u16, Account> = HashMap::new();
        actual.insert(1, Account::new(0.0, 1.1234));

        process_transaction(
            &TransactionData {
                t_type: Type::Resolve,
                id: 1,
                client: 1,
                amount: None,
            },
            &mut transactions,
            &mut actual,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_process_transaction_chargeback_existing_transaction_decreases_available_decreases_held_locks_account(
    ) {
        let mut expected = HashMap::from([(1, Account::new(0.0, 0.0))]);
        let account = expected.get_mut(&1).unwrap();
        account.locked = true;

        let mut transactions: HashMap<u32, Transaction> = HashMap::new();
        transactions.insert(1, Transaction::new(Type::Deposit, 1, 1.1234));
        let transaction = transactions.get_mut(&1).unwrap();
        transaction.state = Some(State::Disputed);

        let mut actual: HashMap<u16, Account> = HashMap::new();
        actual.insert(1, Account::new(0.0, 1.1234));

        process_transaction(
            &TransactionData {
                t_type: Type::Chargeback,
                id: 1,
                client: 1,
                amount: None,
            },
            &mut transactions,
            &mut actual,
        );

        assert_eq!(expected, actual);
    }
}
