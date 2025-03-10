use anchor_lang::prelude::*;

declare_id!("FAK8bTgJLwAXB272ofgn1VJWfaWTrAfQmsiDoFez5PD3");

#[program]
pub mod cpi_b {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from Program B");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'a> {
    pub pda_account: Signer<'a>
}

