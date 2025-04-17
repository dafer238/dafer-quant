// use argon2::{self, Config, ThreadMode, Variant, Version};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserGroup {
    Admin,
    Trader,
    Institution,
    Viewer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub language: String, // e.g., "en", "es", "fr"
    pub theme: Theme,     // Light or Dark
    pub notifications_enabled: bool,
    pub cookie_consent: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
}

/// Owner of an asset (e.g., person holding a position (stock, fund, etc.)).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Owner {
    pub id: Uuid,                       // Unique identifier for the owner
    pub username: String,               // Username of the Owner
    pub email: String,                  // Email of the Owner
    pub password_hash: String,          // Stored password hash
    pub salt: String,                   // Unique salt used for this password
    pub created_at: DateTime<Utc>,      // Datetime of account creation
    pub last_connection: DateTime<Utc>, // Datetime of the last connection
    pub online: bool,                   // Bool is the user connected
    pub usergroup: UserGroup,           // Usergroup of the Owner
    pub preferences: UserPreferences,   // Preferences of the user
}

// FIXME: ALL BELOW AND IMPORTS TO FIX

impl Owner {
    // /// Implementation to create a new `Owner`, username and id shall be provided.
    // /// `id` (u32): Provided by the DB, incremental non-repeating.
    // /// `unique_id` (String): Automatically generated uuid unique string.
    // /// `username` (String): Provided by the username account creation.
    // /// `assets_id` (Vec<u32>): Vector containing the id of the assets possessed by the owner.
    // // Create a new owner with a hashed password
    // // Create a new owner with a hashed password
    // pub fn new(
    //     username: String,
    //     email: String,
    //     plain_password: &str,
    //     usergroup: UserGroup,
    //     preferences: UserPreferences,
    // ) -> Result<Self, argon2::Error> {
    //     // Generate a random salt
    //     let mut salt = [0u8; 16];
    //     OsRng.fill_bytes(&mut salt);
    //     let salt_str = base64::encode(&salt);
    //
    //     // Hash the password
    //     let password_hash = Self::hash_password(plain_password, &salt_str)?;
    //
    //     let now = Utc::now();
    //
    //     Ok(Self {
    //         id: Uuid::new_v4(),
    //         username,
    //         email,
    //         password_hash,
    //         salt: salt_str,
    //         created_at: now,
    //         last_connection: now,
    //         online: false,
    //         usergroup,
    //         preferences,
    //     })
    // }

    // Hash a password using Argon2
    // fn hash_password(password: &str, salt: &str) -> Result<String, argon2::Error> {
    //     let config = Config {
    //         variant: Variant::Argon2id,  // Recommended variant
    //         version: Version::Version13, // Latest version
    //         mem_cost: 65536,             // Memory cost (higher is more secure but slower)
    //         time_cost: 4,                // Iterations (higher is more secure but slower)
    //         lanes: 4,                    // Parallelism factor
    //         thread_mode: ThreadMode::Parallel,
    //         secret: &[],     // Optional secret key
    //         ad: &[],         // Associated data
    //         hash_length: 32, // Output hash length
    //     };
    //
    //     let hash = argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config)?;
    //
    //     Ok(hash)
    // }

    // Verify a password against the stored hash
    // pub fn verify_password(&self, password: &str) -> Result<bool, argon2::Error> {
    //     argon2::verify_encoded(&self.password_hash, password.as_bytes())
    // }

    // Update the password
    // pub fn update_password(&mut self, new_password: &str) -> Result<(), argon2::Error> {
    //     // Generate a new salt for the password change
    //     let mut salt = [0u8; 16];
    //     OsRng.fill_bytes(&mut salt);
    //     let salt_str = base64::encode(&salt);
    //
    //     // Hash the new password
    //     self.password_hash = Self::hash_password(new_password, &salt_str)?;
    //     self.salt = salt_str;
    //
    //     Ok(())
    // }
}
