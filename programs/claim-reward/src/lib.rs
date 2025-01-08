use anchor_lang::prelude::*;
mod errors;
mod instructions;
mod states;
mod utils;
use instructions::initialize::*;
use instructions::update::*;
declare_id!("9nhmbgEBNn6g3p5hDXBJVf6uiH3EsKLyN4WtRgLFVsc9");

#[program]
pub mod claim_reward {
    use super::*;


    pub fn initialize(ctx: Context<Initialize>, signer: Pubkey) -> Result<()> {
        instructions::initialize::initialize(ctx, signer)
    }

    pub fn register_reward_pool(ctx: Context<RegisterRewardPool>, task: u16, total: u64) -> Result<()> {
        instructions::initialize::register_reward_pool(ctx, task, total)
    }

    pub fn update(ctx: Context<Update>, signer: Pubkey) -> Result<()> {
        instructions::update::update(ctx, signer)
    }
}