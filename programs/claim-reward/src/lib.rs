use anchor_lang::prelude::*;

declare_id!("CcKyea6zthoWMKLkeMa6V8VEhPWDpHhJievhS5uvBSsj");

#[program]
pub mod claim_reward {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
