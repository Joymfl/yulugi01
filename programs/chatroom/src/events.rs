use crate::*;

#[event]
pub struct senderMessage{
    pub current_message: String,
    pub user: Pubkey,
    // timestamp: i64,

}