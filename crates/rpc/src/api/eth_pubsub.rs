use crate::types::*;
use futures::stream::Stream;

#[trait_variant::make(EthPubSubApi: Send)]
pub trait LocalEthPubSubApi: Clone + Send + Sync + 'static {
    fn subscribe_new_heads(&self) -> impl Stream<Item = Header> + Unpin;

    fn subscribe_logs(&self, filter: Option<Box<Filter>>) -> impl Stream<Item = Log> + Unpin;

    fn subscribe_new_pending_transaction(&self) -> impl Stream<Item = TransactionHash> + Unpin;
}
