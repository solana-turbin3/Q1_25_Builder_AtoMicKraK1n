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
}

impl From<CurveError> for AmmError {
    fn from(error: CurveError) -> AmmError {
        match error {
            CurveError::SlippageLimitExceeded => AmmError::SlippageExceeded,
            CurveError::InvalidPrecision => todo!(),
            CurveError::Overflow => todo!(),
            CurveError::Underflow => todo!(),
            CurveError::InvalidFeeAmount => todo!(),
            CurveError::InsufficientBalance => todo!(),
            CurveError::ZeroBalance => todo!(),
        }
    }
}
