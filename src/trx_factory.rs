use std::sync::Arc;

use eyre::Context;
use sqlx::{PgPool, PgTransaction};
use tokio::sync::Mutex;

#[derive(Clone)]
pub enum TrxContext {
    Empty,
    Sqlx(Arc<Mutex<Option<PgTransaction<'static>>>>),
}

#[derive(Debug, thiserror::Error)]
pub enum TrxFactoryError {
    #[error("internal error: {0:?}")]
    InternalError(#[from] eyre::Error),
    #[error("failed to run migrations: {0:?}")]
    MigrateFailed(#[source] eyre::Error),
    #[error("failed to begin transaction: {0:?}")]
    BeginTransactionFailed(#[source] eyre::Error),
    #[error("failed to commit transaction: {0:?}")]
    CommitTransactionFailed(#[source] eyre::Error),
    #[error("failed to rollback transaction: {0:?}")]
    RollbackTransactionFailed(#[source] eyre::Error),
}

#[async_trait::async_trait]
pub trait TrxFactory: Send + Sync + 'static {
    async fn begin<'a, D, E, F, Fut>(&self, f: F) -> Result<D, E>
    where
        D: Send + 'a,
        F: FnOnce(TrxContext) -> Fut + Send + 'a,
        Fut: Future<Output = Result<D, E>> + Send,
        E: From<TrxFactoryError> + Send;
}

#[derive(Clone)]
pub struct SqlxTrxFactory {
    pool: PgPool,
}

impl SqlxTrxFactory {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn extract_or_create_trx(
        &self,
        ctx: TrxContext,
    ) -> Result<(Arc<Mutex<Option<PgTransaction<'static>>>>, bool), TrxFactoryError> {
        match ctx {
            TrxContext::Empty => Self::create_ctx(self.pool.clone())
                .await
                .map(|(_, sqlx_tx)| (sqlx_tx, true)),
            TrxContext::Sqlx(sqlx_trx) => Ok((sqlx_trx, false)),
        }
    }

    pub async fn create_ctx(
        pool: PgPool,
    ) -> Result<(TrxContext, Arc<Mutex<Option<PgTransaction<'static>>>>), TrxFactoryError> {
        let trx = pool
            .begin()
            .await
            .context("failed to begin transaction")
            .map_err(TrxFactoryError::BeginTransactionFailed)?;

        let sqlx_trx = Arc::new(Mutex::new(Some(trx)));
        let ctx = TrxContext::Sqlx(sqlx_trx.clone());

        Ok((ctx, sqlx_trx))
    }

    pub async fn commit_ctx(
        sqlx_trx: Arc<Mutex<Option<PgTransaction<'static>>>>,
    ) -> Result<(), TrxFactoryError> {
        let Some(sqlx_tx) = sqlx_trx.lock().await.take() else {
            Err(TrxFactoryError::CommitTransactionFailed(eyre::eyre!(
                "transaction already committed"
            )))?
        };

        sqlx_tx
            .commit()
            .await
            .context("failed to commit transaction")
            .map_err(TrxFactoryError::CommitTransactionFailed)?;

        Ok(())
    }

    pub async fn rollback_ctx(
        sqlx_trx: Arc<Mutex<Option<PgTransaction<'static>>>>,
    ) -> Result<(), TrxFactoryError> {
        let Some(sqlx_tx) = sqlx_trx.lock().await.take() else {
            Err(TrxFactoryError::RollbackTransactionFailed(eyre::eyre!(
                "transaction already rolled back"
            )))?
        };

        sqlx_tx
            .rollback()
            .await
            .context("failed to rollback transaction")
            .map_err(TrxFactoryError::RollbackTransactionFailed)?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl TrxFactory for SqlxTrxFactory {
    async fn begin<'a, D, E, F, Fut>(&self, f: F) -> Result<D, E>
    where
        D: Send + 'a,
        F: FnOnce(TrxContext) -> Fut + Send + 'a,
        Fut: Future<Output = Result<D, E>> + Send,
        E: From<TrxFactoryError> + Send,
    {
        let (ctx, sqlx_tx) = Self::create_ctx(self.pool.clone()).await?;

        let result = f(ctx).await;

        match result {
            Ok(result) => {
                Self::commit_ctx(sqlx_tx).await?;
                Ok(result)
            }
            Err(e) => {
                Self::rollback_ctx(sqlx_tx).await?;
                Err(e)
            }
        }
    }
}
