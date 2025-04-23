use revm::{
    context::{BlockEnv, CfgEnv, Context, ContextTr, Evm, TxEnv},
    database::{CacheDB, EmptyDB, EmptyDBTyped},
    handler::{instructions::EthInstructions, EthPrecompiles},
    interpreter::{interpreter::EthInterpreter, Host},
    primitives::{Address, TxKind, U256},
    state::AccountInfo,
    Database, DatabaseCommit, DatabaseRef, ExecuteCommitEvm, ExecuteEvm, MainBuilder, MainContext,
};
use std::convert::Infallible;

pub type EVM = Evm<
    Context<BlockEnv, TxEnv, CfgEnv, CacheDB<EmptyDBTyped<Infallible>>>,
    (),
    EthInstructions<
        EthInterpreter,
        Context<BlockEnv, TxEnv, CfgEnv, CacheDB<EmptyDBTyped<Infallible>>>,
    >,
    EthPrecompiles,
>;

pub struct VM(EVM);

impl Default for VM {
    fn default() -> Self {
        let database = CacheDB::new(EmptyDB::new());
        let evm = Context::mainnet().with_db(database).build_mainnet();
        Self(evm)
    }
}
