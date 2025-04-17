#[derive(Clone, PartialEq, Eq)]
pub enum BackendError {
    EmptyRawTransaction,
    DecodeTransaction,
    Unimplemented,
}

impl std::fmt::Debug for BackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyRawTransaction => write!(f, "Empty raw transaction"),
            Self::DecodeTransaction => write!(f, "Failed to decode the transaction"),
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
