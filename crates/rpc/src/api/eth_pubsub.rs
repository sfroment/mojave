use futures::stream::Stream;
use mandu_types::{
    network::AnyHeader,
    rpc::{Filter, Header, Log, TransactionHash},
};

#[trait_variant::make(EthPubSubApi: Send)]
pub trait LocalEthPubSubApi: Clone + Send + Sync + 'static {
    fn subscribe_new_heads(&self) -> impl Stream<Item = Header<AnyHeader>> + Unpin;

    fn subscribe_logs(&self, filter: Option<Box<Filter>>) -> impl Stream<Item = Log> + Unpin;

    fn subscribe_new_pending_transaction(&self) -> impl Stream<Item = TransactionHash> + Unpin;
}
