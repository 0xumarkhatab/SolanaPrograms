use anchor_lang::prelude::*;

declare_id!("9JDzuztBjdYuZLeovgUhnbBLaudDQiTJ5wsePgR2zwXG");

#[program]
pub mod cpi_a {
    use anchor_lang::solana_program::{program::{invoke, invoke_signed}, system_instruction};

    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from Program CPI-A");
        let pda_address = _ctx.accounts.pda_account.key();
        let signer_address = _ctx.accounts.signer.key();
        // let bump = _ctx.bumps.pda_account;

        let instruction = &system_instruction::transfer(&pda_address, &signer_address, 1_000_000_000);


        // let signers_seeds: &[&[&[u8]]]= &[&[b"superForgerer", &signer_address.as_ref(),&[bump]]];
        let account_infos = [
            _ctx.accounts.pda_account.to_account_info(),
            _ctx.accounts.signer.to_account_info(),
            _ctx.accounts.system_program.to_account_info()
            ];
        
        // invoke_signed(instruction, &account_infos, signers_seeds)?;
        invoke(instruction, &account_infos)?;
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK:nothing
    #[account(
        mut,
        seeds=[b"superForgerer",signer.key().as_ref()],
        bump,
    )]
    pub pda_account: AccountInfo<'info>,
    #[account(mut)]
    pub signer:Signer<'info>,
    pub system_program:Program<'info,System>
    
}
