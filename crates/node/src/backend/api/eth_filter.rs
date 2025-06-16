use crate::backend::{error::BackendError, Backend};
use mohave_chain_rpc::api::eth_filter::EthFilterApi;
use mohave_chain_types::rpc::*;

impl EthFilterApi for Backend {
    type Error = BackendError;

    /// Returns all filter changes since last poll.
    async fn get_filter_changes(&self, id: String) -> Result<FilterChanges, Self::Error> {
        match self.evm_client().get_filter_changes(&id).await {
            ResponseResult::Success(value) => {
                let response =
                    serde_json::from_value(value).map_err(|_| BackendError::EthFilterResponse)?;
                Ok(response)
            }
            ResponseResult::Error(error) => Err(BackendError::EthFilter(error.message.to_string())),
        }
    }

    /// Returns all logs matching given filter (in a range 'from' - 'to').
    async fn get_filter_logs(&self, id: String) -> Result<Vec<Log>, Self::Error> {
        self.evm_client()
            .get_filter_logs(&id)
            .await
            .map_err(BackendError::EthApi)
    }

    /// Returns logs matching given filter object.
    async fn get_logs(&self, filter: Filter) -> Result<Vec<Log>, Self::Error> {
        self.evm_client()
            .logs(filter)
            .await
            .map_err(BackendError::EthApi)
    }

    /// Creates a new block filter and returns its id.
    async fn new_block_filter(&self) -> Result<String, Self::Error> {
        self.evm_client()
            .new_block_filter()
            .await
            .map_err(BackendError::EthApi)
    }

    /// Creates a new filter and returns its id.
    async fn new_filter(&self, filter: Filter) -> Result<String, Self::Error> {
        self.evm_client()
            .new_filter(filter)
            .await
            .map_err(BackendError::EthApi)
    }

    /// Creates a pending transaction filter and returns its id.
    async fn new_pending_transaction_filter(&self) -> Result<String, Self::Error> {
        self.evm_client()
            .new_pending_transaction_filter()
            .await
            .map_err(BackendError::EthApi)
    }

    /// Uninstalls the filter.
    async fn uninstall_filter(&self, id: String) -> Result<bool, Self::Error> {
        self.evm_client()
            .uninstall_filter(&id)
            .await
            .map_err(BackendError::EthApi)
    }
}
