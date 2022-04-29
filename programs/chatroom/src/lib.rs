use anchor_lang::prelude::*;
use std::{mem::size_of,thread::AccessError};

pub mod events;
// pub use events;
pub use events::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

 

#[program]
pub mod chatroom {
    use crate::instruction::SendMessage;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, current_message: String, user1: Pubkey, user2: Pubkey) -> Result<()> {
        let chat_room = &mut ctx.accounts.chat_room;
        chat_room.current_message = current_message;
        chat_room.user1 = user1;
        chat_room.user2 = user2;
        Ok(())
    }
    pub fn sendMessage(ctx:Context<Send>, current_message: String, user: Pubkey) -> Result<()> {
        let chat_room = &mut ctx.accounts.chat_room;
        chat_room.current_message = current_message;
        chat_room.user1 = user;
        emit!(senderMessage{
            current_message: chat_room.current_message.to_string(),
            user: chat_room.user1,
            // timestamp: Clock::get()?.unix_timestamp,
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
#[account(init,payer=user,space=Chatroom::LEN)]
pub chat_room: Account<'info,Chatroom>,
#[account(mut)]
pub user: Signer<'info>,
pub system_program: Program<'info,System>
}

#[derive(Accounts)]
pub struct Send<'info>{
#[account(mut)]
pub chat_room: Account<'info,Chatroom>,   
}

#[account]
pub struct Chatroom{
    user1: Pubkey,
    user2: Pubkey,
    current_message: String,
}

impl Chatroom{
    pub const LEN:usize = 8 + size_of::<Pubkey>() + size_of::<Pubkey>() + size_of::<String>();
}
