use crate::states::GameAccountShape;
use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::anchor::{commit, delegate, ephemeral};
use ephemeral_rollups_sdk::cpi::DelegateConfig;
use ephemeral_rollups_sdk::ephem::{commit_accounts, commit_and_undelegate_accounts};

#[derive(Accounts)]
pub struct Initialize<'info> {
    //the program to fix the thing
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init, payer = signer, space = 8 + GameAccountShape::INIT_SPACE , seeds = [b"user_game_account", signer.key().as_ref()], bump)]
    pub user_game_account: Account<'info, GameAccountShape>,

    pub system_program: Program<'info, System>,
}
