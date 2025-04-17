use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a position an owner has in an asset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    id: Uuid,                         // Unique identifier
    owner_id: Uuid,                   // Reference to the Owner
    asset_id: Uuid,                   // Reference to the Asset
    status: PositionStatus,           // Current status of this position
    opened_at: DateTime<Utc>,         // When the position was first opened
    closed_at: Option<DateTime<Utc>>, // When position was closed (if applicable)
}

/// Status of a position
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PositionStatus {
    Active,
    Closed,
    Suspended,
}
