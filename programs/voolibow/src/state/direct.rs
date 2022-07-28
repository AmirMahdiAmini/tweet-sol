use anchor_lang::prelude::*;

#[account]
pub struct Direct{
    pub user:Pubkey,
    pub recipient:Pubkey,
    pub timestamp:i64,
    pub content:String,
    pub edited:bool,
}
