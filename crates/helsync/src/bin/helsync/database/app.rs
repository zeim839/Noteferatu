use super::database::CloudProvider;
use helsync::oauth2::{self, Config};

use console::style;

use sqlx::sqlite::SqliteRow;
use sqlx::FromRow;
use sqlx::Row;

#[derive(Clone)]
pub struct App {
    pub name: String,
    pub provider: CloudProvider,
    pub client_id: String,
    pub port: u16,
    pub client_secret: Option<String>,
}

impl FromRow<'_, SqliteRow> for App {
    fn from_row(row: &SqliteRow) -> sqlx::Result<Self> {
        Ok(Self {
            name: row.try_get("name")?,
            provider: row.try_get("provider")?,
            client_id: row.try_get("client_id")?,
            port: row.try_get("port")?,
            client_secret: row.try_get("client_secret")?,
        })
    }
}

impl Into<Config> for App {
    fn into(self) -> Config {
        let provider = self.provider.to_string();
        Config {
            auth_endpoint: oauth2::auth_endpoint(&provider),
            token_endpoint: oauth2::token_endpoint(&provider),
            client_id: self.client_id,
            client_secret: self.client_secret,
            redirect_uri: format!("http://localhost:{}", self.port),
            scope: oauth2::scope(&provider),
        }
    }
}

impl std::fmt::Display for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}: {}] [{}: {}] [{}: {}]",
               style("Name").bold().cyan(),
               &self.name,
               style("Provider").bold().cyan(),
               self.provider,
               style("Port").bold().cyan(),
               self.port)
    }
}
