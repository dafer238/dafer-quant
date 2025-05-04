// ./aljsl/src/errors.rs

use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JSLError {
    #[error("Database error: {0}")]
    Db(#[from] SqlxError),

    #[error("Username or email is already taken.")]
    UsernameOrEmailTaken,

    #[error("Invalid credentials.")]
    InvalidCredentials,

    #[error("Owner not found.")]
    OwnerNotFound,

    #[error("Position not found.")]
    PositionNotFound,

    #[error("Transaction not found.")]
    TransactionNotFound,

    #[error("Transfer not found.")]
    TransferNotFound,

    #[error("Failed to serialize or deserialize data: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Unexpected internal error.")]
    Internal,
}
