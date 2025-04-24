use std::convert::Infallible;

use revm::context::DBErrorMarker;

#[derive(Clone, PartialEq, Eq)]
pub enum BackendError {
    EmptyRawTransaction,
    DecodeTransaction,
    InvalidBlockHash(mandu_types::primitives::B256),
    InvalidBlockId(Option<mandu_types::rpc::BlockId>),
    InvalidBlockNumberOrTag(mandu_types::rpc::BlockNumberOrTag),
    AccountDoesNotExist(mandu_types::primitives::Address),
    CodeDoesNotExist,
    Unimplemented,
}

impl std::fmt::Debug for BackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyRawTransaction => write!(f, "Empty raw transaction"),
            Self::DecodeTransaction => write!(f, "Failed to decode the transaction"),
            Self::InvalidBlockHash(hash) => write!(f, "Invalid block hash: {}", hash),
            Self::InvalidBlockId(id) => write!(f, "Invalid block ID: {:?}", id),
            Self::InvalidBlockNumberOrTag(value) => write!(f, "Invalid block number: {}", value),
            Self::AccountDoesNotExist(account) => write!(f, "Account: {} does not exist", account),
            Self::CodeDoesNotExist => write!(f, "Code does not exist"),
            Self::Unimplemented => write!(f, "Unimplemented"),
        }
    }
}

impl std::fmt::Display for BackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for BackendError {}

impl DBErrorMarker for BackendError {}

impl From<Infallible> for BackendError {
    fn from(value: Infallible) -> Self {
        match value {}
    }
}
