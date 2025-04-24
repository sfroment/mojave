use revm::{
    context::{BlockEnv, CfgEnv, Context, Evm, TxEnv},
    database::{CacheDB, EmptyDB, EmptyDBTyped},
    handler::{instructions::EthInstructions, EthPrecompiles},
    interpreter::interpreter::EthInterpreter,
    MainBuilder, MainContext,
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
