use anchor_lang::prelude::*;

declare_id!("CBHHTYp4NVCnkscwdRwi8sv2ieEiaFYdZC6GJaDqegqj");

#[program]
pub mod finance_wellness_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
