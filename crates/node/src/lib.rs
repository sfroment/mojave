pub mod backend;
pub mod pool;
pub mod service;

use backend::{error::BackendError, Backend};
use futures::FutureExt;
use mandu_abci::{
    client::{AbciClient, AbciClientError},
    server::{AbciServer, AbciServerError, AbciServerHandle},
};
use mandu_rpc::{
    config::RpcConfig,
    error::RpcServerError,
    server::{RpcServer, RpcServerHandle},
};
use std::{
    env,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

pub struct ManduNode;

impl ManduNode {
    pub async fn init() -> Result<ManduNodeHandle, ManduNodeError> {
        // TODO: replace it with clap parser for advance CLI.
        let arguments: Vec<String> = env::args().skip(1).collect();
        let home_directory = arguments.first().expect("Provide the home directory");

        // Initialize anvil backend.
        let node_config = anvil::NodeConfig::empty_state();
        let (evm_client, evm_client_handle) = anvil::try_spawn(node_config).await.unwrap();

        // Initialize ABCI configuration and client.
        let abci_config = AbciServer::<Backend>::init_config(home_directory)?;
        let abci_client = AbciClient::new(abci_config.rpc.laddr.to_string())?;

        // Initialize the backend.
        let backend = Backend::init(evm_client, abci_client);

        // Initialize ABCI server.
        let abci_server_handle = AbciServer::init(home_directory, abci_config, backend.clone())?;

        // Initialize RPC server.
        let rpc_config = RpcConfig::default();
        let rpc_server_handle = RpcServer::init(&rpc_config, backend).await?;

        let handle = ManduNodeHandle {
            abci_server: abci_server_handle,
            rpc_server: rpc_server_handle,
            evm_client_handle: evm_client_handle,
        };
        Ok(handle)
    }
}

pub struct ManduNodeHandle {
    abci_server: AbciServerHandle,
    rpc_server: RpcServerHandle,
    #[allow(unused)]
    evm_client_handle: anvil::NodeHandle,
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
    AbciServer(AbciServerError),
    AbciClient(AbciClientError),
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
        Self::AbciServer(value)
    }
}

impl From<AbciClientError> for ManduNodeError {
    fn from(value: AbciClientError) -> Self {
        Self::AbciClient(value)
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
