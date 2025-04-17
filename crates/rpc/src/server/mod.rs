pub mod http;
pub mod websocket;

use crate::{
    api::{eth::EthApi, eth_filter::EthFilterApi, eth_pubsub::EthPubSubApi},
    config::RpcConfig,
    error::RpcError,
};
use http::HttpServer;
use jsonrpsee::server::ServerHandle;
use std::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};
use websocket::WebsocketServer;

pub struct RpcServer<T>
where
    T: EthApi + EthFilterApi + EthPubSubApi,
{
    http_context: PhantomData<T>,
}

impl<T> RpcServer<T>
where
    T: EthApi + EthFilterApi + EthPubSubApi,
{
    pub async fn init(config: &RpcConfig, backend: T) -> Result<RpcServerHandle, RpcError> {
        let http_server_handle = HttpServer::init(config, backend.clone()).await?;
        let websocket_server_handle = WebsocketServer::init(config, backend).await?;
        Ok(RpcServerHandle {
            http_server_handle,
            websocket_server_handle,
        })
    }
}

pub struct RpcServerHandle {
    http_server_handle: ServerHandle,
    websocket_server_handle: ServerHandle,
}

impl Future for RpcServerHandle {
    type Output = RpcError;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        if this.http_server_handle.is_stopped() {
            let _ = this.websocket_server_handle.stop();
            return Poll::Ready(RpcError::RpcServerStopped);
        }

        if this.websocket_server_handle.is_stopped() {
            let _ = this.http_server_handle.stop();
            return Poll::Ready(RpcError::WebsocketServerStopped);
        }

        Poll::Pending
    }
}
