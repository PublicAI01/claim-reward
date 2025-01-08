use anchor_lang::prelude::*;

#[account]
/// Global configuration information account.
pub struct Config {
    /// Off-chain signature account.
    pub signer: Pubkey,
    pub owner: Pubkey,
    /// Initialization flag. The system needs to be initialized before the following operations can be performed.
    pub initialized: bool,
}

impl Config {
    pub const SEED_PREFIX: &'static [u8; 6] = b"config";
    pub const MAX_SIZE: usize = 32 + 32 + 1;
}

