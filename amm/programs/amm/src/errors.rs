//pl
//sexc
//invamt

use anchor_lang::error_code;
use constant_product_curve::CurveError;


#[error_code]
pub enum AmmError {
    #[msg("DefaultError")]
    DefaultError,
    #[msg("Offer expired.")]
    OfferExpired,
    #[msg("This pool is locked.")]
    PoolLocked,
    #[msg("Slippage exceeded.")]
    SlippageExceeded,
    #[msg("Invalid amount.")]
    InvalidAmount,
    #[msg("Invalid precision.")]
    InvalidPrecision,
    #[msg("Overflow detected.")]
    Overflow,
    #[msg("Underflow detected.")]
    Underflow,
    #[msg("Invalid fee amount.")]
    InvalidFeeAmount,
    #[msg("Insufficient balance.")]
    InsufficientBalance,
    #[msg("Zero balance.")]
    ZeroBalance,
}

impl From<CurveError> for AmmError {
    fn from(error: CurveError) -> AmmError {
        match error {
            CurveError::SlippageLimitExceeded => AmmError::SlippageExceeded,
            CurveError::InvalidPrecision => AmmError::InvalidPrecision,
            CurveError::Overflow => AmmError::Overflow,
            CurveError::Underflow => AmmError::Underflow,
            CurveError::InvalidFeeAmount => AmmError::InvalidFeeAmount,
            CurveError::InsufficientBalance => AmmError::InsufficientBalance,
            CurveError::ZeroBalance => AmmError::ZeroBalance,
        }
    }
}
