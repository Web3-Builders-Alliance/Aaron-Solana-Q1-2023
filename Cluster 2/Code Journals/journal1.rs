// https://github.com/coral-xyz/anchor/blob/master/examples/tutorial/basic-4/programs/basic-4/src/lib.rs

use anchor_lang::prelude::*;
use std::ops::DerefMut; // dont recognize this. 

declare_id!("CwrqeMj2U8tFr1Rhkgwc84tpAsqbt9pTt2a4taoTADPr");

#[program]
pub mod basic_4 {
    // looks like this program defines 2 functions...initialize and increment. neither take in any program data
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {

        // counter here 
        let counter = ctx.accounts.counter.deref_mut();
        //https://doc.rust-lang.org/std/ops/trait.DerefMut.html
        //mutably dereferencing counter. 
        //The deref_mut method is used to get a mutable reference to the underlying data, so that the counter field can be updated later in the code.

        //ref the bump we used to defin ethe counter
        let bump = *ctx.bumps.get("counter").ok_or(ErrorCode::CannotGetBump)?;

        //interesting tway to create an account here. 
        //point to cointer 
        //similar to how we'd use a class/constructor in js
        *counter = Counter {
            authority: *ctx.accounts.authority.key,
            count: 0,
            bump,
        };

        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        // i like this way of validation. checking the authority pubkey (signer) == the authority defined in the pda
        require_keys_eq!(
            ctx.accounts.authority.key(),
            ctx.accounts.counter.authority,
            ErrorCode::Unauthorized
        );
        // simple increment
        ctx.accounts.counter.count += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority, 
        // cool use of impl
        space = Counter::SIZE,
        seeds = [b"counter"], // this means we only have 1 counter for program...need to add the auth here to make 1 per user
        // i acutally don't think this seed is congruent w/ teh inrement instruction.
        // seed only allows 1 counter per program. theincrement is checking authority...looking for just 1 user
        bump
    )]
    counter: Account<'info, Counter>, // using a Counter account, likely defined below
    #[account(mut)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(
        mut,
        seeds = [b"counter"],
        bump = counter.bump
    )]
    counter: Account<'info, Counter>,
    authority: Signer<'info>,
}

//define the counter struct
#[account]
pub struct Counter {
    pub authority: Pubkey,
    pub count: u64,
    pub bump: u8,
}

impl Counter {
    pub const SIZE: usize = 8 + 32 + 8 + 1;//descr+ pubkey+u64+u8
}

//add error handling
#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("Cannot get the bump.")]
    CannotGetBump,
}

//allinall way cleaner/easier than native rust love it!