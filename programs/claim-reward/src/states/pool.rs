use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct RewardPool {
    pub task:u16,
    /// total reward.
    pub total: u64,
    /// Total amount claimed.
    pub claimed: u64,
    /// Number of claims.
    pub tx_num: u16,
}

impl RewardPool {
    pub const SEED_PREFIX: &'static [u8; 4] = b"pool";
    pub const MAX_SIZE: usize = 2 + 8 + 8 +2;
}