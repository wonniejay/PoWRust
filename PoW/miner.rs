use create:: {
    model::{
        Address, Block, BlockHash, Transaction, TransactionPool, TransactionVec, Block_SUBSIDY,
    },
    util::{
        execution::{sleep_millis, Runnable},
        Context,
    },
};
use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MinerError {
    #[error("No valid block was mined at index `{0}`")]
    BlockNotMined(u64),
}

pub struct Miner {}

impl Runnable for Miner {}

impl Miner {
    pub fn new() -> Miner {}
    pub fn start() -> Result<()> {}
    
    fn create_target() -> BlockHash {}
    fn must_stop_mining() -> bool {}
    fn mine_block() -> Option<Block> {}
    fn create_next_block() -> Block {}
    fn create_coinbase_transaction() -> Transaction {}
}

#[cfg(test)]
mod tests {
    fn test_create_next_block() {}
    fn test_create_target_valid_difficulty() {}
    fn test_create_target_overflowing_difficulty() {}
    fn test_mine_block_found() {}
    fn test_mind_block_not_found() {}
    
    #[test]
    #[should_panic(expected="No valid block was mined at index `1`")]
    fn test_run_block_found() {}
    fn test_run_block_not_found() {}
    fn create_default_miner() {}
    fn miner_address() {}
    fn create_mine() {}
    fn create_empty_block() {}
    fn add_mock_transaction() {}
    fn assert_mined_block_is_valid() {}
}