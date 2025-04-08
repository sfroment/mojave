#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BackendError {
    Unimplemented,
}

impl std::fmt::Display for BackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for BackendError {}
