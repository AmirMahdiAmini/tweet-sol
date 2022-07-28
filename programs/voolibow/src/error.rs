use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The provided name should be 50 characters long maximum.")]
    NameTooLong,
	#[msg("The provided data should be 60 characters long maximum.")]
    DataTooLong,
    #[msg("Exceeding maximum tag length of 50 characters")]
	TagTooLong,
	#[msg("Trying to send a tweet without content")]
	NoContent,
	#[msg("Exceeding maximum content length of 280 characters")]
	ContentTooLong,
	#[msg("No changes detected. Nothing that could be updated")]
	NothingChanged,
	
}