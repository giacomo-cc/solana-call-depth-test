use anchor_lang::prelude::*;
use p4;

declare_id!("BaDqZmWdmvFRHLTQNYnnsBZrVJEY2L3nmxKqkGAfVVhD");

#[program]
pub mod p3 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, val: u64) -> Result<()> {
        if val > 0 {
            p4::cpi::initialize(
                CpiContext::new(
                    ctx.accounts.p4.to_account_info(),
                    p4::cpi::accounts::Initialize {
                        p5: ctx.accounts.p5.clone(),
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
    pub p4: AccountInfo<'info>,
    /// CHECK: safe
    pub p5: AccountInfo<'info>,
    /// CHECK: safe
    pub p6: AccountInfo<'info>,
}