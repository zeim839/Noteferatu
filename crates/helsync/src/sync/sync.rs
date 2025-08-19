use crate::core::{FileSystem, File, Result, Delta};
use crate::local::{Client, LocalFile};

use std::collections::HashMap;
use std::sync::Arc;

pub struct Sync<R: FileSystem + Delta> {
    local: Arc<Client>,
    remote: Arc<R>,
    local_token: Option<String>,
    remote_token: Option<String>,
    remote_root: Option<String>,
}

impl<R: FileSystem<Error = crate::core::Error> + Delta> Sync<R> {

    /// Create a new instance of [Sync].
    pub fn new(local: Arc<Client>, remote: Arc<R>) -> Self {
        Self {
            local, remote,
            local_token: None,
            remote_token: None,
            remote_root: None
        }
    }

    /// Fetch combined local and remote deltas.
    ///
    /// Calls [list_deltas](crate::core::FileSystem::list_deltas] for
    /// the local and remote file systems, grouping deltas by their
    /// respective file IDs.
    ///
    /// If `step` is set to true, then the internal delta tokens are
    /// advanced.
    pub async fn list_deltas(&mut self, step: bool) -> Result<Vec<Unreconciled>> {
        let (local_deltas, new_local_token) = self.local
            .list_deltas(self.local_token.as_deref()).await?;

        let (remote_deltas, new_remote_token) = self.remote
            .list_deltas(self.remote_token.as_deref()).await?;

        let mut deltas_map = HashMap::<String, Unreconciled>::new();
        for delta in local_deltas {
            // If the key is already registered, keep the delta with
            // the latest modified_at.
            if let Some(mapped) = deltas_map.get(&delta.id.to_string()) {
                if delta.modified_at > mapped.local.clone().unwrap().modified_at {
                    deltas_map.insert(delta.id.to_string(), Unreconciled {
                        id: delta.clone().remote_id.unwrap_or(delta.id.to_string()),
                        local: Some(delta),
                        remote: None,
                    });
                    continue;
                }
                continue;
            }
            deltas_map.insert(delta.id.to_string(), Unreconciled {
                id: delta.clone().remote_id.unwrap_or(delta.id.to_string()),
                local: Some(delta),
                remote: None,
            });
        }

        for delta in remote_deltas.into_iter().map(|d| Into::<File>::into(d)) {
            if let Some(mapped) = deltas_map.get(&delta.id) {
                let mut mapped: Unreconciled = mapped.clone();
                if let Some(remote) = mapped.remote {
                    if delta.modified_at <= remote.modified_at {
                        continue;
                    }
                }
                mapped.remote = Some(delta);
                deltas_map.insert(mapped.id.clone(), mapped);
                continue;
            }
            deltas_map.insert(delta.id.clone(), Unreconciled {
                id: delta.id.clone(),
                local: None,
                remote: Some(delta),
            });
        }

        if step {
            self.local_token = Some(new_local_token);
            self.remote_token = Some(new_remote_token);
        }

        Ok(deltas_map.into_iter().map(|(_, v)| v).collect())
    }

    /// Performs a full synchronization across two file systems.
    ///
    /// Synchronizes two separate file systems by comparing each of
    /// their files. It is recommended to call this function to
    /// completion at the beginning of the program's runtime, and then
    /// to periodically call [sync_changes].
    ///
    /// Calling `sync_full` ensures that the two filesystems are
    /// synchronized to a consistent state. Subsequent calls to
    /// [sync_changes] will have a higher probability of being
    /// accepted.
    pub async fn sync_full(&self) -> Result<()> {
        unimplemented!();
    }

    /// Synchronizes only the latest local & remote changes.
    ///
    /// It is recommended to sync by tracking changes only after
    /// calling [sync_full] at least once. That is, by calling
    /// [sync_full] and then periodically calling [sync_changes].
    pub async fn sync_changes(&self) -> Result<()> {
        unimplemented!();
    }

    pub async fn sync_one(&self, delta: Unreconciled) -> Result<()> {
        if delta.local.is_some() && delta.remote.is_some() {
        }

        if delta.local.is_some() && delta.remote.is_none() {
            let local_file = delta.local.unwrap();
            let remote_parent_id = match local_file.parent {
                Some(parent_id) => self.local
                    .get_remote_file(&parent_id.to_string())
                    .await?.map(|file| file.id.to_string()),
                _ => None,
            };
            match local_file.remote_id {
                Some(remote_id) => {
                    let remote_file = self.remote.get_file(&remote_id).await;
                    if local_file.is_deleted && remote_file.is_ok() {
                        self.remote.remove_file(&remote_id).await?;
                        return Ok(());
                    }
                    let remote_file: File = remote_file?.into();
                    if local_file.name != remote_file.name || remote_parent_id != remote_file.parent_id {
                        self.remote.move_file(&remote_id, remote_parent_id.as_deref(), Some(&local_file.name)).await?;
                    }
                    return Ok(());
                },
                None => {
                    let uploaded: File = self.remote.create_file(remote_parent_id.as_deref(), &local_file.name).await?.into();
                    self.local.set_remote_id(local_file.id, &uploaded.id).await?;
                    return Ok(());
                }
            }
        }

        if delta.local.is_none() && delta.remote.is_some() {
            let remote_file = delta.remote.unwrap();
            let mut remote_parent_id: Option<String> = None;
            if let Some(parent_id) = remote_file.parent_id {
                if self.remote_root.clone().is_some_and(|id| id == parent_id) {
                    remote_parent_id = None;
                } else {
                    match self.local.get_remote_file(&parent_id).await? {
                        Some(parent_file) => {
                            remote_parent_id = Some(parent_file.id.to_string());
                        },
                        None => return Ok(()),
                    }
                }
            }

            if self.remote_root.is_some() && remote_parent_id.is_none() {
                return Ok(());
            }

            let local_file = self.local.get_remote_file(&remote_file.id).await?;
            match local_file {
                Some(local_file) => {
                    let local_parent_id = match local_file.parent {
                        Some(parent_id) => self.local.get_file(&parent_id.to_string()).await?.remote_id,
                        None => None,
                    };
                    if remote_file.is_deleted && !local_file.is_deleted {
                        self.local.remove_file(&local_file.id.to_string()).await?;
                        return Ok(());
                    }
                    if local_file.name != remote_file.name || local_parent_id != remote_parent_id {
                        self.local.move_file(&local_file.id.to_string(), remote_parent_id.as_deref(), Some(&remote_file.name)).await?;
                    }
                    return Ok(());
                },
                None => {
                    let new_file = self.local.create_file(remote_parent_id.as_deref(), &remote_file.name).await?;
                    self.local.set_remote_id(new_file.id, &remote_file.id).await?;
                    return Ok(());
                }
            }
        }

        Ok(())
    }
}

/// Defines an unreconciled change in the local or remote filesystems.
#[derive(Clone, Debug)]
pub struct Unreconciled {
    pub id: String,
    pub local: Option<LocalFile>,
    pub remote: Option<File>,
}

#[cfg(test)]
mod tests {
    use crate::local::Client as LClient;
    use crate::cloud::onedrive::Client as RClient;

    use crate::oauth2;
    use crate::core::FileSystem as _;
    use std::sync::Arc;
    use tokio::sync::OnceCell;
    use dotenv::dotenv;
    use std::env;

    static CLIENT: OnceCell<Arc<LClient>> = OnceCell::const_new();
    static TOKEN: OnceCell<Arc<oauth2::Token>> = OnceCell::const_new();

    async fn get_remote_client() -> RClient {
        dotenv().ok();
        let client_id = env::var("ONEDRIVE_CLIENT_ID")
            .expect("missing ONEDRIVE_CLIENT_ID env variable");

        let redirect_uri = env::var("ONEDRIVE_REDIRECT_URI")
            .expect("missing ONEDRIVE_REDIRECT_URI env variable");

        let refresh_token = env::var("ONEDRIVE_REFRESH_TOKEN")
            .expect("missing ONEDRIVE_REFRESH_TOKEN env variable");

        let app_config = oauth2::Config::onedrive(&client_id, &redirect_uri);
        let token = TOKEN.get_or_init(|| async {
            let token = oauth2::Token::from_refresh_token(&refresh_token, &app_config).await.unwrap();
            Arc::new(token)
        }).await.clone();

        RClient::new(&token, &app_config)
    }

    async fn get_local_client() -> Arc<LClient> {
        CLIENT.get_or_init(|| async {
            let db_name = "./hs-localfs-test-db.sqlite";

            // Add files for testing.
            const TESTING_SCHEMA: &str = r#"
INSERT INTO File VALUES
  (0, "test.txt", NULL, NULL, FALSE, 0, 0, NULL, FALSE, FALSE),
  (1, "test-dltd", NULL, NULL, TRUE, 0, 0, NULL, TRUE, FALSE),
  (2, "my_folder", NULL, NULL, FALSE, 0, 0, NULL, TRUE, FALSE),
  (3, "move_me.txt", NULL, NULL, FALSE, 0, 0, NULL, FALSE, FALSE),
  (4, "delete_me.txt", NULL, NULL, FALSE, 0, 0, NULL, FALSE, FALSE),
  (5, "delete_me", NULL, NULL, FALSE, 0, 0, NULL, TRUE, FALSE),
  (6, "deleted_child.txt", 5, NULL, FALSE, 0, 0, NULL, FALSE, FALSE),
  (7, "test-list", NULL, NULL, FALSE, 0, 0, NULL, TRUE, FALSE),
  (8, "list-child.txt", 7, NULL, FALSE, 0, 0, NULL, FALSE, FALSE),
  (9, "test-list-deleted", NULL, NULL, FALSE, 0, 0, NULL, TRUE, FALSE),
  (10, "list-child.txt", 9, NULL, FALSE, 0, 0, NULL, FALSE, FALSE),
  (11, "bookmarked.txt", NULL, NULL, FALSE, 0, 0, NULL, FALSE, TRUE);
"#;

            let db = database::Database::new(&database::Config {
                max_connections: 1,
                local_path: db_name.to_string(),
                migrations: vec![
                    database::Migration {
                        version: 0,
                        sql: crate::local::SCHEMA_VERSION_0.to_string(),
                        kind: database::MigrationType::Up,
                    },
                    database::Migration {
                        version: 1,
                        sql: TESTING_SCHEMA.to_string(),
                        kind: database::MigrationType::Up,
                    }
                ]
            }).await.unwrap();
            Arc::new(LClient::new(Arc::new(db)))
        }).await.clone()
    }

    #[tokio::test]
    async fn test_list_children() {
        let rclient = get_remote_client().await;
        rclient.list_files(None).await.unwrap().iter().for_each(|file| println!("{file:?}"));
    }
}
