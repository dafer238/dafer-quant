#[allow(unused)]
use polars::prelude::DataFrame;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
/// Owner of an asset (e.g., person holding a position (stock, fund, etc.)).
pub struct Owner {
    id: u32,             // Unique identifier for the owner
    username: String,    // Username of the Owner
    unique_id: String,   // Alphanumeric unique ID
    assets: Vec<String>, // Assets in which the owner has active/closed positions.
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
            assets: Vec::new(),
            unique_id: Uuid::new_v4().to_string(),
        }
    }
    /// Add asset to Owner portfolio
    pub fn add_asset(&mut self, new_isin: String) {
        self.assets.push(new_isin);
    }
    /// Add assets to Owner portfolio
    pub fn add_assets(&mut self, new_isins: Vec<String>) {
        self.assets.extend(new_isins);
    }
}

#[derive(Debug, Clone)]
/// Position in an Asset, owned by an Owner.
pub struct Position {
    isin: String,      // To match with the Asset in the DB.
    owner: Owner,      // To assign to an Owner
    shares: DataFrame, // Dataframe of Datetime \ Shares bought \ Shares acc.
}

impl Position {
    pub fn new(isin: String, owner: Owner, shares: DataFrame) -> Self {
        Self {
            isin,
            owner,
            shares,
        }
    }
}

#[derive(Debug, Clone)]
/// Struct to define an Asset univocally, keeping its information, and positions.
pub struct Asset {
    id: u32,                              // id of the Asset in the DB.
    isin: String,                         // isin OR ticker of the Asset.
    short_name: String,                   // abreviated name of the Asset.
    name: String,                         // long name of the Asset
    desc: String,                         // description of the Asset
    positions: HashMap<String, Position>, // hasmap of Owner: Position of the Asset
    history_data: DataFrame,              // historical data of the Asset
}

impl Asset {
    pub fn new(
        id: u32,
        isin: String,
        short_name: String,
        name: String,
        desc: String,
        positions: HashMap<String, Position>,
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
        let new_owner = new_position.owner.username.clone();
        self.positions.insert(new_owner, new_position);
    }
    pub fn update_owner_position(&mut self, updated_position: Position) {
        let owner = updated_position.owner.username.clone();
        self.positions
            .entry(owner)
            .and_modify(|pos| *pos = updated_position.clone())
            .or_insert(updated_position);
    }
}
