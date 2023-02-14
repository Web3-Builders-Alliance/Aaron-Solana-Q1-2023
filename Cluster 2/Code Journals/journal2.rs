// SRC: https://github.com/Woody4618/workshops_fork/blob/main/workshops/tug-of-war/program/src/lib.rs


use anchor_lang::prelude::*;
//solana-keygen grind --starts-with tug:1
declare_id!("tugLiwCj74Nb5uNqtVgtoQ3x95Jhctz2RDRdLwmG9dF");

#[program]
pub mod tug_of_war {
    use super::*;

    // using constants -- i will do more of this. 
    const MAX_POSITION: u16 = 20;

    // on the surface initialize looks like it does nothing
    // look at Initialize contenxt and yuou see that anchor inits a PDA
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        // i think we should probably set 
        // player_position = maxposition/2 
        // or impl a default value
        Ok(())
    }

    pub fn restart_game(ctx: Context<Restart>) -> Result<()> {
        let game_data_account = &mut ctx.accounts.game_data_account;

        // logic here to prevent ending a game early
        // probably best practice here to set 0 as a min constant as well
        if game_data_account.player_position > 0 && game_data_account.player_position < MAX_POSITION {
            panic!("Cant restart game, game is still running!");
        }

        // probably should have some rounding logic here to make sure we end up w/ integer value
        // question: what is 13/2 if both values are u intead of f?
        // looked it up:  In Rust, when you divide two integers using the / operator, the remainder is discarded and only the quotient is returned.
        game_data_account.player_position = MAX_POSITION / 2;
        Ok(())
    }

    pub fn pull_left(ctx: Context<MoveLeft>) -> Result<()> {
        let game_data_account = &mut ctx.accounts.game_data_account;
        
        if game_data_account.player_position <= 0 || game_data_account.player_position >= MAX_POSITION {
            panic!("Cant pull left, game is over!");
        }

        // seems a little weird to me to check the winner before moving the position. i'd probably flip the if/else here
        if game_data_account.player_position <= 0 {
            msg!("Team Left won! \\o/");
            display_game(game_data_account.player_position);
        } else {
            game_data_account.player_position -= 1;
            // i don't really see what this function is doing...it's not setting state anywhere
            display_game(game_data_account.player_position);
        }
        Ok(())
    }

    // i'd probably clean this up a bit and have a pull fn w/ data passing left or right sicne this is otherwise pretty redundant
    pub fn pull_right(ctx: Context<MoveRight>) -> Result<()> {
        let game_data_account = &mut ctx.accounts.game_data_account;

        if game_data_account.player_position <= 0 || game_data_account.player_position >= MAX_POSITION {
            panic!("Cant pull right, game is over!");
        }

        if game_data_account.player_position >= MAX_POSITION {
            msg!("Team Right won! \\o/");
            display_game(game_data_account.player_position);
        } else {
            game_data_account.player_position = game_data_account.player_position + 1;
            display_game(game_data_account.player_position);
        }
        Ok(())
    }

}

// this is pretty weird. we're just fetching a string, but i don't think we're doing anythign with it
fn display_game(position: u16) -> &'static str{
    match position {
          0 => "\\o/-------|-------OOO____________________",
          1 => "_ooo-------|-------OOO___________________",
          2 => "__ooo-------|-------OOO__________________",
          3 => "___ooo-------|-------OOO_________________",
          4 => "____ooo-------|-------OOO________________",
          5 => "_____ooo-------|-------OOO_______________",
          6 => "______ooo-------|-------OOO______________",
          7 => "_______ooo-------|-------OOO_____________",
          8 => "________ooo-------|-------OOO____________",
          9 => "_________ooo-------|-------OOO___________",
         10 => "__________ooo-------|-------OOO__________",
         11 => "___________ooo-------|-------OOO_________",
         12 => "____________ooo-------|-------OOO________",
         13 => "_____________ooo-------|-------OOO_______",
         14 => "______________ooo-------|-------OOO______",
         15 => "_______________ooo-------|-------OOO_____",
         16 => "________________ooo-------|-------OOO____",
         17 => "_________________ooo-------|-------OOO___",
         18 => "__________________ooo-------|-------OOO__",
         19 => "___________________ooo-------|-------OOO_",
         20 => "____________________ooo-------|-------\\o/",
        _ => "",
    }
}

#[derive(Accounts)]
// this is awesome--allows for the function to just say Ok(()) and get a new PDA using init
pub struct Initialize<'info> {
    // We must specify the space in order to initialize an account.
    // First 8 bytes are default account discriminator,
    // next 2 byte come from NewAccount.data being type i16.
    // (u16 = 16 bits signed integer = 8 bytes)
    #[account(
        init,
        // this means only 1 PDA for the program...which allows anybody to play. that's pretty cool but also limiting
        seeds = [b"tug_of_war"],
        bump,
        payer = signer,
        space = 8 + 2
    )]
    pub new_game_data_account: Account<'info, GameDataAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

//interesting to see how bare these accounts are 
// i would actually imagine this doesn't work.somebody needs to pay the tx fees. 
// perhaps the point here is that they don't matter (e.g., anybody can pay), so they can be excluded in anchor? 
#[derive(Accounts)]
pub struct MoveLeft<'info> {
    #[account(mut)]
    pub game_data_account: Account<'info, GameDataAccount>,
}

#[derive(Accounts)]
pub struct MoveRight<'info> {
    #[account(mut)]
    pub game_data_account: Account<'info, GameDataAccount>,
}

#[derive(Accounts)]
pub struct Restart<'info> {
    #[account(mut)]
    pub game_data_account: Account<'info, GameDataAccount>,
}

// just interesting to think of setting state really small here. 
// makes me think of lots of potential use cases
#[account]
pub struct GameDataAccount {
    pub player_position: u16,
}