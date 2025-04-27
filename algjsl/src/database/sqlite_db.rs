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

        // Create Owners table (UUID as primary key)
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS owners (
                uuid TEXT PRIMARY KEY,
                username TEXT UNIQUE NOT NULL,
                email TEXT UNIQUE NOT NULL,
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

        // Create Assets table (ISIN as primary key)
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS assets (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                isin TEXT UNIQUE NOT NULL,
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

        // Create Positions table (autoincrementing id)
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS positions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                owner_uuid TEXT NOT NULL,
                asset_id TEXT NOT NULL,
                status TEXT NOT NULL,
                opened_at TEXT NOT NULL,
                closed_at TEXT,
                FOREIGN KEY (owner_uuid) REFERENCES owners(uuid),
                FOREIGN KEY (asset_id) REFERENCES assets(id)
            );
        "#,
        )
        .execute(&*self.pool)
        .await?;

        // Create Transactions table (autoincrementing id)
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS transactions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                position_id INTEGER NOT NULL,
                transaction_type TEXT NOT NULL,
                datetime TEXT NOT NULL,
                shares REAL NOT NULL,
                price REAL NOT NULL,
                fees REAL NOT NULL,
                notes TEXT,
                transfer_id INTEGER,
                FOREIGN KEY (position_id) REFERENCES positions(id),
                FOREIGN KEY (transfer_id) REFERENCES transfers(id)
            );
        "#,
        )
        .execute(&*self.pool)
        .await?;

        // Create Transfers table (autoincrementing id)
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS transfers (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                from_transaction_id INTEGER NOT NULL,
                to_transaction_id INTEGER NOT NULL,
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
