use super::api::AbciApi;
use futures::FutureExt;
use std::{
    future::Future,
    marker::PhantomData,
    task::{Context, Poll},
};
use tendermint_abci::ServerBuilder;
use tokio::{process::Command, task::JoinHandle};

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
    pub fn start_cometbft_node(
        home_directory: impl AsRef<str>,
        proxy_app_address: impl AsRef<str>,
    ) -> JoinHandle<AbciServerError> {
        let mut cometbft_node = Command::new("cometbft");
        cometbft_node.args([
            "start",
            "--home",
            home_directory.as_ref(),
            "--proxy-app",
            proxy_app_address.as_ref(),
        ]);

        let handle = tokio::spawn(async move {
            match cometbft_node.kill_on_drop(true).spawn() {
                Ok(mut handle) => match handle.wait().await {
                    Ok(status) => return AbciServerError::CometBft(status.to_string()),
                    Err(error) => return AbciServerError::CometBft(error.to_string()),
                },
                Err(error) => return AbciServerError::CometBft(error.to_string()),
            }
        });
        handle
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
        let server_handle = tokio::spawn(async move {
            match server.listen() {
                Ok(()) => return AbciServerError::Server(None),
                Err(error) => return AbciServerError::Server(Some(error)),
            }
        });

        let cometbft_node_handle = Self::start_cometbft_node(home_directory, address);

        let abci_server_handle = AbciServerHandle {
            server: server_handle,
            cometbft_node: cometbft_node_handle,
        };
        Ok(abci_server_handle)
    }
}

pub struct AbciServerHandle {
    server: JoinHandle<AbciServerError>,
    cometbft_node: JoinHandle<AbciServerError>,
}

impl Future for AbciServerHandle {
    type Output = AbciServerError;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        match this.server.poll_unpin(cx) {
            Poll::Pending => {}
            Poll::Ready(value) => {
                this.cometbft_node.abort();
                match value {
                    Ok(value) => return Poll::Ready(value),
                    Err(error) => return Poll::Ready(AbciServerError::Join(error)),
                }
            }
        }

        match this.cometbft_node.poll_unpin(cx) {
            Poll::Pending => {}
            Poll::Ready(value) => {
                this.server.abort();
                match value {
                    Ok(value) => return Poll::Ready(value),
                    Err(error) => return Poll::Ready(AbciServerError::Join(error)),
                }
            }
        }

        Poll::Pending
    }
}

pub enum AbciServerError {
    Build(tendermint_abci::Error),
    Server(Option<tendermint_abci::Error>),
    CometBft(String),
    Join(tokio::task::JoinError),
}

impl std::fmt::Debug for AbciServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Build(error) => write!(f, "Failed to build ABCI server: {}", error),
            Self::Server(error) => match error {
                Some(error) => write!(f, "ABCI server stopped with an error: {}", error),
                None => write!(f, "ABCI server stopped"),
            },
            Self::CometBft(error) => write!(f, "CometBFT node stopped with an error: {}", error),
            Self::Join(error) => write!(f, "{}", error),
        }
    }
}

impl std::fmt::Display for AbciServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AbciServerError {}
