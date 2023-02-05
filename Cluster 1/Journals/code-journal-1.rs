/* 

What are the concepts (borrowing, ownership, vectors etc)
What is the organization? 
What is the contract doing? What is the mechanism? 
How could it be better? More efficient? Safer? 
As we shift to week 2 (CLUSTER II- this one is two weeks long), your lens of Native Rust -> Anchor should be explored, annotated, and analyzed.  

*/

// Import Solana Program and grab necessary deps
use solana_program::{
    account_info::{ AccountInfo, next_account_info }, 
    entrypoint, 
    entrypoint::ProgramResult, 
    msg, 
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program,
};

// Declare the starting point for the Solana program...in this case, run the process_instructions function
entrypoint!(process_instruction);

// instructions pass the program id, all accounts used in the instruction as an array, and instruction data (in this case, it looks like none)
// note the instruction data is a u8 array (so it must be searialized)
// need to look into more why these have & before them...what would happen if thye weren't there. are they saying that they can be referenced from here throughout the code?
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
    // result is a ProgramResult (a type from teh solana_program library)
) -> ProgramResult {

    // You can verify the program ID from the instruction is in fact 
    //      the program ID of your program.
    // i don't like how this is structured. wish that check_id returned true if matched. oh well 
    // i think we could have imported system_program::check_id and just called check::id
    if system_program::check_id(program_id) {
        return Err(ProgramError::IncorrectProgramId)
    };
    
    // You can verify the list has the correct number of accounts.
    // This error will get thrown by default if you 
    //      try to reach past the end of the iter.
    // Should we also have a >= 5 logic as well? we can't hvae too many as well; i suppose we can they just don't do anything
    if accounts.len() < 4 {
        msg!("This instruction requires 4 accounts:");
        msg!("  payer, account_to_create, account_to_change, system_program");
        return Err(ProgramError::NotEnoughAccountKeys)
    };

    // Accounts passed in a vector must be in the expected order.
    // where's this .iter method coming from? 
    // this defines for our program which each of the 4 accounts passed in are. must submitted in the correct order
    let accounts_iter = &mut accounts.iter();
    let _payer = next_account_info(accounts_iter)?;
    let account_to_create = next_account_info(accounts_iter)?;
    let account_to_change = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // You can make sure an account has NOT been initialized.
    
    // .key() gets the publicy key of the account...seems that msg! auto converts to a string
    msg!("New account: {}", account_to_create.key);
    // make sure it's not already initialized w/ some sol
    if account_to_create.lamports() != 0 {
        msg!("The program expected the account to create to not yet be initialized.");
        return Err(ProgramError::AccountAlreadyInitialized)
    };
    // (Create account...)

    // You can also make sure an account has been initialized.
    msg!("Account to change: {}", account_to_change.key);
    // does the opposite...checking if there's no prior balance. 
    // note hte assumption here is that 0 balance = unitilialzed...curious if that's okay (e.g., i had acct and then withdrawal all sol)
    if account_to_change.lamports() == 0 {
        msg!("The program expected the account to change to be initialized.");
        return Err(ProgramError::UninitializedAccount)
    };

    // If we want to modify an account's data, it must be owned by our program.
    // .owner lives on the account struct...is there a way to see the struct like we do in TS?
    if account_to_change.owner != program_id {
        msg!("Account to change does not have the correct program id.");
        return Err(ProgramError::IncorrectProgramId)
    };

    // You can also check pubkeys against constants.
    // my hunch is that this is a security contstraint. 
    // why &system_program and not system_program?  does system_program.key not need &?
    // are these 2 variables w/ the same name??
    if system_program.key != &system_program::ID {
        return Err(ProgramError::IncorrectProgramId)
    };

    
    // okay, so this fn doesn't actually do anything...it just checks that the accounts submitted are valid but then are not used
    Ok(())
}


/*

What are the concepts (borrowing, ownership, vectors etc)
- seems major reference points here are hte parameters and then use of accounts.iter();
What is the organization?
- code is very organized and easy to read
What is the contract doing? What is the mechanism? 
- just verifying the account inputs. it doen't actually do anything
How could it be better? More efficient? Safer?
The code could be safer and better if…..add an acct.len >=5 check. though more accounts doesn't seem to hurt this program, it would reduce the liklihood of a user submitting wrong accounts (or misunderstanding account parameters. )

*/