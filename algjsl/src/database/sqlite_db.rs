use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::sync::Arc;

#[derive(Clone)]
pub struct Database {
    pub pool: Arc<SqlitePool>,
}

impl Database {
    pub async fn new(db_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await?;

        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    pub async fn init_db(&self) -> Result<(), sqlx::Error> {
        // Enable foreign key constraints
        sqlx::query("PRAGMA foreign_keys = ON;")
            .execute(&*self.pool)
            .await?;

        // Create Owners table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS owners (
                id TEXT PRIMARY KEY,
                username TEXT NOT NULL,
                email TEXT NOT NULL,
                password_hash TEXT NOT NULL,
                salt TEXT NOT NULL,
                created_at TEXT NOT NULL,
                last_connection TEXT NOT NULL,
                online INTEGER NOT NULL,
                usergroup TEXT NOT NULL,
                preferences TEXT NOT NULL
            );
        "#,
        )
        .execute(&*self.pool)
        .await?;

        // Create Assets table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS assets (
                id TEXT PRIMARY KEY,
                isin TEXT NOT NULL,
                short_name TEXT NOT NULL,
                name TEXT NOT NULL,
                description TEXT NOT NULL,
                asset_type TEXT NOT NULL,
                history_data TEXT
            );
        "#,
        )
        .execute(&*self.pool)
        .await?;

        // Create Positions table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS positions (
                id TEXT PRIMARY KEY,
                owner_id TEXT NOT NULL,
                asset_id TEXT NOT NULL,
                status TEXT NOT NULL,
                opened_at TEXT NOT NULL,
                closed_at TEXT,
                FOREIGN KEY (owner_id) REFERENCES owners(id),
                FOREIGN KEY (asset_id) REFERENCES assets(id)
            );
        "#,
        )
        .execute(&*self.pool)
        .await?;

        // Create Transactions table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS transactions (
                id TEXT PRIMARY KEY,
                position_id TEXT NOT NULL,
                transaction_type TEXT NOT NULL,
                datetime TEXT NOT NULL,
                shares REAL NOT NULL,
                price REAL NOT NULL,
                fees REAL NOT NULL,
                notes TEXT,
                transfer_id TEXT,
                FOREIGN KEY (position_id) REFERENCES positions(id),
                FOREIGN KEY (transfer_id) REFERENCES transfers(id)
            );
        "#,
        )
        .execute(&*self.pool)
        .await?;

        // Create Transfers table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS transfers (
                id TEXT PRIMARY KEY,
                from_transaction_id TEXT NOT NULL,
                to_transaction_id TEXT NOT NULL,
                reason TEXT NOT NULL,
                datetime TEXT NOT NULL,
                notes TEXT,
                FOREIGN KEY (from_transaction_id) REFERENCES transactions(id),
                FOREIGN KEY (to_transaction_id) REFERENCES transactions(id)
            );
        "#,
        )
        .execute(&*self.pool)
        .await?;

        Ok(())
    }
}
