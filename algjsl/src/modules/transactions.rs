// ./aljsl/src/modules/transactions.rs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A single transaction affecting a position
pub struct Transaction {
    id: Uuid,                          // Unique identifier
    position_id: Uuid,                 // Reference to the Position
    transaction_type: TransactionType, // Type of transaction
    datetime: DateTime<Utc>,           // When the transaction occurred
    shares: f64,                       // Number of shares involved
    price: f64,                        // Price per share
    fees: f64,                         // Transaction fees/commissions
    notes: Option<String>,             // Optional notes about the transaction
    transfer_id: Option<Uuid>,         // Reference to a transfer if this transaction is part of one
}

/// Types of transactions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionType {
    Buy,
    Sell,
    Dividend,
    Split,
    Transfer,
    Fee,
    Other(String),
}

/// For transfers between assets
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transfer {
    id: Uuid,                  // Unique identifier
    from_transaction_id: Uuid, // Transaction representing the outgoing side
    to_transaction_id: Uuid,   // Transaction representing the incoming side
    reason: TransferReason,    // Why the transfer occurred
    datetime: DateTime<Utc>,   // When the transfer occurred
    notes: Option<String>,     // Optional notes about the transfer
}

/// Reasons for transfers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransferReason {
    Exchange,
    Conversion,
    Migration,
    Other(String),
}
