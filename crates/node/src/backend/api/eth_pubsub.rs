// use crate::backend::{error::BackendError, Backend};
// use futures::stream::Stream;
// use mandu_rpc::{api::eth_pubsub::EthPubSubApi, types::*};

// impl EthPubSubApi for Backend {
//     type Error = BackendError;

//     fn subscribe_new_heads(&self) -> impl Stream<Item = Result<Header, Self::Error>> + Unpin {
//         Err(BackendError::Unimplemented)
//     }

//     fn subscribe_logs(
//         &self,
//         filter: Option<Box<Filter>>,
//     ) -> impl Stream<Item = Result<Log, Self::Error>> + Unpin {
//         Err(BackendError::Unimplemented)
//     }

//     fn subscribe_new_pending_transaction(
//         &self,
//     ) -> impl Stream<Item = Result<TransactionHash, Self::Error>> + Unpin {
//         Err(BackendError::Unimplemented)
//     }
// }
