use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct Data {
    pub value: String,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone)]
pub struct Cache {
    storage: Arc<RwLock<HashMap<String, Data>>>,
}

const DEFAULT_TTL: chrono::Duration = chrono::Duration::hours(1);

impl Default for Cache {
    fn default() -> Self {
        Self::new()
    }
}

impl Cache {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get<K, V>(&self, key: &K) -> Option<V>
    where
        K: Serialize,
        V: for<'de> Deserialize<'de>,
    {
        let key = serde_json::to_string(key).ok()?;

        let storage = self.storage.read().await;
        let data = storage.get(&key)?;
        if let Some(expires_at) = data.expires_at {
            if expires_at < chrono::Utc::now() {
                return None;
            }
        }

        let data: V = serde_json::from_str(&data.value).ok()?;
        Some(data)
    }

    pub async fn set<K: Serialize, V: Serialize>(
        &self,
        key: &K,
        value: V,
        expires_in: Option<chrono::Duration>,
    ) -> Option<V> {
        let key = serde_json::to_string(key).ok()?;
        let value_json = serde_json::to_string(&value).ok()?;

        let expires_in = Some(expires_in.unwrap_or(DEFAULT_TTL));

        let mut storage = self.storage.write().await;
        storage.insert(
            key.clone(),
            Data {
                value: value_json,
                expires_at: expires_in.map(|expires_in| chrono::Utc::now() + expires_in),
            },
        );

        drop(storage);

        if let Some(expires_in) = expires_in {
            let storage = self.storage.clone();
            tokio::spawn(async move {
                tokio::time::sleep(expires_in.to_std().unwrap()).await;
                let mut storage = storage.write().await;
                storage.remove(&key);
            });
        }

        Some(value)
    }

    pub async fn cached<F, Fut, K, V, E>(
        &self,
        action_fn: F,
        key: K,
        expires_in: Option<chrono::Duration>,
    ) -> Result<V, E>
    where
        K: Serialize + std::fmt::Debug,
        V: for<'de> Deserialize<'de> + Serialize,
        Fut: Future<Output = Result<V, E>>,
        F: FnOnce() -> Fut + Send + 'static,
    {
        let data: Option<V> = self.get(&key).await;
        if let Some(data) = data {
            log::debug!("[cache] cache hit key={key:?}");
            return Ok(data);
        }

        match action_fn().await {
            Err(err) => Err(err),
            Ok(value) => {
                let value = self
                    .set(&key, value, expires_in)
                    .await
                    .expect("failed to set cache");
                log::debug!("[cache] new entry key={key:?}");

                Ok(value)
            }
        }
    }
}
