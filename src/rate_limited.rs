use std::sync::Arc;

use tokio::sync::Semaphore;
use tokio::time::{Duration, sleep};

/// Example
/// ```rust
/// struct InternalClient;
///
/// impl InternalClient {
///     async fn some_fn(&self) {
///         println!("Executing request...");
///     }
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let client = InternalClient;
///     let rate_limited_client = RateLimitedClient::new(client, 3);
///
///     rate_limited_client
///         .call(|client| async move { client.some_fn().await })
///         .await;
///
///     // Calling `execute` again requires a new slot
///     rate_limited_client
///         .call(|client| async move { client.some_fn().await })
///         .await;
/// }
/// ```
pub struct RateLimitedClient<T> {
    client: Arc<T>,
    semaphore: Arc<Semaphore>,
    disable_limit: bool,
}

impl<T> Clone for RateLimitedClient<T> {
    fn clone(&self) -> Self {
        Self {
            client: Arc::clone(&self.client),
            semaphore: Arc::clone(&self.semaphore),
            disable_limit: self.disable_limit,
        }
    }
}

impl<T> RateLimitedClient<T> {
    pub fn new(client: T, max_concurrent: usize) -> Self {
        Self {
            client: Arc::new(client),
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            disable_limit: false,
        }
    }

    pub fn disable_limit(&mut self) {}

    pub async fn call<F, Fut, R>(&self, func: F) -> R
    where
        F: FnOnce(Arc<T>) -> Fut,
        Fut: std::future::Future<Output = R>,
    {
        if !self.disable_limit {
            let permit = self.semaphore.clone().acquire_owned().await.unwrap();

            let client = Arc::clone(&self.client);
            let result = func(client).await;

            // Delay release of semaphore
            tokio::spawn(async move {
                sleep(Duration::from_secs(1)).await;
                drop(permit);
            });

            result
        } else {
            let client = Arc::clone(&self.client);
            func(client).await
        }
    }
}
