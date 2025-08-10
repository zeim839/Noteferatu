use super::conversation::Conversation;
use crate::core::{Error, Result, Model};
use super::context::Context;
use super::agent::Agent;
use crate::core::Client as _;

use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::{RwLock, Mutex};
use std::collections::HashMap;
use database::Database;
use std::sync::Arc;

/// Manages conversation threads.
///
/// Satisfies chat completion [requests](crate::core::Request) by
/// routing models to their provider APIs. Preserves conversations and
/// messages in an SQLite [database].
pub struct Manager {
    db: Arc<Database>,
    agent: Arc<RwLock<Agent>>,
    cache: Mutex<HashMap<i64, Arc<Context>>>,
}

impl Manager {

    /// Creates a new [Manager] from an existing [database] instance.
    pub fn new(db: Arc<Database>) -> Self {
        Self {
            agent: Arc::new(RwLock::new(Agent::new())),
            cache: Mutex::new(HashMap::new()),
            db,
        }
    }

    /// List all models available to the manager.
    pub async fn list_models(&self) -> Result<Vec<Model>> {
        self.agent.read().await.list_models().await
    }

    /// List all stored [Conversation](super::Conversation)s.
    pub async fn list_conversations(&self) -> Result<Vec<Conversation>> {
        let mut conn = self.db.acquire().await?;
        let convs: Vec<Conversation> = sqlx::query_as("SELECT * FROM
    Conversation ORDER BY created_at DESC")
            .fetch_all(&mut *conn)
            .await?;

        Ok(convs)
    }

    /// Create a new [Conversation](super::Conversation).
    pub async fn create_conversation(&self, name: &str) -> Result<Conversation> {
        let mut conn = self.db.acquire().await?;
        let created_at: i64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let res = sqlx::query("INSERT INTO Conversation(name,
        created_at) VALUES (?, ?)")
            .bind(name)
            .bind(created_at)
            .execute(&mut *conn)
            .await?;

        Ok(Conversation {
            id: res.last_insert_rowid().into(),
            name: name.to_string(),
            created_at,
        })
    }

    /// Delete a [Conversation](super::Conversation).
    pub async fn remove_conversation(&self, id: i64) -> Result<()> {
        let mut conn = self.db.acquire().await?;
        let res = sqlx::query("DELETE FROM Conversation WHERE id=?")
            .bind(id)
            .execute(&mut *conn)
            .await?;

        if res.rows_affected() == 0 {
            return Err(Error::Sql(sqlx::Error::RowNotFound.to_string()));
        }

        Ok(())
    }

    /// Rename a [Conversation](super::Conversation).
    pub async fn rename_conversation(&self, id: i64, new_name: &str) -> Result<()> {
        let mut conn = self.db.acquire().await?;
        let res = sqlx::query("UPDATE Conversation SET name=? WHERE id=?")
            .bind(new_name)
            .bind(id)
            .execute(&mut *conn)
            .await?;

        if res.rows_affected() == 0 {
            return Err(Error::Sql(sqlx::Error::RowNotFound.to_string()));
        }

        Ok(())
    }

    /// Retrieve a conversation's [Context](super::Context).
    ///
    /// Use a [Context](super::Context) to generate chat completions
    /// and automatically write responses to the [database].
    pub async fn get_conversation(&self, id: i64) -> Result<Arc<Context>> {
        let mut cache = self.cache.lock().await;
        if let Some(ctx) = cache.get(&id) {
            return Ok(ctx.clone());
        }

        let mut conn = self.db.acquire().await?;
        let conv: Conversation = sqlx::query_as("SELECT * FROM Conversation WHERE id=?")
            .bind(id)
            .fetch_one(&mut *conn)
            .await?;

        let ctx = Context::new(self.db.clone(), self.agent.clone(), conv);
        let ctx = Arc::new(ctx);
        cache.insert(id, ctx.clone());

        Ok(ctx.clone())
    }

    /// Connect an [Anthropic](crate::providers::anthropic) client.
    pub async fn connect_anthropic(&self, api_key: &str) -> Result<()> {
        let mut agent = self.agent.write().await;
        agent.connect_anthropic(api_key).await
    }

    /// Connect a [Google](crate::providers::google) client.
    pub async fn connect_google(&self, api_key: &str) -> Result<()> {
        let mut agent = self.agent.write().await;
        agent.connect_google(api_key).await
    }

    /// Connect an [Ollama](crate::providers::ollama) client.
    pub async fn connect_ollama(&self, api_key: &str) -> Result<()> {
        let mut agent = self.agent.write().await;
        agent.connect_ollama(api_key).await
    }

    /// Connect an [OpenAI](crate::providers::openai) client.
    pub async fn connect_openai(&self, api_key: &str) -> Result<()> {
        let mut agent = self.agent.write().await;
        agent.connect_openai(api_key).await
    }

    /// Connect an [OpenRouter](crate::providers::openrouter) client.
    pub async fn connect_openrouter(&self, api_key: &str) -> Result<()> {
        let mut agent = self.agent.write().await;
        agent.connect_openrouter(api_key).await
    }
}

#[cfg(test)]
pub(crate) mod tests {

    use dotenv::dotenv;
    use std::env;
    use crate::core::Client as _;

    use super::*;
    use std::sync::Arc;
    use tokio::sync::OnceCell;
    use std::time::{SystemTime, UNIX_EPOCH};

    use database::{Database, Migration, MigrationType, Config};
    use crate::agent::SCHEMA_VERSION_0;
    use crate::providers::ollama::Client as Ollama;

    static MANAGER: OnceCell<Arc<Manager>> = OnceCell::const_new();

    pub(crate) async fn get_manager() -> Arc<Manager> {
        MANAGER.get_or_init(|| async {
            dotenv().ok();
            let db_name = "./agent-test-db.sqlite";

            let _ = std::fs::remove_file(db_name);
            let _ = std::fs::remove_file(&format!("{db_name}-shm"));
            let _ = std::fs::remove_file(&format!("{db_name}-wal"));

            const TESTING_SCHEMA: &str = r#"
INSERT INTO Conversation VALUES
  (0, "My Conversation", 0),
  (1, "My Other Conversation", 0);
"#;

            let db = Database::new(&Config {
                max_connections: 1,
                local_path: db_name.to_string(),
                migrations: vec![
                    Migration {
                        version: 0,
                        sql: SCHEMA_VERSION_0.to_string(),
                        kind: MigrationType::Up,
                    },
                    Migration {
                        version: 1,
                        sql: TESTING_SCHEMA.to_string(),
                        kind: MigrationType::Up,
                    },
                ],
            }).await.unwrap();

            let mgr = Manager::new(Arc::new(db));
            if let Ok(key) = env::var("ANTHROPIC_API_KEY") {
                if !key.is_empty() {
                    mgr.connect_anthropic(&key).await.unwrap();
                }
            }

            if let Ok(key) = env::var("GOOGLE_API_KEY") {
                if !key.is_empty() {
                    mgr.connect_google(&key).await.unwrap();
                }
            }

            let ollama_client = Ollama::new("http://localhost:11434");
            if ollama_client.check().await.is_ok() {
                mgr.connect_ollama("http://localhost:11434").await.unwrap();
            }

            if let Ok(key) = env::var("OPENAI_API_KEY") {
                if !key.is_empty() {
                    mgr.connect_openai(&key).await.unwrap();
                }
            }

            if let Ok(key) = env::var("OPENROUTER_API_KEY") {
                if !key.is_empty() {
                    mgr.connect_openrouter(&key).await.unwrap();
                }
            }

            Arc::new(mgr)
        }).await.clone()
    }

    #[tokio::test]
    async fn test_list_conversations() {
        let manager = get_manager().await;
        let convs = manager.list_conversations().await.unwrap();
        assert!(convs.len() >= 2);
        for item in convs {
            if item.id == 0 {
                assert!(item.name == "My Conversation");
            }
            if item.id == 1 {
                assert!(item.name == "My Other Conversation");
            }
        }
    }

    #[tokio::test]
    async fn test_create_conversation() {
        let manager = get_manager().await;
        let created_at: i64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let conv = manager.create_conversation("test create new conversation")
            .await.unwrap();

        assert!(conv.created_at >= created_at);
        assert!(conv.name == "test create new conversation");

        let mut has_new_conv = false;
        for item in manager.list_conversations().await.unwrap() {
            if item.id == conv.id {
                assert!(item.name == "test create new conversation");
                has_new_conv = true;
                break;
            }
        }

        assert!(has_new_conv);
    }

    #[tokio::test]
    async fn test_remove_conversation() {
        let manager = get_manager().await;
        let conv = manager.create_conversation("delete me")
            .await.unwrap();

        manager.remove_conversation(conv.id).await.unwrap();
        for item in manager.list_conversations().await.unwrap() {
            if item.id == conv.id {
                panic!("conversation was not deleted");
            }
        }
    }

    #[tokio::test]
    async fn test_rename_conversation() {
        let client = get_manager().await;
        let conv = client.create_conversation("rename me")
            .await.unwrap();

        client.rename_conversation(conv.id, "renamed you")
            .await.unwrap();

        let mut has_renamed_conv = false;
        for item in client.list_conversations().await.unwrap() {
            if item.id == conv.id {
                assert!(item.name == "renamed you");
                has_renamed_conv = true;
                break;
            }
        }

        assert!(has_renamed_conv);
    }

    #[tokio::test]
    async fn test_get_conversation() {
        let client = get_manager().await;
        assert!(client.get_conversation(0).await.is_ok());
        assert!(client.get_conversation(69).await.is_err());
    }

    #[tokio::test]
    async fn list_models() {
        let client = get_manager().await;
        let models = client.list_models().await.unwrap();
        assert!(models.len() > 0);
    }
}
