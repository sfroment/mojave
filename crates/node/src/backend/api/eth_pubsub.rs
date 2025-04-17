use crate::backend::Backend;
use futures::stream::Stream;
use mandu_rpc::{api::eth_pubsub::EthPubSubApi, types::*};

impl EthPubSubApi for Backend {
    fn subscribe_new_heads(&self) -> impl Stream<Item = Header> + Unpin {
        self.pubsub_service().subscribe_new_heads()
    }

    fn subscribe_logs(&self, filter: Option<Box<Filter>>) -> impl Stream<Item = Log> + Unpin {
        self.pubsub_service().subscribe_logs(filter)
    }

    fn subscribe_new_pending_transaction(&self) -> impl Stream<Item = TransactionHash> + Unpin {
        self.pubsub_service().subscribe_new_pending_transaction()
    }
}
