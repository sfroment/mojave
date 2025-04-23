use revm::{
    context::DBErrorMarker,
    database::{CacheDB, EmptyDB},
    primitives::{Address, B256, U256},
    state::{AccountInfo, Bytecode},
    DatabaseRef,
};
use std::{convert::Infallible, sync::Arc};

pub struct StateDatabase(Arc<CacheDB<EmptyDB>>);

impl Clone for StateDatabase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl From<CacheDB<EmptyDB>> for StateDatabase {
    fn from(value: CacheDB<EmptyDB>) -> Self {
        Self(Arc::new(value))
    }
}

impl DatabaseRef for StateDatabase {
    type Error = DatabaseError;

    fn basic_ref(&self, address: Address) -> Result<Option<AccountInfo>, Self::Error> {
        self.0.basic_ref(address).map_err(|error| error.into())
    }

    fn code_by_hash_ref(&self, code_hash: B256) -> Result<Bytecode, Self::Error> {
        self.0
            .code_by_hash_ref(code_hash)
            .map_err(|error| error.into())
    }

    fn storage_ref(&self, address: Address, index: U256) -> Result<U256, Self::Error> {
        self.0
            .storage_ref(address, index)
            .map_err(|error| error.into())
    }

    fn block_hash_ref(&self, number: u64) -> Result<B256, Self::Error> {
        self.0.block_hash_ref(number).map_err(|error| error.into())
    }
}

pub enum DatabaseError {}

impl std::fmt::Debug for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for DatabaseError {}

impl From<Infallible> for DatabaseError {
    fn from(value: Infallible) -> Self {
        match value {}
    }
}

impl DBErrorMarker for DatabaseError {}
