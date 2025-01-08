use crate::errors::error::ErrorCode;
use crate::states::config::*;
use anchor_lang::prelude::*;
#[derive(Accounts)]
pub struct Update<'info> {
    #[account(
    mut,
    seeds = [Config::SEED_PREFIX],
    bump,
    )]
    pub config: Box<Account<'info, Config>>,
    #[account(mut,
    constraint = config.owner == payer.key()
    )]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn update(ctx: Context<Update>, signer: Pubkey) -> Result<()> {
    let config_state = &mut ctx.accounts.config;
    if !config_state.initialized {
        return Err(ErrorCode::NotInitialized.into());
    }
    config_state.signer = signer;
    Ok(())
}
