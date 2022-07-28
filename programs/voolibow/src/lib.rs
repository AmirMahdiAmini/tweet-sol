use instrucations::*;

mod error;
mod instrucations;
mod state;
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod voolibow {
    use super::*;
    pub fn create_profile(ctx: Context<CreateProfile>,name:String,bio:String) -> Result<()> {
        instrucations::create_profile(ctx,name,bio)
    }
    pub fn send_tweet(ctx:Context<SendTweet>,tag:String,content:String)->Result<()>{
        instrucations::send_tweet(ctx, tag, content)
    }
    pub fn update_tweet(ctx:Context<UpdateTweet>,new_tag:String,new_content:String)->Result<()>{
        instrucations::update_tweet(ctx, new_tag, new_content)
    }
    pub fn delete_tweet(ctx:Context<DeleteTweet>)->Result<()>{
        instrucations::delete_tweet(ctx)
    }
    pub fn send_direct(ctx: Context<SendDirect>,recipient:Pubkey,content:String)->Result<()>{
        instrucations::send_direct(ctx, recipient, content)
    }
    pub fn update_direct(ctx:Context<UpdateDirect>,new_content:String)->Result<()>{
        instrucations::update_direct(ctx, new_content)
    }
    pub fn delete_direct(ctx:Context<DeleteDirect>)->Result<()>{
        instrucations::delete_direct(ctx)
    }
}
