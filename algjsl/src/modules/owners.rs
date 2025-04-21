use crate::database::sqlite_db::Database;
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
    pub async fn create_owner(db: &Database, owner: &Owner) -> Result<(), sqlx::Error> {
        let preferences_json = serde_json::to_string(&owner.preferences)?;

        sqlx::query!(
            r#"
            INSERT INTO owners (
                id, username, email, password_hash, salt,
                created_at, last_connection, online,
                usergroup, preferences
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            owner.id.to_string(),
            owner.username,
            owner.email,
            owner.password_hash,
            owner.salt,
            owner.created_at.to_rfc3339(),
            owner.last_connection.to_rfc3339(),
            owner.online as i32,
            owner.usergroup.to_string(),
            preferences_json
        )
        .execute(&*db.pool)
        .await?;

        Ok(())
    }

    pub async fn get_owner_by_id(db: &Database, id: Uuid) -> Result<Owner, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            SELECT * FROM owners WHERE id = ?
            "#,
            id.to_string()
        )
        .fetch_one(&*db.pool)
        .await?;

        let preferences: UserPreferences = serde_json::from_str(&row.preferences)?;
        Ok(Owner {
            id: Uuid::parse_str(&row.id)?,
            username: row.username,
            email: row.email,
            password_hash: row.password_hash,
            salt: row.salt,
            created_at: DateTime::parse_from_rfc3339(&row.created_at)?.with_timezone(&Utc),
            last_connection: DateTime::parse_from_rfc3339(&row.last_connection)?
                .with_timezone(&Utc),
            online: row.online != 0,
            usergroup: UserGroup::from_str(&row.usergroup),
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
            let preferences: UserPreferences = serde_json::from_str(&row.preferences)?;
            owners.push(Owner {
                id: Uuid::parse_str(&row.id)?,
                username: row.username,
                email: row.email,
                password_hash: row.password_hash,
                salt: row.salt,
                created_at: DateTime::parse_from_rfc3339(&row.created_at)?.with_timezone(&Utc),
                last_connection: DateTime::parse_from_rfc3339(&row.last_connection)?
                    .with_timezone(&Utc),
                online: row.online != 0,
                usergroup: UserGroup::from_str(&row.usergroup),
                preferences,
            });
        }

        Ok(owners)
    }

    pub async fn update_owner(db: &Database, owner: &Owner) -> Result<(), sqlx::Error> {
        let preferences_json = serde_json::to_string(&owner.preferences)?;

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
            owner.created_at.to_rfc3339(),
            owner.last_connection.to_rfc3339(),
            owner.online as i32,
            owner.usergroup.to_string(),
            preferences_json,
            owner.id.to_string()
        )
        .execute(&*db.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_owner(db: &Database, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM owners WHERE id = ?
            "#,
            id.to_string()
        )
        .execute(&*db.pool)
        .await?;

        Ok(())
    }
}
