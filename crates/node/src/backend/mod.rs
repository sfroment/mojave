use crate::service::PubSubService;
use std::sync::Arc;
pub mod api;
pub mod error;

pub struct Backend {
    inner: Arc<BackendInner>,
}

struct BackendInner {
    evm_client: anvil::eth::EthApi,
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
    pub fn init(evm_client: anvil::eth::EthApi) -> Self {
        Self {
            inner: Arc::new(BackendInner {
                evm_client,
                pubsub_service: PubSubService::default(),
            }),
        }
    }

    pub fn evm_client(&self) -> &anvil::eth::EthApi {
        &self.inner.evm_client
    }

    pub fn pubsub_service(&self) -> &PubSubService {
        &self.inner.pubsub_service
    }
}
