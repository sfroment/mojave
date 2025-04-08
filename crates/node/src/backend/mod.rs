pub mod api;
pub mod error;

use crate::{pool::TransactionPool, service::Service};
use std::sync::Arc;

pub struct Backend {
    inner: Arc<BackendInner>,
}

impl Clone for Backend {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

struct BackendInner {
    pool: TransactionPool,
    service: Service,
}
