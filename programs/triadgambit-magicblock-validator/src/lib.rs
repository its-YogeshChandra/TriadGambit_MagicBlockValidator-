use anchor_lang::prelude::*;

declare_id!("9JvQDQ3VNQA1jc4sEwfV7PVxkd934tmLrT3rt4hFk8DF");

#[program]
pub mod triadgambit_magicblock_validator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn play_game(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}
#[account]
#[derive(InitSpace)]
pub struct GameAccountShape {
    #[max_len(50)]
    user_selected_element: String,

    #[max_len(50)]
    user_received_element: String,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    //the program to fix the thing
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init, payer = signer, space = 8 + GameAccountShape::INIT_SPACE , seeds = [b"user_game_account", signer.key().as_ref()], bump)]
    pub user_game_account: Account<'info, GameAccountShape>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlayGame<'info> {
    //the program to fix the thing
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub user_game_account: Account<'info, GameAccountShape>,

    pub system_program: Program<'info, System>,
}
