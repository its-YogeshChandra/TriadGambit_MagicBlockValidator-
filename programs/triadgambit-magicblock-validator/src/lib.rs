use anchor_lang::prelude::*;

declare_id!("9JvQDQ3VNQA1jc4sEwfV7PVxkd934tmLrT3rt4hFk8DF");

#[program]
pub mod triadgambit_magicblock_validator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
