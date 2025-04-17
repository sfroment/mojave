pub mod backend;
pub mod pool;
pub mod sequencer;
pub mod service;

use backend::{error::BackendError, Backend};
use futures::FutureExt;
use mandu_abci::server::{AbciServer, AbciServerError, AbciServerHandle};
use mandu_rpc::{
    config::RpcConfig,
    error::RpcServerError,
    server::{RpcServer, RpcServerHandle},
};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

const HOME_DIRECTORY: &str = "/home/kanet/Projects/rs-mandu/cometbft";
const APP_ADDRESS: &str = "127.0.0.1:26658";
const BUFFER_SIZE: usize = 1048576;
const RPC_ADDRESS: &str = "127.0.0.1:8545";
const WEBSOCKET_ADDRESS: &str = "127.0.0.1:8546";

#[derive(Default)]
pub struct ManduNode {
    backend: Backend,
}

impl ManduNode {
    pub async fn init(self) -> Result<ManduNodeHandle, ManduNodeError> {
        let abci_server_handle = AbciServer::init(
            HOME_DIRECTORY,
            APP_ADDRESS,
            BUFFER_SIZE,
            self.backend.clone(),
        )?;

        let rpc_config = RpcConfig {
            rpc_address: RPC_ADDRESS.to_owned(),
            websocket_address: WEBSOCKET_ADDRESS.to_owned(),
        };
        let rpc_server_handle = RpcServer::init(&rpc_config, self.backend).await?;

        let handle = ManduNodeHandle {
            abci_server: abci_server_handle,
            rpc_server: rpc_server_handle,
        };
        Ok(handle)
    }
}

pub struct ManduNodeHandle {
    abci_server: AbciServerHandle,
    rpc_server: RpcServerHandle,
}

impl Future for ManduNodeHandle {
    type Output = ManduNodeError;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        if let Poll::Ready(error) = this.abci_server.poll_unpin(cx) {
            return Poll::Ready(error.into());
        }

        if let Poll::Ready(error) = this.rpc_server.poll_unpin(cx) {
            return Poll::Ready(error.into());
        }

        Poll::Pending
    }
}

#[derive(Debug)]
pub enum ManduNodeError {
    Abci(AbciServerError),
    Rpc(RpcServerError),
    Backend(BackendError),
}

impl std::fmt::Display for ManduNodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ManduNodeError {}

impl From<AbciServerError> for ManduNodeError {
    fn from(value: AbciServerError) -> Self {
        Self::Abci(value)
    }
}

impl From<RpcServerError> for ManduNodeError {
    fn from(value: RpcServerError) -> Self {
        Self::Rpc(value)
    }
}

impl From<BackendError> for ManduNodeError {
    fn from(value: BackendError) -> Self {
        Self::Backend(value)
    }
}
