use revm::context::{BlockEnv, CfgEnv, TxEnv};

#[derive(Default)]
pub struct Environments {
    pub cfg_env: CfgEnv,
    pub block_env: BlockEnv,
    pub tx_env: TxEnv,
}
