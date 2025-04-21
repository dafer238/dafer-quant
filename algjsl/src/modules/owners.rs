use crate::database::sqlite_db::Database;
use argon2::password_hash::{SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHasher};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

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

impl fmt::Display for UserGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_str = match self {
            UserGroup::Admin => "Admin",
            UserGroup::Trader => "Trader",
            UserGroup::Institution => "Institution",
            UserGroup::Viewer => "Viewer",
        };
        write!(f, "{}", as_str)
    }
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_str = match self {
            Theme::Light => "Light",
            Theme::Dark => "Dark",
        };
        write!(f, "{}", as_str)
    }
}

impl FromStr for UserGroup {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Admin" => Ok(UserGroup::Admin),
            "Trader" => Ok(UserGroup::Trader),
            "Institution" => Ok(UserGroup::Institution),
            "Viewer" => Ok(UserGroup::Viewer),
            _ => Err(()),
        }
    }
}

impl FromStr for Theme {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Light" => Ok(Theme::Light),
            "Dark" => Ok(Theme::Dark),
            _ => Err(()),
        }
    }
}

impl Owner {
    pub fn new(
        username: String,
        email: String,
        raw_password: String,
        usergroup: Option<UserGroup>,
        preferences: Option<UserPreferences>,
    ) -> Self {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(raw_password.as_bytes(), &salt)
            .expect("Password hashing failed")
            .to_string();

        let now = Utc::now();

        Owner {
            id: Uuid::new_v4(),
            username,
            email,
            password_hash,
            salt: salt.as_str().to_string(),
            created_at: now,
            last_connection: now,
            online: false,
            usergroup: usergroup.unwrap_or(UserGroup::Trader),
            preferences: preferences.unwrap_or(UserPreferences {
                language: "en".to_string(),
                theme: Theme::Dark,
                notifications_enabled: true,
                cookie_consent: true,
            }),
        }
    }
    pub async fn get_owner_by_id(db: &Database, id: Uuid) -> Result<Owner, sqlx::Error> {
        let id_str = id.to_string();
        let row = sqlx::query!(
            r#"
            SELECT * FROM owners WHERE id = ?
            "#,
            id_str,
        )
        .fetch_one(&*db.pool)
        .await?;

        let preferences: UserPreferences =
            serde_json::from_str(&row.preferences).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        Ok(Owner {
            id: Uuid::parse_str(
                row.id
                    .as_ref()
                    .ok_or(sqlx::Error::ColumnNotFound("id".into()))?,
            )
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?,
            username: row.username,
            email: row.email,
            password_hash: row.password_hash,
            salt: row.salt,
            created_at: DateTime::parse_from_rfc3339(&row.created_at)
                .map_err(|e| sqlx::Error::Decode(Box::new(e)))?
                .with_timezone(&Utc),
            last_connection: DateTime::parse_from_rfc3339(&row.last_connection)
                .map_err(|e| sqlx::Error::Decode(Box::new(e)))?
                .with_timezone(&Utc),
            online: row.online != 0,
            usergroup: UserGroup::from_str(&row.usergroup)
                .map_err(|_| sqlx::Error::Decode("Invalid usergroup".into()))?,
            preferences,
        })
    }

    pub async fn get_all_owners(db: &Database) -> Result<Vec<Owner>, sqlx::Error> {
        let rows = sqlx::query!(
            r#"
            SELECT * FROM owners
            "#
        )
        .fetch_all(&*db.pool)
        .await?;

        let mut owners = Vec::new();

        for row in rows {
            let preferences: UserPreferences = serde_json::from_str(&row.preferences)
                .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
            owners.push(Owner {
                id: Uuid::parse_str(
                    row.id
                        .as_ref()
                        .ok_or(sqlx::Error::ColumnNotFound("id".into()))?,
                )
                .map_err(|e| sqlx::Error::Decode(Box::new(e)))?,
                username: row.username,
                email: row.email,
                password_hash: row.password_hash,
                salt: row.salt,
                created_at: DateTime::parse_from_rfc3339(&row.created_at)
                    .map_err(|e| sqlx::Error::Decode(Box::new(e)))?
                    .with_timezone(&Utc),
                last_connection: DateTime::parse_from_rfc3339(&row.last_connection)
                    .map_err(|e| sqlx::Error::Decode(Box::new(e)))?
                    .with_timezone(&Utc),
                online: row.online != 0,
                usergroup: UserGroup::from_str(&row.usergroup)
                    .map_err(|_| sqlx::Error::Decode("Invalid usergroup".into()))?,
                preferences,
            });
        }

        Ok(owners)
    }

    pub async fn create_owner(db: &Database, owner: &Owner) -> Result<(), sqlx::Error> {
        let id_str = owner.id.to_string();
        let created_at_str = owner.created_at.to_rfc3339();
        let last_connection_str = owner.last_connection.to_rfc3339();
        let usergroup_str = owner.usergroup.to_string();
        let preferences_json = serde_json::to_string(&owner.preferences)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let online_i32 = owner.online as i32;

        sqlx::query!(
            r#"
            INSERT INTO owners (
                id, username, email, password_hash, salt,
                created_at, last_connection, online,
                usergroup, preferences
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            id_str,
            owner.username,
            owner.email,
            owner.password_hash,
            owner.salt,
            created_at_str,
            last_connection_str,
            online_i32,
            usergroup_str,
            preferences_json
        )
        .execute(&*db.pool)
        .await?;

        Ok(())
    }

    pub async fn update_owner(db: &Database, owner: &Owner) -> Result<(), sqlx::Error> {
        let id_str = owner.id.to_string();
        let created_at_str = owner.created_at.to_rfc3339();
        let last_connection_str = owner.last_connection.to_rfc3339();
        let usergroup_str = owner.usergroup.to_string();
        let online_i32 = owner.online as i32;
        let preferences_json = serde_json::to_string(&owner.preferences)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        sqlx::query!(
            r#"
            UPDATE owners
            SET username = ?, email = ?, password_hash = ?, salt = ?,
                created_at = ?, last_connection = ?, online = ?,
                usergroup = ?, preferences = ?
            WHERE id = ?
            "#,
            owner.username,
            owner.email,
            owner.password_hash,
            owner.salt,
            created_at_str,
            last_connection_str,
            online_i32,
            usergroup_str,
            preferences_json,
            id_str
        )
        .execute(&*db.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_owner(db: &Database, id: Uuid) -> Result<(), sqlx::Error> {
        let id_str = id.to_string();

        sqlx::query!(
            r#"
            DELETE FROM owners WHERE id = ?
            "#,
            id_str
        )
        .execute(&*db.pool)
        .await?;

        Ok(())
    }
}
