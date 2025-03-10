use anchor_lang::prelude::*;

declare_id!("13rLMW78wDBofP3jnJKoo7re8Hg95bEPjKBcHsR8bQpd");

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
    pub signer: Signer<'a>
}
