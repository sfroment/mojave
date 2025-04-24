use mandu_types::{
    consensus::TxEnvelope,
    rpc::{Transaction, TransactionHash},
};

#[derive(Default)]
pub struct TransactionPool {
    pending: Vec<Transaction>,
}

impl TransactionPool {
    pub fn add_pending_transaction(&mut self, transaction: TxEnvelope) -> TransactionHash {
        let recovered = transaction.try_into_recovered().unwrap();
        let transaction_hash = recovered.hash().clone();
        let transaction = Transaction {
            inner: recovered,
            block_hash: None,
            block_number: None,
            transaction_index: None,
            effective_gas_price: None,
        };
        self.pending.push(transaction);
        transaction_hash
    }

    pub fn get_transaction_count(&self) -> usize {
        self.pending.len()
    }
}
