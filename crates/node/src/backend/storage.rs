use super::database::StateDatabase;
use mandu_types::{
    primitives::B256,
    rpc::{Block, BlockId, BlockNumberOrTag},
};
use revm::primitives::{map::B256HashMap, HashMap};
use std::collections::VecDeque;

pub const DEFAULT_LIMIT: usize = 256;

/// In-memory blockchain database.
pub struct Blockchain {
    block: B256HashMap<Block>,
    // Map the block number to block hash.
    block_hash: HashMap<u64, B256>,
    current_hash: B256,
    current_number: u64,
    genesis_hash: B256,
    state_map: B256HashMap<StateDatabase>,
    state_hash: VecDeque<B256>,
    state_limit: usize,
}

impl Default for Blockchain {
    fn default() -> Self {
        Self {
            block: B256HashMap::default(),
            // Map the block number to block hash.
            block_hash: HashMap::default(),
            current_hash: B256::default(),
            current_number: u64::default(),
            genesis_hash: B256::default(),
            state_map: B256HashMap::default(),
            state_hash: VecDeque::with_capacity(DEFAULT_LIMIT),
            state_limit: DEFAULT_LIMIT,
        }
    }
}

impl Blockchain {
    pub fn get_block_by_hash(&self, hash: B256) -> Option<Block> {
        self.block.get(&hash).cloned()
    }

    pub fn get_block_by_number(&self, number: u64) -> Option<Block> {
        self.get_block_hash(number)
            .and_then(|hash| self.get_block_by_hash(hash))
    }

    pub fn get_block_hash(&self, block_number: u64) -> Option<B256> {
        self.block_hash.get(&block_number).cloned()
    }

    pub fn get_current_hash(&self) -> B256 {
        self.current_hash
    }

    pub fn get_current_number(&self) -> u64 {
        self.current_number
    }

    pub fn get_genesis_hash(&self) -> B256 {
        self.genesis_hash
    }

    /// Return [None] when requesting for the pending block.
    pub fn get_block_hash_by_id(&self, block_id: &Option<BlockId>) -> Option<B256> {
        match block_id {
            Some(block_id) => match block_id {
                BlockId::Hash(hash) => Some(hash.block_hash),
                BlockId::Number(number_or_tag) => {
                    self.get_block_hash_by_number_or_tag(number_or_tag)
                }
            },
            None => Some(self.current_hash),
        }
    }

    /// Return [None] when requesting for the pending block.
    pub fn get_block_hash_by_number_or_tag(
        &self,
        number_or_tag: &BlockNumberOrTag,
    ) -> Option<B256> {
        let slots_in_an_epoch: u64 = 32;
        match number_or_tag {
            BlockNumberOrTag::Latest => Some(self.current_hash),
            BlockNumberOrTag::Finalized => {
                if self.current_number > (slots_in_an_epoch * 2_u64) {
                    let number = self.current_number - (slots_in_an_epoch * 2_u64);
                    self.get_block_hash(number)
                } else {
                    Some(self.genesis_hash)
                }
            }
            BlockNumberOrTag::Safe => {
                if self.current_number > slots_in_an_epoch {
                    let number = self.current_number - slots_in_an_epoch;
                    self.get_block_hash(number)
                } else {
                    Some(self.genesis_hash)
                }
            }
            BlockNumberOrTag::Earliest => Some(self.genesis_hash),
            BlockNumberOrTag::Pending => None,
            BlockNumberOrTag::Number(number) => self.get_block_hash(*number),
        }
    }

    pub fn add_new_block(&mut self) {}

    pub fn get_state(&self, hash: &B256) -> Option<StateDatabase> {
        self.state_map.get(hash).cloned()
    }

    pub fn get_current_state(&self) -> Option<StateDatabase> {
        self.state_hash.back().and_then(|hash| self.get_state(hash))
    }

    pub fn insert_state(&mut self, hash: B256, state: impl Into<StateDatabase>) {
        if self.state_hash.len() == self.state_limit {
            // Remove the oldest state database.
            if let Some(hash_to_be_removed) = self.state_hash.pop_front() {
                self.state_map.remove(&hash_to_be_removed);
            }

            // Insert the new state database.
            self.state_hash.push_back(hash);
            self.state_map.insert(hash, state.into());
        }
    }
}
