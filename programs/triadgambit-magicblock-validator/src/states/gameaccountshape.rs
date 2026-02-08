use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct GameAccountShape {
    #[max_len(50)]
    pub user_selected_element: String,

    #[max_len(50)]
    pub user_received_element: String,

    #[max_len(50)]
    pub result: String,
}
