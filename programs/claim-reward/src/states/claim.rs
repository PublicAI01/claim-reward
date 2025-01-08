use anchor_lang::{account, InitSpace};
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ClaimState {
    pub owner: Pubkey,
    pub task: u16,
    pub reward:u64,
}