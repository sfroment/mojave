use crate::service::{AbciService, PubSubService};
use drip_chain_abci::client::AbciClient;
use std::sync::Arc;
pub mod api;
pub mod error;

pub struct Backend {
    inner: Arc<BackendInner>,
}

struct BackendInner {
    evm_client: anvil::eth::EthApi,
    abci_client: AbciClient,
    abci_service: AbciService,
    pubsub_service: PubSubService,
}

impl Clone for Backend {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl Backend {
    pub fn init(evm_client: anvil::eth::EthApi, abci_client: AbciClient) -> Self {
        Self {
            inner: Arc::new(BackendInner {
                evm_client,
                abci_client,
                abci_service: AbciService::init(),
                pubsub_service: PubSubService::default(),
            }),
        }
    }

    pub fn evm_client(&self) -> &anvil::eth::EthApi {
        &self.inner.evm_client
    }

    pub fn abci_client(&self) -> &AbciClient {
        &self.inner.abci_client
    }

    pub fn abci_service(&self) -> &AbciService {
        &self.inner.abci_service
    }

    pub fn pubsub_service(&self) -> &PubSubService {
        &self.inner.pubsub_service
    }
}
