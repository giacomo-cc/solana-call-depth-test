use anchor_lang::prelude::*;
use p6;

declare_id!("Eb5qgD2JomsCytRSjEQyEyhm8ZRsQ6fXMeneBpsPTkpz");

#[program]
pub mod p5 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, val: u64) -> Result<()> {
        if val > 0 {
            p6::cpi::initialize(
                CpiContext::new(
                    ctx.accounts.p6.to_account_info(),
                    p6::cpi::accounts::Initialize {
                        p6: ctx.accounts.p6.clone()
                    }
                )
            )?;
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: safe
    pub p6: AccountInfo<'info>,
}