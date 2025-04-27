use crate::backend::Backend;
use futures::{channel::oneshot, Stream, StreamExt};
use mandu_abci::types::{RequestCheckTx, RequestFinalizeBlock, ResponseCheckTx, ResponseCommit};
use mandu_types::rpc::{Filter, Header, Log, TransactionHash};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio::sync::{broadcast, mpsc};
use tokio_stream::wrappers::BroadcastStream;

pub const QUEUE_LIMIT: usize = 100;

pub struct PubSubService {
    new_heads: broadcast::Sender<Header>,
    logs: broadcast::Sender<Log>,
    pending_transaction: broadcast::Sender<TransactionHash>,
}

impl Default for PubSubService {
    fn default() -> Self {
        Self {
            new_heads: broadcast::channel(QUEUE_LIMIT).0,
            logs: broadcast::channel(QUEUE_LIMIT).0,
            pending_transaction: broadcast::channel(QUEUE_LIMIT).0,
        }
    }
}

impl PubSubService {
    pub fn subscribe_new_heads(&self) -> NewHeadsStream {
        self.new_heads.subscribe().into()
    }

    pub fn subscribe_logs(&self, filter: Option<Box<Filter>>) -> LogsStream {
        self.logs.subscribe().into()
    }

    pub fn subscribe_new_pending_transaction(&self) -> PendingTransactionStream {
        self.pending_transaction.subscribe().into()
    }

    pub fn publish_new_head(&self, new_head: Header) {
        let _ = self.new_heads.send(new_head);
    }

    pub fn publish_pending_transaction(&self, transaction_hash: TransactionHash) {
        let _ = self.pending_transaction.send(transaction_hash);
    }
}

pub struct NewHeadsStream(BroadcastStream<Header>);

impl From<broadcast::Receiver<Header>> for NewHeadsStream {
    fn from(value: broadcast::Receiver<Header>) -> Self {
        Self(value.into())
    }
}

impl Stream for NewHeadsStream {
    type Item = Header;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        match this.0.poll_next_unpin(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(value) => match value {
                Some(item) => match item {
                    Ok(item) => Poll::Ready(Some(item)),
                    Err(_error) => Poll::Ready(None),
                },
                None => Poll::Ready(None),
            },
        }
    }
}

pub struct LogsStream(BroadcastStream<Log>);

impl From<broadcast::Receiver<Log>> for LogsStream {
    fn from(value: broadcast::Receiver<Log>) -> Self {
        Self(value.into())
    }
}

impl Stream for LogsStream {
    type Item = Log;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        match this.0.poll_next_unpin(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(value) => match value {
                Some(item) => match item {
                    Ok(item) => Poll::Ready(Some(item)),
                    Err(_error) => Poll::Ready(None),
                },
                None => Poll::Ready(None),
            },
        }
    }
}

pub struct PendingTransactionStream(BroadcastStream<TransactionHash>);

impl From<broadcast::Receiver<TransactionHash>> for PendingTransactionStream {
    fn from(value: broadcast::Receiver<TransactionHash>) -> Self {
        Self(value.into())
    }
}

impl Stream for PendingTransactionStream {
    type Item = TransactionHash;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        match this.0.poll_next_unpin(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(value) => match value {
                Some(item) => match item {
                    Ok(item) => Poll::Ready(Some(item)),
                    Err(_error) => Poll::Ready(None),
                },
                None => Poll::Ready(None),
            },
        }
    }
}

pub struct AbciService {
    sender: mpsc::UnboundedSender<(Backend, AbciRequest, oneshot::Sender<AbciResponse>)>,
}

impl AbciService {
    pub fn init() -> Self {
        let (sender, mut receiver) =
            mpsc::unbounded_channel::<(Backend, AbciRequest, oneshot::Sender<AbciResponse>)>();
        tokio::spawn(async move {
            loop {
                if let Some((backend, request, sender)) = receiver.recv().await {
                    match request {
                        AbciRequest::CheckTx(request) => {
                            let response = backend.check_transaction(request).await;
                            sender.send(response.into());
                        }
                        AbciRequest::Commit => {
                            let response = backend.do_commit().await;
                            sender.send(response.into());
                        }
                    }
                }
            }
        });

        Self { sender }
    }

    pub fn send(
        &self,
        backend: Backend,
        request: impl Into<AbciRequest>,
    ) -> oneshot::Receiver<AbciResponse> {
        let (sender, receiver) = oneshot::channel::<AbciResponse>();
        let _ = self.sender.send((backend, request.into(), sender));
        receiver
    }
}

pub enum AbciRequest {
    CheckTx(RequestCheckTx),
    Commit,
}

impl From<RequestCheckTx> for AbciRequest {
    fn from(value: RequestCheckTx) -> Self {
        Self::CheckTx(value)
    }
}

pub enum AbciResponse {
    CheckTx(ResponseCheckTx),
    Commit(ResponseCommit),
}

impl From<ResponseCheckTx> for AbciResponse {
    fn from(value: ResponseCheckTx) -> Self {
        Self::CheckTx(value)
    }
}

impl From<ResponseCommit> for AbciResponse {
    fn from(value: ResponseCommit) -> Self {
        Self::Commit(value)
    }
}
