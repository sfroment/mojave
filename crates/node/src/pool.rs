use mandu_types::rpc::Transaction;

#[derive(Default)]
pub struct TransactionPool {
    pending: Vec<Transaction>,
}

impl TransactionPool {
    // pub fn add_pending_transaction(&mut self) {}

    pub fn get_transaction_count(&self) -> usize {
        self.pending.len()
    }
}
