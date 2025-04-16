#[allow(unused)]
use polars::prelude::DataFrame;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Owner of an asset (e.g., person holding a position (stock, fund, etc.)).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Owner {
    pub id: u32,             // Unique identifier for the owner
    pub username: String,    // Username of the Owner
    pub unique_id: String,   // Alphanumeric unique ID
    pub asset_ids: Vec<u32>, // Asset IDs in which the owner has active/closed positions.
}

impl Owner {
    /// Implementation to create a new `Owner`, name and id shall be provided.
    /// `id` (u32): Provided by the DB, incremental non-repeating.
    /// `name` (String): Provided by the username account creation.
    /// `unique_id` (String): Automatically generated uuid unique string.
    pub fn new(id: u32, username: String) -> Self {
        Self {
            id,
            username,
            unique_id: Uuid::new_v4().to_string(),
            asset_ids: Vec::new(),
        }
    }
    /// Add asset to Owner portfolio
    pub fn add_asset(&mut self, new_id: u32) {
        self.asset_ids.push(new_id);
    }
    /// Add assets to Owner portfolio
    pub fn add_assets(&mut self, new_id: Vec<u32>) {
        self.asset_ids.extend(new_id);
    }
}

/// Position in an Asset, owned by an Owner.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub isin: String,      // To match with the Asset in the DB.
    pub owner_id: u32,     // To assign to an Owner
    pub shares: DataFrame, // Dataframe of Datetime \ Shares bought \ Shares acc.
}

impl Position {
    pub fn new(isin: String, owner_id: u32, shares: DataFrame) -> Self {
        Self {
            isin,
            owner_id,
            shares,
        }
    }
}

/// Struct to define an Asset univocally, keeping its information, and positions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: u32,                           // id of the Asset in the DB.
    pub isin: String,                      // isin OR ticker of the Asset.
    pub short_name: String,                // abreviated name of the Asset.
    pub name: String,                      // long name of the Asset
    pub desc: String,                      // description of the Asset
    pub positions: HashMap<u32, Position>, // hasmap of owner_id: Position of the Asset
    pub history_data: DataFrame,           // historical data of the Asset
}

impl Asset {
    pub fn new(
        id: u32,
        isin: String,
        short_name: String,
        name: String,
        desc: String,
        positions: HashMap<u32, Position>,
        history_data: DataFrame,
    ) -> Self {
        Self {
            id,
            isin,
            short_name,
            name,
            desc,
            positions,
            history_data,
        }
    }
    pub fn update_history_data(&mut self, new_history_data: DataFrame) {
        self.history_data = new_history_data;
    }
    pub fn add_new_owner(&mut self, new_position: Position) {
        let new_owner_id = new_position.owner_id;
        self.positions.insert(new_owner_id, new_position);
    }
    pub fn update_owner_position(&mut self, updated_position: Position) {
        let owner = updated_position.owner_id;
        self.positions
            .entry(owner)
            .and_modify(|pos| *pos = updated_position.clone())
            .or_insert(updated_position);
    }
}
