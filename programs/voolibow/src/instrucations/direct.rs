use anchor_lang::prelude::*;

use crate::state::Direct;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct SendDirect<'info>{
    #[account(init,payer = user,space = 8 + 32 + 32 + 8 + (4 * 280 * 4) + 1)]
    pub direct: Account<'info,Direct>,
    #[account(mut)]
    pub user:Signer<'info>,
    pub system_program:Program<'info,System>
}
pub fn send_direct(ctx: Context<SendDirect>,recipient:Pubkey,content:String)->Result<()>{
    let direct = &mut ctx.accounts.direct;
    let user = &ctx.accounts.user;
    let clock = Clock::get().unwrap();

    require!(content.chars().count() <= 280,ErrorCode::ContentTooLong);
    direct.user = *user.key;
    direct.recipient = recipient;
    direct.timestamp = clock.unix_timestamp;
    direct.content = content;
    direct.edited = false;
    Ok(())
}
#[derive(Accounts)]
pub struct UpdateDirect<'info>{
    #[account(mut,has_one = user)]
    pub direct:Account<'info,Direct>,
    pub user:Signer<'info>
}
pub fn update_direct(ctx:Context<UpdateDirect>,new_content:String)->Result<()>{
    let direct  =&mut  ctx.accounts.direct;

    require!(direct.content != new_content,ErrorCode::NothingChanged);
    
    direct.content = new_content;
    direct.edited = true;
    Ok(())
}
#[derive(Accounts)]
pub struct DeleteDirect<'info>{
    #[account(mut,has_one = user,close = user)]
    pub direct:Account<'info,Direct>,
    pub user:Signer<'info>
}
pub fn delete_direct(_ctx:Context<DeleteDirect>)->Result<()>{
    Ok(())
}
