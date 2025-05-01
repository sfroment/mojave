use crate::backend::Backend;
use futures::stream::Stream;
use mandu_rpc::api::eth_pubsub::EthPubSubApi;
use mandu_types::{
    network::AnyHeader,
    primitives::B256,
    rpc::{Filter, Header, Log},
};

impl EthPubSubApi for Backend {
    fn subscribe_new_heads(&self) -> impl Stream<Item = Header<AnyHeader>> + Unpin {
        self.pubsub_service().subscribe_new_heads()
    }

    fn subscribe_logs(&self, filter: Option<Box<Filter>>) -> impl Stream<Item = Log> + Unpin {
        self.pubsub_service().subscribe_logs(filter)
    }

    fn subscribe_new_pending_transaction(&self) -> impl Stream<Item = B256> + Unpin {
        self.pubsub_service().subscribe_new_pending_transaction()
    }
}
