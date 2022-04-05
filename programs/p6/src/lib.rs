use anchor_lang::prelude::*;

declare_id!("8X6NeTnsGingsUDEfYknbttaecHRpVNQRaj3Yq6WGRoC");

#[program]
pub mod p6 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: safe
    pub p6: AccountInfo<'info>,
}