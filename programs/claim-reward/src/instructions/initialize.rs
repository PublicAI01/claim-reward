use crate::errors::error::ErrorCode;
use crate::states::config::*;
use crate::states::pool::*;
use anchor_lang::prelude::*;
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
    init,
    seeds = [Config::SEED_PREFIX],
    bump,
    payer = payer,
    space = 8 + Config::MAX_SIZE
    )]
    pub config: Box<Account<'info, Config>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<Initialize>, signer: Pubkey) -> Result<()> {
    let config_state = &mut ctx.accounts.config;
    if config_state.initialized {
        return Err(ErrorCode::Initialized.into());
    }
    config_state.owner = *ctx.accounts.payer.key;
    config_state.signer = signer;
    config_state.initialized = true;
    Ok(())
}

#[derive(Accounts)]
#[instruction(task: u16)]
pub struct RegisterRewardPool<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
    has_one = owner @ ErrorCode::OwnerOnly,
    seeds = [Config::SEED_PREFIX],
    bump,
    )]
    pub config: Box<Account<'info, Config>>,

    #[account(
    init,
    seeds = [
        RewardPool::SEED_PREFIX,
        &task.to_le_bytes()[..]
    ],
    bump,
    payer = owner,
    space = 8 + RewardPool::INIT_SPACE
    )]
    pub pool: Box<Account<'info, RewardPool>>,
    pub system_program: Program<'info, System>,
}

pub fn register_reward_pool(ctx: Context<RegisterRewardPool>, task: u16, total: u64) -> Result<()> {
    let config_state = &mut ctx.accounts.config;
    // To initialize first.
    if !config_state.initialized {
        return Err(ErrorCode::NotInitialized.into());
    }

    let pool = &mut ctx.accounts.pool;
    pool.task = task;
    pool.total = total;
    pool.claimed = 0;
    pool.tx_num = 0;

    Ok(())
}
