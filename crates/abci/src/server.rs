use crate::api::AbciApi;
use futures::FutureExt;
use std::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};
use tendermint_abci::ServerBuilder;
use tokio::task::JoinHandle;

pub struct AbciServer<T>
where
    T: AbciApi,
{
    backend: PhantomData<T>,
}

impl<T> AbciServer<T>
where
    T: AbciApi,
{
    pub fn init(
        buffer_size: usize,
        address: impl AsRef<str>,
        backend: T,
    ) -> Result<AbciServerHandle, AbciServerError> {
        let server = ServerBuilder::new(buffer_size)
            .bind(address.as_ref(), backend)
            .map_err(AbciServerError::Build)?;

        let handle = tokio::spawn(async move { server.listen().map_err(AbciServerError::Listen) });
        Ok(AbciServerHandle(handle))
    }
}

pub struct AbciServerHandle(JoinHandle<Result<(), AbciServerError>>);

impl Future for AbciServerHandle {
    type Output = Result<(), AbciServerError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        match this.0.poll_unpin(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(value) => match value {
                Ok(value) => Poll::Ready(value),
                Err(error) => Poll::Ready(Err(AbciServerError::Join(error))),
            },
        }
    }
}

#[derive(Debug)]
pub enum AbciServerError {
    Build(tendermint_abci::Error),
    Listen(tendermint_abci::Error),
    Join(tokio::task::JoinError),
}
