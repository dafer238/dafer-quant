use anyhow::Result;
use serde::Deserialize;
use std::env;

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub port: u16,
    pub environment: Environment,
    pub database_url: String,
    // Add more configuration fields as needed
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Development,
    Production,
    Testing,
}

impl AppConfig {
    /// Loads the application configuration from environment variables and .env file
    pub fn load() -> Result<Self> {
        // Load .env file if it exists
        dotenv::dotenv().ok();

        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()?;

        let environment = match env::var("ENVIRONMENT")
            .unwrap_or_else(|_| "development".to_string())
            .to_lowercase()
            .as_str()
        {
            "production" => Environment::Production,
            "testing" => Environment::Testing,
            _ => Environment::Development,
        };

        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://localhost/my_app".to_string());

        Ok(Self {
            port,
            environment,
            database_url,
        })
    }

    /// Returns true if the application is running in development mode
    pub fn is_development(&self) -> bool {
        self.environment == Environment::Development
    }

    /// Returns true if the application is running in production mode
    pub fn is_production(&self) -> bool {
        self.environment == Environment::Production
    }
}
