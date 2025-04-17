#[allow(unused)]
use chrono::{DateTime, Utc};
use polars::prelude::DataFrame;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

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
