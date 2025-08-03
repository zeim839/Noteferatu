use crate::filesystem::{Filesystem, Delta};
use crate::local::{LocalFile, LocalFS};
use crate::errors::Result;

use std::collections::HashMap;

pub struct Sync<R: Filesystem> {
    local: LocalFS,
    remote: R,
}

impl<R: Filesystem> Sync<R> {

    pub fn new(local: LocalFS, remote: R) -> Self {
        Self { local, remote }
    }

    /// Fetches [unreconciled](Unreconciled) changes.
    pub async fn fetch_changes(&self) -> Result<Vec<Unreconciled<R::Delta>>> {
        let mut map = HashMap::<String, Unreconciled<R::Delta>>::new();
        let (local_changes, token) = self.local.track_changes(None).await?;
        for change in local_changes {
            if let Some(delta) = map.get(&Delta::id(&change)) {
                let local = delta.local.clone().unwrap();
                if local.modified_at > change.modified_at {
                    continue;
                }
            }
            map.insert(Delta::id(&change), Unreconciled {
                local: Some(change),
                remote: None,
            });
        }

        let (remote_changes, token) = self.remote.track_changes(None).await?;
        for change in remote_changes {
            if let Some(delta) = map.get_mut(&Delta::id(&change)) {
                if let Some(remote) = &delta.remote {
                    if remote.modified_at() > change.modified_at() {
                        continue;
                    }
                }
                delta.remote = Some(change);
                continue;
            }
            map.insert(Delta::id(&change), Unreconciled {
                local: None,
                remote: Some(change),
            });
        }

        let mut vec = Vec::new();
        for (_, v) in map {
            vec.push(v)
        }

        Ok(vec)
    }

    pub async fn sync_change(&self, change: Unreconciled<R::Delta>) {
        if change.local.is_some() && change.remote.is_some() {
            return self.sync_conflict(change).await;
        }
        if change.local.is_some() && change.remote.is_none() {
            return self.sync_local(change).await;
        }
        return self.sync_remote(change).await;
    }

    async fn sync_conflict(&self, change: Unreconciled<R::Delta>) {
    }

    async fn sync_local(&self, change: Unreconciled<R::Delta>) {
    }

    async fn sync_remote(&self, change: Unreconciled<R::Delta>) {
    }
}


pub struct Unreconciled<D: Delta> {
    pub local: Option<LocalFile>,
    pub remote: Option<D>,
}
