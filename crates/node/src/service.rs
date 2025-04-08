use std::sync::Arc;

pub struct Service {
    inner: Arc<ServiceInner>,
}

impl Clone for Service {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl Service {}

struct ServiceInner {}
