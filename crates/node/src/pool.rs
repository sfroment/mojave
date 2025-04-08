use std::sync::Arc;
use tokio::sync::Mutex;

pub type Transaction = Vec<u8>;

pub struct TransactionPool {
    pending: Arc<Mutex<Vec<Transaction>>>,
}

impl Clone for TransactionPool {
    fn clone(&self) -> Self {
        Self {
            pending: self.pending.clone(),
        }
    }
}

impl Default for TransactionPool {
    fn default() -> Self {
        Self {
            pending: Arc::new(Mutex::new(Vec::default())),
        }
    }
}

impl TransactionPool {
    pub async fn add_pending_transaction(&self, transaction: Transaction) {
        let mut pending = self.pending.lock().await;
        pending.push(transaction);
    }
}
