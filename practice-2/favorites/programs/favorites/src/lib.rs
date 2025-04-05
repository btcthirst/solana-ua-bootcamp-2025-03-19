use anchor_lang::prelude::*;

declare_id!("DrmQuX3uxQ1wRZGov1jmXuQBqWKPgrdXSDyjTWkMB2Ln");

#[program]
pub mod favorites {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
