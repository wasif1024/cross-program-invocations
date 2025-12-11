use anchor_lang::prelude::*;
use child::cpi::accounts::SetData;
use child::program::Child;
use child::{self, Data};

// Master program drives a CPI into the child program to write a u64 value.
declare_id!("DpvoTtkJ9xTbaSDZf6G9B2W1zGDKiDtoVuPEbYgzQ4TC");

#[program]
pub mod master {
    use super::*;

    // Perform a CPI into the child program to set the stored value.
    // The test expects the CPI to write the literal 42.
    pub fn pull_strings(ctx: Context<PullStrings>, data: u64) -> Result<()> {
        // Build CPI context and call the child program.
        child::cpi::set_data(ctx.accounts.set_data_ctx(), data)?;

        // Reload to observe the value written by the CPI without another RPC.
        ctx.accounts.child.reload()?;

        // Simple sanity check used by the example test.
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
    // Anchor still needs the explicit signer to pass through the CPI.
    pub authority: Signer<'info>,
}
impl<'info> PullStrings<'info> {
    // Helper for building the CPI context into the child program.
    pub fn set_data_ctx(&self) -> CpiContext<'_, '_, '_, 'info, SetData<'info>> {
        let cpi_program = self.child_program.to_account_info();
        let cpi_accounts = SetData {
            child: self.child.to_account_info(),
            authority: self.authority.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
