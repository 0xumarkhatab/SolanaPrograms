
use anchor_lang::prelude::*;

use cpi_b::program::CpiB;
use cpi_b::cpi::initialize;

declare_id!("EmN7bh9JyNB9PPDYKt17uvM7W74hT25nFdoGyYFFj9zE");

#[program]
pub mod cpi_a {
    use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from Program CPI-A");
        let signer_address = _ctx.accounts.signer.key();
        let bump = _ctx.bumps.pda_account;



        let signer_seeds: &[&[&[u8]]]= &[&[b"superForgerer", &signer_address.as_ref(),&[bump]]];
        
        /*****  Sol transfer from pda to signer *******/
        // let pda_address = _ctx.accounts.pda_account.key();
        // let instruction = &system_instruction::transfer(&pda_address, &signer_address, 1_000_000_000);        
        // let account_infos = [
        //     _ctx.accounts.pda_account.to_account_info(),
        //     _ctx.accounts.signer.to_account_info(),
        //     _ctx.accounts.system_program.to_account_info()
        //     ];
        
        // invoke_signed(instruction, &account_infos, signers_seeds)?;

        let cpi_ctx = CpiContext::new_with_signer(
            _ctx.accounts.cpi_b.to_account_info(),
            cpi_b::cpi::accounts::Initialize{pda_account:_ctx.accounts.pda_account.to_account_info()},
             signer_seeds
        );
        cpi_b::cpi::initialize(cpi_ctx)?;
        
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
    pub system_program:Program<'info,System>,
    pub cpi_b:Program<'info,CpiB>
    
}
