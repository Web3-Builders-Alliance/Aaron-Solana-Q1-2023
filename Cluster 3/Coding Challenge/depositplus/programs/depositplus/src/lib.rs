use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer as SplTransfer}
};
declare_id!("GeTM4QBoYJ6sStEohmNRiDiARJTK4jjNsjxsVp6A5jHc");

#[program]
pub mod depositplus {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let deposit_account = &mut ctx.accounts.vault;
        deposit_account.owner = ctx.accounts.initializer.key();
        deposit_account.balance = 0;
        deposit_account.bump = *ctx.bumps.get("vault").unwrap();

        Ok(())
    }
    pub fn deposit (ctx: Context<Deposit>, deposit_amount: u64) -> Result<()> {

        let deposit_account = &mut ctx.accounts.vault;
        let owner_main_account = &mut ctx.accounts.owner;
        require_keys_eq!(owner_main_account.key(),deposit_account.owner,BankError::Unauthorized);
        require!(**owner_main_account.try_borrow_lamports()? >= deposit_amount, BankError::InsufficientFunds);
    
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &owner_main_account.key(),
            &deposit_account.key(),
            deposit_amount,
        );
        
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                owner_main_account.to_account_info(),
                deposit_account.to_account_info(),
            ],
        )?;

        deposit_account.balance += deposit_amount;

        Ok(())
    }
    pub fn withdraw(ctx: Context<Withdraw>, withdraw_amount: u64) -> Result<()> {
        let receiver = &mut ctx.accounts.owner;
        let vault = &mut ctx.accounts.vault;
        require_keys_eq!(receiver.key(),vault.owner,BankError::Unauthorized);
        require!(withdraw_amount <= vault.balance, BankError::InsufficientFunds);

        **vault.to_account_info().try_borrow_mut_lamports()? -= withdraw_amount;
        **receiver.try_borrow_mut_lamports()? += withdraw_amount;
        vault.balance -= withdraw_amount;

        Ok(())
    }
    pub fn deposit_spl(ctx: Context<DepositSpl>, deposit_amount: u64) -> Result<()> {
        let destination = &mut ctx.accounts.to_ata;
        let source = &mut ctx.accounts.from_ata;
        let authority = &mut ctx.accounts.payer;
        let token_program = &mut ctx.accounts.token_program;


        // Transfer tokens from taker to initializer
        let cpi_accounts = SplTransfer {
            from: source.to_account_info().clone(),
            to: destination.to_account_info().clone(),
            authority: authority.to_account_info().clone(),
        };
        let cpi_program = token_program.to_account_info();
        
        token::transfer(
            CpiContext::new(cpi_program, cpi_accounts),
            deposit_amount)?;

        Ok(())
    }

    pub fn withdraw_spl(ctx: Context<WithdrawSpl>, withdraw_amount: u64) -> Result<()> {
        let destination = &mut ctx.accounts.to_ata;
        let vault = &mut ctx.accounts.vault;
        let source = &mut ctx.accounts.from_ata;
        let token_program = &mut ctx.accounts.token_program;
        let auth = &mut ctx.accounts.payer;
        

        // Transfer tokens from taker to initializer
        let cpi_accounts = SplTransfer {
            from: source.to_account_info().clone(),
            to: destination.to_account_info().clone(),
            authority: vault.to_account_info().clone(),
        };
        let seeds = &[
            b"vault",
            auth.to_account_info().key.as_ref(),
            &[vault.bump],
        ];
        let signer = &[&seeds[..]];
        let cpi = CpiContext::new_with_signer(
            token_program.to_account_info(),
            cpi_accounts,
            signer,
        );
        anchor_spl::token::transfer(cpi, withdraw_amount)?;

        Ok(())
    }


}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    #[account(
        init, 
        payer = initializer, 
        space = Vault::LEN,
        seeds=[b"vault".as_ref(), initializer.key().as_ref()], 
        bump
    )]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds=[b"vault".as_ref(), owner.key().as_ref()], 
        bump
    )]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct DepositSpl<'info> {
    
    #[account(mut)]
    pub payer: Signer<'info>,

    // note: will have the same SOL vault we use to 'own' the token account - won't change here 
    #[account(
        seeds=[b"vault".as_ref(), payer.to_account_info().key().as_ref()], 
        bump
    )]
    pub vault: Account<'info, Vault>,

    // the payer must send the tokens
    #[account(
        mut,
        //mint = token_mint,
        //owner = payer.to_account_info().key()
    )]
    pub from_ata: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        associated_token::mint = token_mint,
        payer = payer,
        associated_token::authority = vault,
    )]
    pub to_ata: Account<'info, TokenAccount>,

    pub token_mint: Account<'info, Mint>, 
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,    
    pub system_program: Program<'info, System>,
    
}

#[derive(Accounts)]
pub struct WithdrawSpl<'info> {
    
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds=[b"vault".as_ref(), payer.to_account_info().key().as_ref()], 
        bump
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub from_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to_ata: Account<'info, TokenAccount>,

    pub token_mint: Account<'info, Mint>, 
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,    
    pub system_program: Program<'info, System>,
    
}






#[account]
pub struct Vault {
    owner: Pubkey,
    bump: u8,
    balance: u64
    //token_account: Pubkey
}

impl Vault {
    const LEN: usize = 
        8 +     // discriminator
        1 +     // bump
        32 +    // owner
        8;      //balance
}



#[error_code]
pub enum BankError {
    #[msg("Trying to withdraw more funds than in the account")]
    InsufficientFunds,
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
}
