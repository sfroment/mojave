pub mod api;
pub mod block;
pub mod database;
pub mod env;
pub mod error;
pub mod evm;
pub mod storage;

use crate::{
    pool::TransactionPool,
    service::{FilterService, PubSubService},
};
use env::Environments;
use mandu_abci::client::AbciClient;
use std::sync::Arc;
use storage::Blockchain;
use tokio::sync::RwLock;

pub struct Backend {
    inner: Arc<BackendInner>,
}

#[derive(Default)]
struct BackendInner {
    environments: RwLock<Environments>,
    blockchain: RwLock<Blockchain>,
    transaction_pool: RwLock<TransactionPool>,
    pubsub_service: PubSubService,
    filter_service: FilterService,
    abci_client: AbciClient,
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
    pub fn environments(&self) -> &RwLock<Environments> {
        &self.inner.environments
    }

    pub fn blockchain(&self) -> &RwLock<Blockchain> {
        &self.inner.blockchain
    }

    pub fn transaction_pool(&self) -> &RwLock<TransactionPool> {
        &self.inner.transaction_pool
    }

    pub fn pubsub_service(&self) -> &PubSubService {
        &self.inner.pubsub_service
    }

    pub fn filter_service(&self) -> &FilterService {
        &self.inner.filter_service
    }

    pub fn abci_client(&self) -> &AbciClient {
        &self.inner.abci_client
    }
}
