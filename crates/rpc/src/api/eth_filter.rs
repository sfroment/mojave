use mohave_chain_types::rpc::{Filter, FilterChanges, Log};

#[trait_variant::make(EthFilterApi: Send)]
pub trait LocalEthFilterApi: Clone + Send + Sync + 'static {
    type Error: std::error::Error + Send + 'static;

    /// Returns all filter changes since last poll.
    async fn get_filter_changes(&self, id: String) -> Result<FilterChanges, Self::Error>;

    /// Returns all logs matching given filter (in a range 'from' - 'to').
    async fn get_filter_logs(&self, id: String) -> Result<Vec<Log>, Self::Error>;

    /// Returns logs matching given filter object.
    async fn get_logs(&self, filter: Filter) -> Result<Vec<Log>, Self::Error>;

    /// Creates a new block filter and returns its id.
    async fn new_block_filter(&self) -> Result<String, Self::Error>;

    /// Creates a new filter and returns its id.
    async fn new_filter(&self, filter: Filter) -> Result<String, Self::Error>;

    /// Creates a pending transaction filter and returns its id.
    async fn new_pending_transaction_filter(&self) -> Result<String, Self::Error>;

    /// Uninstalls the filter.
    async fn uninstall_filter(&self, id: String) -> Result<bool, Self::Error>;
}
