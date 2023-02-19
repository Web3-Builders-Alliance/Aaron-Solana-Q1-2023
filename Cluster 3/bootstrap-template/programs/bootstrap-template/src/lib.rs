pub mod state;
pub mod constants;
pub mod instructions;
pub mod model;
pub mod id;

use anchor_lang::prelude::*;
use instructions::*;

pub use id::ID;

#[program]
pub mod bootstrap_template {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}
