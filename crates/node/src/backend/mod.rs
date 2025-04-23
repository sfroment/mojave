pub mod api;
pub mod block;
pub mod database;
pub mod error;
pub mod evm;
pub mod storage;
pub mod transaction;

use crate::{pool::TransactionPool, service::PubSubService};
use std::sync::Arc;
use storage::{BlockStorage, StateStorage};

pub struct Backend {
    inner: Arc<BackendInner>,
}

#[derive(Default)]
struct BackendInner {
    blocks: BlockStorage,
    states: StateStorage,
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
    pub fn blocks(&self) -> &BlockStorage {
        &self.inner.blocks
    }

    pub fn states(&self) -> &StateStorage {
        &self.inner.states
    }

    pub fn transaction_pool(&self) -> &TransactionPool {
        &self.inner.transaction_pool
    }

    pub fn pubsub_service(&self) -> &PubSubService {
        &self.inner.pubsub_service
    }
}
