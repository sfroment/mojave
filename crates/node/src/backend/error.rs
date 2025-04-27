use std::convert::Infallible;

use revm::context::DBErrorMarker;

pub enum BackendError {
    // CheckTx related errors.
    EmptyRawTransaction,
    DecodeTransaction,
    GetCurrentState,
    RecoverSigner,
    AccountDoesNotExist,
    NonceTooLow,
    InsufficientBalance,
    Undefined,
    InvalidTransactionHash,
    InvalidBlockHash(mandu_types::primitives::B256),
    InvalidBlockId(Option<mandu_types::rpc::BlockId>),
    InvalidBlockNumberOrTag(mandu_types::rpc::BlockNumberOrTag),
    CodeDoesNotExist,
    BroadcastTransaction(mandu_abci::client::AbciClientError),
    EmptyTransactionHash,
    Unimplemented,
}

impl std::fmt::Debug for BackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyRawTransaction => write!(f, "Empty raw transaction"),
            Self::DecodeTransaction => write!(f, "Failed to decode the transaction"),
            Self::GetCurrentState => write!(f, "Failed to get the current state"),
            Self::RecoverSigner => write!(f, "Failed to recover the signer"),
            Self::AccountDoesNotExist => write!(f, "Account does not exist"),
            Self::NonceTooLow => write!(f, "Nonce too low"),
            Self::InsufficientBalance => write!(f, "Insufficient balance"),
            Self::Undefined => write!(f, "Undefined error"),
            Self::InvalidTransactionHash => write!(f, "Invalid transaction hash"),
            Self::InvalidBlockHash(hash) => write!(f, "Invalid block hash: {}", hash),
            Self::InvalidBlockId(id) => write!(f, "Invalid block ID: {:?}", id),
            Self::InvalidBlockNumberOrTag(value) => write!(f, "Invalid block number: {}", value),
            Self::CodeDoesNotExist => write!(f, "Code does not exist"),
            Self::BroadcastTransaction(error) => {
                write!(f, "Failed to broadcast the transaction: {}", error)
            }
            Self::EmptyTransactionHash => write!(f, "Empty transaction hash"),
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

impl From<u32> for BackendError {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::EmptyRawTransaction,
            2 => Self::DecodeTransaction,
            3 => Self::GetCurrentState,
            4 => Self::RecoverSigner,
            5 => Self::AccountDoesNotExist,
            6 => Self::NonceTooLow,
            7 => Self::InsufficientBalance,
            _ => Self::Undefined,
        }
    }
}

impl From<BackendError> for u32 {
    fn from(value: BackendError) -> Self {
        match value {
            BackendError::EmptyRawTransaction => 1,
            BackendError::DecodeTransaction => 2,
            BackendError::GetCurrentState => 3,
            BackendError::RecoverSigner => 4,
            BackendError::AccountDoesNotExist => 5,
            BackendError::NonceTooLow => 6,
            BackendError::InsufficientBalance => 7,
            _ => u32::MAX,
        }
    }
}
