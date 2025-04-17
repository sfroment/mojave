pub mod api;
pub mod error;
pub mod evm;
pub mod transaction;

use crate::{pool::TransactionPool, service::PubSubService};
use std::sync::Arc;

pub struct Backend {
    inner: Arc<BackendInner>,
}

#[derive(Default)]
struct BackendInner {
    transaction_pool: TransactionPool,
    pubsub_service: PubSubService,
}

impl Clone for Backend {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl Default for Backend {
    fn default() -> Self {
        Self {
            inner: Arc::new(BackendInner::default()),
        }
    }
}

impl Backend {
    pub fn transaction_pool(&self) -> &TransactionPool {
        &self.inner.transaction_pool
    }

    pub fn pubsub_service(&self) -> &PubSubService {
        &self.inner.pubsub_service
    }
}
