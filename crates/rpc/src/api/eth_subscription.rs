use crate::types::*;
use futures::stream::Stream;

#[trait_variant::make(EthPubSubApi: Send)]
pub trait LocalEthPubSubApi: Clone + Send + Sync + 'static {
    type Error: std::error::Error + Send + 'static;

    fn subscribe_new_heads(&self) -> impl Stream<Item = Result<Header, Self::Error>> + Unpin;

    fn subscribe_logs(&self) -> impl Stream<Item = Result<Log, Self::Error>> + Unpin;

    fn subscribe_new_pending_transaction(
        &self,
    ) -> impl Stream<Item = Result<(), Self::Error>> + Unpin;

    fn subscribe_syncing(&self) -> impl Stream<Item = Result<(), Self::Error>> + Unpin;
}
