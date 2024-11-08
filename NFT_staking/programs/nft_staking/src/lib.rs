pub mod errors;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use instructions::*;
pub use state::*;

declare_id!("FTbdnyYKQyEqYa3HrnZArGQHzDCmHzquGNvu2iATjryQ");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn init_stake_config(
        ctx: Context<InitializeConfig>,
        points_per_stake: u8,
        max_stake: u8,
        freeze_period: u32,
    ) -> Result<()> {
        ctx.accounts
            .init_config(points_per_stake, max_stake, freeze_period, &ctx.bumps)
    }

    pub fn init_user_account(ctx: Context<InitializeUser>) -> Result<()> {
        ctx.accounts.init_user(&ctx.bumps)
    }

    pub fn init_stake_account(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake()
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.claim_rewards()
    }
}
