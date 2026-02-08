use crate::states::GameAccountShape;
use anchor_lang::prelude::*;
use rand;
use rand::seq::{IndexedRandom, SliceRandom};
#[derive(Accounts)]
pub struct PlayGame<'info> {
    //the program to fix the thing
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub user_game_account: Account<'info, GameAccountShape>,

    pub system_program: Program<'info, System>,
}

impl<'info> PlayGame<'info> {
    pub fn update_user_account(&mut self, user_turn: String) -> Result<()> {
        let mut rng = rand::rng();
        let mut nums: Vec<i32> = (0..2).collect();
        nums.shuffle(&mut rng);
        let selected_index = nums.choose(&mut rng).unwrap();

        let game_element = ["rock", "paper", "scissors"];
        let selected_game_element = game_element[*selected_index as usize];

        //change the top from the thing
        if user_turn == selected_game_element {
            self.user_game_account.user_selected_element = user_turn;
            self.user_game_account.user_received_element = selected_game_element.to_string();
            self.user_game_account.result = "tie".to_string();

            println!("It's a tie!");
        } else if (user_turn == "rock" && selected_game_element == "scissors")
            || (user_turn == "paper" && selected_game_element == "rock")
            || (user_turn == "scissors" && selected_game_element == "paper")
        {
            self.user_game_account.user_selected_element = user_turn;
            self.user_game_account.user_received_element = selected_game_element.to_string();
            self.user_game_account.result = "win".to_string();
        } else {
            // If it's not a tie and you didn't win, you lost.
            self.user_game_account.user_selected_element = user_turn;
            self.user_game_account.user_received_element = selected_game_element.to_string();
            self.user_game_account.result = "lose".to_string();
        }
        Ok(())
    }
}
