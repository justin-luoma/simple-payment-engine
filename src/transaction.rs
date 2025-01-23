use crate::TransactionType;
use core::option::Option;
use core::option::Option::None;

#[derive(Debug, PartialEq)]
pub enum TransactionState {
    Disputed,
    Resolved,
    Chargeback,
}

/// Represents a single transaction log entry
#[derive(Debug, PartialEq)]
pub struct Transaction {
    pub transaction_type: TransactionType,
    pub client: u16,
    pub amount: f32,
    pub state: Option<TransactionState>,
}

impl Transaction {
    pub fn new(transaction_type: TransactionType, client: u16, amount: f32) -> Self {
        Self {
            transaction_type,
            client,
            amount,
            state: None,
        }
    }
}
