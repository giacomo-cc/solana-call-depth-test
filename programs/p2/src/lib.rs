use anchor_lang::prelude::*;
use p3;

declare_id!("3GbLiBe9A21Q23iauMcnr9mnHVf3zuhC41oCAnHqZdFh");

#[program]
pub mod p2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, val: u64) -> Result<()> {
        if val > 0 {
            p3::cpi::initialize(
                CpiContext::new(
                    ctx.accounts.p3.to_account_info(),
                    p3::cpi::accounts::Initialize {
                        p4: ctx.accounts.p4.clone(),
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
    pub p3: AccountInfo<'info>,
    /// CHECK: safe
    pub p4: AccountInfo<'info>,
    /// CHECK: safe
    pub p5: AccountInfo<'info>,
    /// CHECK: safe
    pub p6: AccountInfo<'info>,
}
