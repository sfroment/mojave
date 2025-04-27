use std::sync::Arc;

use mandu_abci::client::AbciClient;

use crate::service::AbciService;
pub mod api;
pub mod error;

pub struct Backend {
    inner: Arc<BackendInner>,
}

struct BackendInner {
    eth_driver: anvil::eth::EthApi,
    abci_client: AbciClient,
    abci_service: AbciService,
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
                eth_driver: eth_api,
                abci_client: AbciClient::new("http://127.0.0.1:26657").unwrap(),
                abci_service: AbciService::init(),
            }),
        };
        (backend, handle)
    }

    pub fn driver(&self) -> &anvil::eth::EthApi {
        &self.inner.eth_driver
    }

    pub fn abci_client(&self) -> &AbciClient {
        &self.inner.abci_client
    }

    pub fn abci_service(&self) -> &AbciService {
        &self.inner.abci_service
    }
}
