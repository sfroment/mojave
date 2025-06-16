#[derive(Debug)]
pub struct RpcConfig {
    pub rpc_address: String,
    pub websocket_address: String,
}

impl Default for RpcConfig {
    fn default() -> Self {
        Self {
            rpc_address: "0.0.0.0:8585".to_owned(),
            websocket_address: "0.0.0.0:8586".to_owned(),
        }
    }
}
