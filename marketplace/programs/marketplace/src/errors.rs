use anchor_lang::error_code;

#[error_code]
pub enum MarketplaceError{
    #[msg("Name must be at least 1 character long")]
    NameTooLong,
}