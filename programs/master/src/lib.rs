use anchor_lang::prelude::*;
use child::cpi::accounts::SetData;
use child::program::Child;
use child::{self,Data};
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

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
        child::cpi::set_data(ctx.accounts.set_data_ctx(), data)
    }
}

#[derive(Accounts)]
pub struct PullStrings<'info> {
    #[account(mut)]
    pub data: Account<'info, Data>,
    pub child_program: Program<'info, Child>,
}
impl<'info> PullStrings<'info> {
    pub fn set_data_ctx(&self) -> CpiContext<'_, '_, '_, 'info, SetData<'info>> {
        let cpi_program = self.child_program.to_account_info();
        let cpi_accounts = SetData {
            child: self.data.to_account_info()
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
