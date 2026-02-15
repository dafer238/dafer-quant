// ./pmm-core/src/modules/transactions.rs

use chrono::{DateTime, Utc};
use pmm_utils::money::ScaledInt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A single transaction affecting a position
pub struct Transaction {
    id: Uuid,                          // Unique identifier
    position_id: Uuid,                 // Reference to the Position
    transaction_type: TransactionType, // Type of transaction
    datetime: DateTime<Utc>,           // When the transaction occurred
    shares: ScaledInt,                 // Number of shares involved (exact decimal)
    price: ScaledInt,                  // Price per share (exact decimal)
    fees: ScaledInt,                   // Transaction fees/commissions (exact decimal)
    notes: Option<String>,             // Optional notes about the transaction
    transfer_id: Option<Uuid>,         // Reference to a transfer if this transaction is part of one
}

impl Transaction {
    /// Compute the total cost of this transaction: `shares * price + fees`.
    ///
    /// Uses exact scaled-integer arithmetic — no floating-point rounding.
    pub fn total_cost(&self) -> ScaledInt {
        (self.shares * self.price) + self.fees
    }

    /// Compute the net amount (total cost with sign based on transaction type).
    ///
    /// - Buy / Fee: negative (cash outflow)
    /// - Sell / Dividend: positive (cash inflow)
    /// - Other types: positive by default
    pub fn net_amount(&self) -> ScaledInt {
        let cost = self.total_cost();
        match self.transaction_type {
            TransactionType::Buy | TransactionType::Fee => -cost,
            _ => cost,
        }
    }
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
