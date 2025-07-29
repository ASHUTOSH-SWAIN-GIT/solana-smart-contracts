#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use anchor_spl::{token::TokenAccount, token_interface::TokenInterface};

declare_id!("FqzkXZdwYjurnUKetJCAvaUw5WAqbwzU6gZEwydeEfqS");

#[program]
pub mod vesting {
   pub fn create_vesting_account(ctx:Context<CreateVestingAccount> , company_name:String) -> Result<()> {
    
    *ctx.accounts.vesting_account = VestingAccount{
        owner: ctx.accounts.signer.key(),
        mint: ctx.accounts.mint.key(),
        treasury_token_account:ctx.accounts.treasury_token_account.key(),
        company_name,
        treasury_bump:ctx.bumps.treasury_token_account,
        bump:ctx.bumps.vesting_account,
    };
      Ok(())
   }
}

#[derive(Accounts)]
#[instruction(company_name:String)]
pub struct CreateVestingAccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        space = 8 + INIT_SPACE,
        payer = signer,
        seeds = [company_name.as_ref()],
         bump 
    )]
    pub vesting_account: Account<'info, VestingAccount>,
    pub mint: InterfaceAccount<'info , Mint>,
    
    #[account(
        init,
        token::mint = mint,
        token::authority = treasury_token_account,
        payer = signer,
        seeds = [b"vesting_treasury" , company_name.as_bytes()],
        bump,
    )]

    pub treasury_token_account: InterfaceAccount<'info , TokenAccount>,

    pub system_program: Program<'info , System>,
    pub token_program: Interface<'info , TokenInterface>,

}
#[account]
#[derive(InitSpace)]
pub struct VestingAccount {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub treasury_token_account: Pubkey,
    #[max_len(50)]
    pub company_name: String,
    pub treasury_bump: u8,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct EmployeeAccount {
    pub start_time : i64,
    pub end_time : i64,
    pub cliff_time : i64,
    pub vesting_account : Pubkey,
    pub total_amount : u64,
    pub total_withdraw : u64,
}