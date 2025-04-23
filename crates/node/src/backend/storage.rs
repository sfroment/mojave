use crate::backend::database::StateDatabase;
use revm::primitives::{map::B256HashMap, HashMap, B256};
use std::{collections::VecDeque, sync::Arc};
use tokio::sync::RwLock;

pub const DEFAULT_STATES_LIMIT: usize = 256;

/// In-memory blockchain data.
pub struct BlockStorage {
    inner: Arc<RwLock<BlockStorageInner>>,
}

#[derive(Default)]
struct BlockStorageInner {
    block: B256HashMap<B256>,
    // Map the block number to block hash.
    block_hash: HashMap<u64, B256>,
    current_hash: B256,
    current_number: u64,
    genesis_hash: B256,
}

impl Clone for BlockStorage {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl Default for BlockStorage {
    fn default() -> Self {
        Self {
            inner: Arc::new(RwLock::new(BlockStorageInner::default())),
        }
    }
}

impl BlockStorage {
    pub async fn get_block_hash(&self, block_number: u64) -> Option<B256> {
        let inner = self.inner.read().await;
        inner.block_hash.get(&block_number).cloned()
    }

    pub async fn current_hash(&self) -> B256 {
        let inner = self.inner.read().await;
        inner.current_hash
    }

    pub async fn current_number(&self) -> u64 {
        let inner = self.inner.read().await;
        inner.current_number
    }
}

/// In-memory block states.
pub struct StateStorage {
    inner: Arc<RwLock<StateStorageInner>>,
}

struct StateStorageInner {
    state_map: B256HashMap<StateDatabase>,
    state_hash: VecDeque<B256>,
    state_limit: usize,
}

impl Default for StateStorageInner {
    fn default() -> Self {
        Self {
            state_map: B256HashMap::default(),
            state_hash: VecDeque::with_capacity(DEFAULT_STATES_LIMIT),
            state_limit: DEFAULT_STATES_LIMIT,
        }
    }
}

impl Clone for StateStorage {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl Default for StateStorage {
    fn default() -> Self {
        Self {
            inner: Arc::new(RwLock::new(StateStorageInner::default())),
        }
    }
}

impl StateStorage {
    pub async fn get(&self, hash: &B256) -> Option<StateDatabase> {
        let inner = self.inner.read().await;
        inner.state_map.get(hash).cloned()
    }

    pub async fn insert(&self, hash: B256, state: impl Into<StateDatabase>) {
        let mut inner = self.inner.write().await;
        if inner.state_hash.len() == inner.state_limit {
            // Remove the oldest state database.
            if let Some(hash_to_be_removed) = inner.state_hash.pop_front() {
                inner.state_map.remove(&hash_to_be_removed);
            }

            // Insert the new state database.
            inner.state_hash.push_back(hash);
            inner.state_map.insert(hash, state.into());
        }
    }
}
