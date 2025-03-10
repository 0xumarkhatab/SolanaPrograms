use anchor_lang::prelude::*;

declare_id!("GyrNzRTrg7UfuWsMhPUbvfqPPkJSTe3xhTY7mQvcFV8q");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,msg:String) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        let data_account: &mut Account<'_, DataAccount>     = &mut ctx.accounts.data_account;
        data_account.msg=msg;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer:Signer<'info>,
    #[account{
            init,
            payer=signer,
            space=300,
        }]
    pub data_account:Account<'info,DataAccount>,
    pub system_program:Program<'info,System>

}

#[account]
pub struct DataAccount{
    pub msg:String
}


