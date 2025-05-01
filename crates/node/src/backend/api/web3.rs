use crate::backend::{error::BackendError, Backend};
use mandu_rpc::api::web3::Web3Api;
use mandu_types::primitives::Bytes;

impl Web3Api for Backend {
    type Error = BackendError;

    async fn client_version(&self) -> Result<String, Self::Error> {
        self.evm_client()
            .client_version()
            .map_err(BackendError::EthApi)
    }

    async fn sha3(&self, bytes: Bytes) -> Result<String, Self::Error> {
        self.evm_client().sha3(bytes).map_err(BackendError::EthApi)
    }
}
