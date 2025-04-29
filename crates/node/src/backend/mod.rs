use crate::service::{AbciService, PubSubService};
use mandu_abci::client::AbciClient;
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
    pub async fn init() -> (Self, anvil::NodeHandle) {
        let node_config = anvil::NodeConfig::empty_state();
        let (eth_api, handle) = anvil::try_spawn(node_config).await.unwrap();

        let backend = Self {
            inner: Arc::new(BackendInner {
                evm_client: eth_api,
                abci_client: AbciClient::new("http://127.0.0.1:26657").unwrap(),
                abci_service: AbciService::init(),
                pubsub_service: PubSubService::default(),
            }),
        };
        (backend, handle)
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
