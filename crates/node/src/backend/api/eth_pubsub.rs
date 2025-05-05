use crate::backend::Backend;
use drip_chain_rpc::api::eth_pubsub::EthPubSubApi;
use drip_chain_types::{
    network::AnyHeader,
    primitives::B256,
    rpc::{Filter, Header, Log},
};
use futures::stream::Stream;

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
