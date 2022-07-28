use anchor_lang::prelude::*;

#[account]
pub struct Profile{
    pub owner:Pubkey,
    pub name:String,
    pub bio:String,
}

impl Profile {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH
        + STRING_LENGTH_PREFIX + MAX_NAME_LENGTH;
}



const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_NAME_LENGTH: usize = 50 * 4; 