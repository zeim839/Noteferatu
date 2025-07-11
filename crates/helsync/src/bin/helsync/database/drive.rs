use console::style;
use helsync::oauth2::Token;

use sqlx::sqlite::SqliteRow;
use sqlx::FromRow;
use sqlx::Row;

pub struct Drive {
    pub name: String,
    pub app: String,
    pub path: String,
    pub token: Option<Token>,
}

impl FromRow<'_, SqliteRow> for Drive {
    fn from_row(row: &SqliteRow) -> sqlx::Result<Self> {
        let access_token: Option<String> = row.try_get("access_token")?;
        let mut token: Option<Token> = None;
        if let Some(access_token) = access_token {
            token = Some(Token {
                access_token,
                refresh_token: row.try_get("refresh_token")?,
                created_at: row.try_get("created_at")?,
                expires_in: row.try_get("expires_in")?,
            })
        }
        Ok(Self {
            name: row.try_get("name")?,
            app: row.try_get("app")?,
            path: row.try_get("path")?,
            token,
        })
    }
}

impl std::fmt::Display for Drive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}: {}] [{}: {}] [{}: {}] [{}: {}]",
               style("Name").bold().cyan(),
               &self.name,
               style("App").bold().cyan(),
               &self.app,
               style("Path").bold().cyan(),
               &self.path,
               style("Conn").bold().cyan(),
               self.token.clone().is_some_and(|tk| !tk.is_expired()),
        )
    }
}
