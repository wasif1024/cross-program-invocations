use anchor_lang::prelude::*;

// Program that holds a simple u64 value and an authority that is allowed to
// update it. The master program will exercise this over CPI.
declare_id!("DMAgXZKGf4uM2rXex25YeLQbzMmZjR9hRKHdNvmdZtGf");

#[program]
pub mod child {
    use super::*;

    // Create the child account and remember who controls future updates.
    pub fn initialize(_ctx: Context<Initialize>, authority: Pubkey) -> Result<()> {
        _ctx.accounts.child.authority = authority;
        Ok(())
    }

    // Allow the authority to write a new value into the child account.
    pub fn set_data(ctx: Context<SetData>, data: u64) -> Result<()> {
        let child = &mut ctx.accounts.child;
        child.data = data;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // New PDA-like account that stores the data and authority.
    #[account(init, payer = user, space = 8 + 8 + 32)]
    pub child: Account<'info, Data>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetData<'info> {
    // Only the recorded authority may perform this CPI.
    #[account(mut, has_one = authority)]
    pub child: Account<'info, Data>,
    pub authority: Signer<'info>,
}

#[account]
pub struct Data {
    pub data: u64,
    pub authority: Pubkey,
}
