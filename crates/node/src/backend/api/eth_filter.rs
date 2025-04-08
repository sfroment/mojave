use crate::backend::{error::BackendError, Backend};
use mandu_rpc::{api::eth_filter::EthFilterApi, types::*};

impl EthFilterApi for Backend {
    type Error = BackendError;

    /// Returns all filter changes since last poll.
    async fn get_filter_changes(&self, id: FilterId) -> Result<FilterChanges, Self::Error> {
        Err(BackendError::Unimplemented)
    }

    /// Returns all logs matching given filter (in a range 'from' - 'to').
    async fn get_filter_logs(&self, id: FilterId) -> Result<Vec<Log>, Self::Error> {
        Err(BackendError::Unimplemented)
    }

    /// Returns logs matching given filter object.
    async fn get_logs(&self, filter: Filter) -> Result<Vec<Log>, Self::Error> {
        Err(BackendError::Unimplemented)
    }

    /// Creates a new block filter and returns its id.
    async fn new_block_filter(&self) -> Result<FilterId, Self::Error> {
        Err(BackendError::Unimplemented)
    }

    /// Creates a new filter and returns its id.
    async fn new_filter(&self, filter: Filter) -> Result<FilterId, Self::Error> {
        Err(BackendError::Unimplemented)
    }

    /// Creates a pending transaction filter and returns its id.
    async fn new_pending_transaction_filter(
        &self,
        kind: Option<PendingTransactionFilterKind>,
    ) -> Result<FilterId, Self::Error> {
        Err(BackendError::Unimplemented)
    }

    /// Uninstalls the filter.
    async fn uninstall_filter(&self, id: FilterId) -> Result<bool, Self::Error> {
        Err(BackendError::Unimplemented)
    }
}
