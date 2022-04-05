use anchor_lang::prelude::*;
use p5;

declare_id!("ChLTypEikpHPuFv1nPmgdPDifcp7JiuUxghUaz4t9oVx");

#[program]
pub mod p4 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, val: u64) -> Result<()> {
        if val > 0 {
            p5::cpi::initialize(
                CpiContext::new(
                    ctx.accounts.p5.to_account_info(),
                    p5::cpi::accounts::Initialize {
                        p6: ctx.accounts.p6.clone(),
                    }
                ),
                val - 1
            )?;
        }
            
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: safe
    pub p5: AccountInfo<'info>,
    /// CHECK: safe
    pub p6: AccountInfo<'info>,
}