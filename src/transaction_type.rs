use serde::Deserialize;

/// Represents the possible transaction types
#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}
