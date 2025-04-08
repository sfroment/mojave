pub mod http;
pub mod websocket;

use crate::{
    api::{eth::EthApi, eth_filter::EthFilterApi, eth_pubsub::EthPubSubApi},
    config::RpcConfig,
    error::RpcError,
};
use http::HttpServer;
use jsonrpsee::server::ServerHandle;
use std::marker::PhantomData;
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
