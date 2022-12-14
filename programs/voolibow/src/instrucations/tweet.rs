use anchor_lang::prelude::*;

use crate::error::ErrorCode;
use crate::state::Tweet;

#[derive(Accounts)]
pub struct SendTweet<'info>{
    #[account(init,payer = user,space = 8 + 32 + 8 + (4 + 50 * 4) + (4 + 280 * 4) + 1)]
    pub tweet:Account<'info,Tweet>,
    #[account(mut)]
    pub user:Signer<'info>,
    pub system_program:Program<'info,System> 
}
pub fn send_tweet(ctx:Context<SendTweet>,tag:String,content:String)->Result<()>{
    let tweet = &mut ctx.accounts.tweet;
    let user:&Signer = &ctx.accounts.user;
    let clock:Clock = Clock::get().unwrap();

    require!(tag.chars().count() <= 50, ErrorCode::TagTooLong);
    require!(content.chars().count() <= 280, ErrorCode::ContentTooLong);
    require!(content.chars().count() > 0, ErrorCode::NoContent);

    tweet.user = *user.key;
    tweet.timestamp = clock.unix_timestamp;
    tweet.tag = tag;
    tweet.content = content;
    tweet.edited = false;
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateTweet<'info>{
    #[account(mut,has_one = user)]
    pub tweet : Account<'info,Tweet>,
    pub user:Signer<'info>
}

pub fn update_tweet(ctx:Context<UpdateTweet>,new_tag:String,new_content:String)->Result<()>{
    let tweet = &mut ctx.accounts.tweet;

    require!(tweet.tag != new_tag || tweet.content != new_content, ErrorCode::NothingChanged);
    require!(new_tag.chars().count() <= 50, ErrorCode::TagTooLong);
    require!(new_content.chars().count() <= 280, ErrorCode::ContentTooLong);
    require!(new_content.chars().count() > 0, ErrorCode::NoContent);

    tweet.tag = new_tag;
	tweet.content = new_content;
	tweet.edited = true;
    Ok(())
}

#[derive(Accounts)]
pub struct DeleteTweet<'info>{
    #[account(mut,has_one = user, close = user)]
    pub tweet: Account<'info,Tweet>,
    pub user:Signer<'info>
}

pub fn delete_tweet(_ctx:Context<DeleteTweet>)->Result<()>{
    Ok(())
}