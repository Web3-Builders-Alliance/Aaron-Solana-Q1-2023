/* 
Journal from https://github.com/joncinque/solana-program-library/blob/e88e9f8331660ba99838a359e0cde31ff4d58690/associated-token-account/program/src/processor.rs
SPL Token Library CreateAccount



What are the concepts (borrowing, ownership, vectors etc)
What is the organization? 
What is the contract doing? What is the mechanism? 
How could it be better? More efficient? Safer? 
As we shift to week 2 (CLUSTER II- this one is two weeks long), your lens of Native Rust -> Anchor should be explored, annotated, and analyzed.  


*/

//! Program state processor


use {
    
// Importsfrom other files in the library (this will be helpful to track for organizing larger projects)
    crate::{
        error::AssociatedTokenAccountError,
        instruction::AssociatedTokenAccountInstruction,
        tools::account::{create_pda_account, get_account_len},
        *,
    },
    //curious to know what the difference between different serialization tools are
    borsh::BorshDeserialize,
    // pretty standard solana imports here
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program::invoke,
        program_error::ProgramError,
        pubkey::Pubkey,
        rent::Rent,
        system_program,
        sysvar::Sysvar,
    },
    // I don't immediately know what these are...i'll see if i can figure out by context later on. 
    spl_token::{extension::StateWithExtensions, state::Account},
};


/// Specify when to create the associated token account
// Something unclear to me here... What is `PartialEq`? it' snot imported
// looked it up: https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#derivable
// what would be different if this wasn't derived here? 
#[derive(PartialEq)]
enum CreateMode {
    /// Always try to create the ATA
    Always,
    /// Only try to create the ATA if non-existent
    Idempotent,
}

/// Instruction processor
// pretty standard. a
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    //if no address is passed in the input, we are going to assume to create a new acct
    //note this if format is kind of like in JS using inline let x = y ?? a : b
    let instruction = if input.is_empty() {
        AssociatedTokenAccountInstruction::Create
    } else {
        // else slice our input.
        AssociatedTokenAccountInstruction::try_from_slice(input)
            .map_err(|_| ProgramError::InvalidInstructionData)?
    };

    msg!("{:?}", instruction);

    // so this is pretty cool, we're letting the user pass in an idempotent instruction OR nothing. if nothing, we'll create, otherwsise we'll createIdempotent
    match instruction {
        AssociatedTokenAccountInstruction::Create => {
            process_create_associated_token_account(program_id, accounts, CreateMode::Always)
        }
        AssociatedTokenAccountInstruction::CreateIdempotent => {
            process_create_associated_token_account(program_id, accounts, CreateMode::Idempotent)
        }
    }
}

/// Processes CreateAssociatedTokenAccount instruction
fn process_create_associated_token_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    // note that we're passing in the create mode from the previous step
    create_mode: CreateMode,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    // IMO in native solana contracts, doing all the iter up front like this is the way to go. 
    let funder_info = next_account_info(account_info_iter)?;
    let associated_token_account_info = next_account_info(account_info_iter)?;
    let wallet_account_info = next_account_info(account_info_iter)?;
    let spl_token_mint_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;
    let spl_token_program_info = next_account_info(account_info_iter)?;
    let spl_token_program_id = spl_token_program_info.key;

    // if mode is demopotent and the passed assoc account owner is the right program (SPL token program pub key)
    if create_mode == CreateMode::Idempotent
        && associated_token_account_info.owner == spl_token_program_id
    {
        // get a read only reference to the ata account info
        let ata_data = associated_token_account_info.data.borrow();
        //check if associated_token_account is an account. (this is cool implementation...a little tricky to read but i like it)
        // checking if the result of the unpack method on StateWithExtensions is Ok.
        // didn't find much good doc on StateWithExtensions, buti think it just means it's an account that's already initiated
        if let Ok(associated_token_account) = StateWithExtensions::<Account>::unpack(&ata_data) {
            //making sure that we're not passing in an incorrect owner. 
            if associated_token_account.base.owner != *wallet_account_info.key {
                let error = AssociatedTokenAccountError::InvalidOwner;
                msg!("{}", error);
                return Err(error.into());
            }
            // same...just checking mint. both of these are really just making sure we have the right PDA matching our owner/mint pair
            if associated_token_account.base.mint != *spl_token_mint_info.key {
                return Err(ProgramError::InvalidAccountData);
            }

            // so what we did here was check that the ATA already exists. if it does exist, we're going to exit the program w/ OK
            // otherwise, we'll keep moving along w/ the code (presumably to create the account)
            return Ok(());
        }
    }

    // before creating, we need to make sure this acct hasn't been initiated by another program
    if *associated_token_account_info.owner != system_program::id() {
        return Err(ProgramError::IllegalOwner);
    }

    // we gonna reference the solana rent program
    //https://docs.rs/solana-program/latest/solana_program/sysvar/trait.Sysvar.html#method.get
    let rent = Rent::get()?;

    // derive the pda here...i suspect we'll check this == associated_token_account_info.key
    let (associated_token_address, bump_seed) = get_associated_token_address_and_bump_seed_internal(
        wallet_account_info.key,
        spl_token_mint_info.key,
        program_id,
        spl_token_program_id,
    );

    //YEP here's the check. using * to deref. still a little hairy on this but will keep an eye for it
    if associated_token_address != *associated_token_account_info.key {
        msg!("Error: Associated address does not match seed derivation");
        return Err(ProgramError::InvalidSeeds);
    }

    // create the seeds for using the program to sign the pda. 
    let associated_token_account_signer_seeds: &[&[_]] = &[
        &wallet_account_info.key.to_bytes(),
        &spl_token_program_id.to_bytes(),
        &spl_token_mint_info.key.to_bytes(),
        &[bump_seed],
    ];

    //  i don't know why we need this...
    // looked it up: https://github.com/solana-labs/solana-program-library/blob/master/associated-token-account/program/src/tools/account.rs
    // determines data length 
    let account_len = get_account_len(
        spl_token_mint_info,
        spl_token_program_info,
        &[spl_token::extension::ExtensionType::ImmutableOwner],
    )?;
    // very similar to the web3js parameters
    create_pda_account(
        funder_info,
        &rent,
        account_len,
        spl_token_program_id,
        system_program_info,
        associated_token_account_info,
        associated_token_account_signer_seeds,
    )?;

    msg!("Initialize the associated token account");

    // set spl program as owner. i don't really understand the 2nd parameter

    // found this. pretty handy. https://spl.solana.com/token-2022/onchain
    invoke(
        &spl_token::instruction::initialize_immutable_owner(
            spl_token_program_id,
            associated_token_account_info.key,
        )?,
        &[
            associated_token_account_info.clone(),
            spl_token_program_info.clone(),
        ],
    )?;
    invoke(
        &spl_token::instruction::initialize_account3(
            spl_token_program_id,
            associated_token_account_info.key,
            spl_token_mint_info.key,
            wallet_account_info.key,
        )?,
        &[
            associated_token_account_info.clone(),
            spl_token_mint_info.clone(),
            wallet_account_info.clone(),
            spl_token_program_info.clone(),
        ],
    )
}

/*

What are the concepts (borrowing, ownership, vectors etc)
- what i like about this is the use of conditionals to direct certain outcomes/actions. i think this can give us a lot of flex to create vs recreate
What is the organization?
- code is very organized and easy to read
What is the contract doing? What is the mechanism? 
- creating a new SPL token account if one doesn't exist already using pda derviation checks. 
How could it be better? More efficient? Safer?
The code could be safer and better if….
--dunno...lots of really good checks and errors in here. 


NOTE: since this was a long one...just gonna do 2 journals this week!
*/