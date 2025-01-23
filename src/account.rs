use crate::transaction::Transaction;
use crate::TransactionData;
use std::collections::HashMap;

/// Represents a single account
#[derive(Debug, PartialEq)]
pub struct Account {
    pub id: u16,
    pub available: f32,
    pub held: f32,
    pub locked: bool,
    pub transactions: HashMap<u32, Transaction>,
}

impl Account {
    pub(crate) fn new(id: u16) -> Self {
        Self {
            id,
            available: 0.0,
            held: 0.0,
            locked: false,
            transactions: HashMap::new(),
        }
    }

    pub(crate) fn update(&mut self, transaction: TransactionData) {
        if self.id == transaction.client {
            transaction.process(self);
        }
    }
}
