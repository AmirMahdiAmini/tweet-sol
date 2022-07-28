use anchor_lang::prelude::*;

use crate::error::ErrorCode;
use crate::state::Profile;

#[derive(Accounts)]
pub struct CreateProfile<'info>{
    #[account(
        init, payer = owner, space = Profile::LEN,
    )]
    pub profile:Account<'info,Profile>,
    #[account(mut)]
    pub owner:Signer<'info>,
    pub system_program:Program<'info,System>,
}
pub fn create_profile(ctx : Context<CreateProfile>, name:String,bio:String)->Result<()>{

    require!(name.chars().count()>=50,ErrorCode::NameTooLong);
    require!(bio.chars().count()>=60,ErrorCode::DataTooLong);

    let profile: &mut Account<Profile> = &mut ctx.accounts.profile;
    let owner: &Signer = &ctx.accounts.owner;

    profile.owner = *owner.key;
    profile.name =  name;
    profile.bio =  bio;
    msg!("Profile Created");
    Ok(())
}