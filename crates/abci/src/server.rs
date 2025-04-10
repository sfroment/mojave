use super::api::AbciApi;
use futures::FutureExt;
use std::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    process::{Child, Command, Output},
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
    fn start_cometbft_node(
        home_directory: impl AsRef<str>,
        proxy_app_address: impl AsRef<str>,
    ) -> Result<Child, AbciServerError> {
        let mut cometbft_node = Command::new("cometbft");
        cometbft_node.args([
            "start",
            "--home",
            home_directory.as_ref(),
            "--proxy-app",
            proxy_app_address.as_ref(),
        ]);
        let handle = cometbft_node.spawn().map_err(AbciServerError::CometBft)?;
        Ok(handle)
    }

    pub fn init(
        home_directory: impl AsRef<str>,
        address: impl AsRef<str>,
        buffer_size: usize,
        backend: T,
    ) -> Result<AbciServerHandle, AbciServerError> {
        let server = ServerBuilder::new(buffer_size)
            .bind(address.as_ref(), backend)
            .map_err(AbciServerError::Build)?;
        let server_handle =
            tokio::spawn(async move { server.listen().map_err(AbciServerError::Server) });

        let cometbft_node = Self::start_cometbft_node(home_directory, address)?;
        let cometbft_node_handle = tokio::spawn(async move {
            cometbft_node
                .wait_with_output()
                .map_err(AbciServerError::CometBft)
        });
        Ok(AbciServerHandle {
            server: server_handle,
            cometbft_node: cometbft_node_handle,
        })
    }
}

pub struct AbciServerHandle {
    server: JoinHandle<Result<(), AbciServerError>>,
    cometbft_node: JoinHandle<Result<Output, AbciServerError>>,
}

impl Future for AbciServerHandle {
    type Output = AbciServerStatus;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        match this.server.poll_unpin(cx) {
            Poll::Pending => {}
            Poll::Ready(value) => {
                this.server.abort();
                match value {
                    Ok(Ok(())) => return Poll::Ready(AbciServerStatus(None)),
                    Ok(Err(error)) => return Poll::Ready(AbciServerStatus(Some(error))),
                    Err(error) => return Poll::Ready(AbciServerStatus(Some(error.into()))),
                }
            }
        }

        match this.server.poll_unpin(cx) {
            Poll::Pending => {}
            Poll::Ready(value) => {
                this.cometbft_node.abort();
                match value {
                    Ok(Ok(())) => return Poll::Ready(AbciServerStatus(None)),
                    Ok(Err(error)) => return Poll::Ready(AbciServerStatus(Some(error))),
                    Err(error) => return Poll::Ready(AbciServerStatus(Some(error.into()))),
                }
            }
        }

        Poll::Pending
    }
}

#[derive(Debug)]
pub enum AbciServerError {
    Build(tendermint_abci::Error),
    Server(tendermint_abci::Error),
    CometBft(std::io::Error),
    Join(tokio::task::JoinError),
}

impl std::fmt::Display for AbciServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AbciServerError {}

impl From<tokio::task::JoinError> for AbciServerError {
    fn from(value: tokio::task::JoinError) -> Self {
        Self::Join(value)
    }
}

pub struct AbciServerStatus(Option<AbciServerError>);

impl std::fmt::Debug for AbciServerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            None => write!(f, "ABCI server stopped"),
            Some(error) => write!(f, "ABCI server stopped with an error: {}", error),
        }
    }
}
