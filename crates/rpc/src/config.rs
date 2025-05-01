#[derive(Debug)]
pub struct RpcConfig {
    pub rpc_address: String,
    pub websocket_address: String,
}

impl Default for RpcConfig {
    fn default() -> Self {
        Self {
            rpc_address: "".to_owned(),
            websocket_address: "".to_owned(),
        }
    }
}
