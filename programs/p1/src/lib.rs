use anchor_lang::prelude::*;
use p2;

declare_id!("7g7gd8n4gwnMg18KfJmB26ub7PXmWYQ8pUvtBzNgawV7");

#[program]
pub mod p1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, val: u64) -> Result<()> {
        if val > 0 {
            p2::cpi::initialize(
                CpiContext::new(
                    ctx.accounts.p2.to_account_info(),
                    p2::cpi::accounts::Initialize {
                        p3: ctx.accounts.p3.clone(),
                        p4: ctx.accounts.p4.clone(),
                        p5: ctx.accounts.p5.clone(),
                        p6: ctx.accounts.p6.clone(),
                    }
                ), val - 1
            )?;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: safe
    pub p1: AccountInfo<'info>,
    /// CHECK: safe
    pub p2: AccountInfo<'info>,
    /// CHECK: safe
    pub p3: AccountInfo<'info>,
    /// CHECK: safe
    pub p4: AccountInfo<'info>,
    /// CHECK: safe
    pub p5: AccountInfo<'info>,
    /// CHECK: safe
    pub p6: AccountInfo<'info>,
}
