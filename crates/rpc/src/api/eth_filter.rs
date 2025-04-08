use crate::types::*;

#[trait_variant::make(EthFilterApi: Send)]
pub trait LocalEthFilterApi: Clone + Send + Sync + 'static {
    type Error: std::error::Error + Send + 'static;

    /// Returns all filter changes since last poll.
    async fn get_filter_changes(&self, id: FilterId) -> Result<FilterChanges, Self::Error>;

    /// Returns all logs matching given filter (in a range 'from' - 'to').
    async fn get_filter_logs(&self, id: FilterId) -> Result<Vec<Log>, Self::Error>;

    /// Returns logs matching given filter object.
    async fn get_logs(&self, filter: Filter) -> Result<Vec<Log>, Self::Error>;

    /// Creates a new block filter and returns its id.
    async fn new_block_filter(&self) -> Result<FilterId, Self::Error>;

    /// Creates a new filter and returns its id.
    async fn new_filter(&self, filter: Filter) -> Result<FilterId, Self::Error>;

    /// Creates a pending transaction filter and returns its id.
    async fn new_pending_transaction_filter(
        &self,
        kind: Option<PendingTransactionFilterKind>,
    ) -> Result<FilterId, Self::Error>;

    /// Uninstalls the filter.
    async fn uninstall_filter(&self, id: FilterId) -> Result<bool, Self::Error>;
}
