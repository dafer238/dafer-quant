use polars::prelude::DataFrame;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Struct to define an Asset univocally, keeping its information, and positions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: Uuid,                        // Id of the Asset in the DB.
    pub isin: String,                    // Isin OR ticker of the Asset.
    pub short_name: String,              // Abreviated name of the Asset.
    pub name: String,                    // Long name of the Asset
    pub description: String,             // Description of the Asset
    pub asset_type: AssetType,           // Type of the Asset
    pub history_data: Option<DataFrame>, // Historical data of the Asset
}

impl Asset {
    pub fn new(
        isin: String,
        short_name: String,
        name: String,
        description: String,
        asset_type: AssetType,
        history_data: Option<DataFrame>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            isin,
            short_name,
            name,
            description,
            asset_type,
            history_data,
        }
    }
    // TODO: Update checking if there is already some data saved. Read first
    // update and then save.
    pub fn update_history_data(&mut self, new_history_data: Option<DataFrame>) {
        self.history_data = new_history_data;
    }
}

/// Types of assets supported by the system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AssetType {
    Stock,
    Bond,
    ETF,
    MutualFund,
    Cryptocurrency,
    RealEstate,
    Cash,
    Other(String),
}
