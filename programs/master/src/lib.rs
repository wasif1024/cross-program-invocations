use anchor_lang::prelude::*;
use child::cpi::accounts::SetData;
use child::program::Child;
use child::{self,Data};
declare_id!("9SfdzDgtgwcazgzhQcZSbr1mVgWZuGcP5X5GVRfM2ydM");

#[program]
pub mod master {
    use super::*;

    pub fn pull_strings(ctx: Context<PullStrings>, data: u64) -> Result<()> {
        /*let cpi_program = ctx.accounts.puppet_program.to_account_info();
        let cpi_accounts = SetData {
            child: ctx.accounts.data.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        //puppet::cpi::set_data(cpi_ctx, data)
        child::cpi::set_data(cpi_ctx, data);*/
        child::cpi::set_data(ctx.accounts.set_data_ctx(), data)?;
        //This reload will load latest data of accounts after setting
        ctx.accounts.child.reload()?;
        if ctx.accounts.child.data != 42 {
            panic!();
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PullStrings<'info> {
    #[account(mut)]
    pub child: Account<'info, Data>,
    pub child_program: Program<'info, Child>,
     // Even though the puppet program already checks that authority is a signer
    // using the Signer type here is still required because the anchor ts client
    // can not infer signers from programs called via CPIs
    pub authority: Signer<'info>
}
impl<'info> PullStrings<'info> {
    pub fn set_data_ctx(&self) -> CpiContext<'_, '_, '_, 'info, SetData<'info>> {
        let cpi_program = self.child_program.to_account_info();
        let cpi_accounts = SetData {
            child: self.child.to_account_info(),
            authority:self.authority.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
