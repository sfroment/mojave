use futures::{Stream, StreamExt};
use mandu_types::rpc::{Filter, Header, Log, TransactionHash};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio::sync::broadcast;
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
