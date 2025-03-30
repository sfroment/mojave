#[derive(Debug)]
pub enum RpcError {
    Build(std::io::Error),
    RegisterMethod(jsonrpsee::core::RegisterMethodError),
}

impl From<jsonrpsee::core::RegisterMethodError> for RpcError {
    fn from(value: jsonrpsee::core::RegisterMethodError) -> Self {
        Self::RegisterMethod(value)
    }
}

impl std::fmt::Display for RpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for RpcError {}
