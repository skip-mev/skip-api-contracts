use cosmwasm_std::{OverflowError, StdError};
use thiserror::Error;

pub type ContractResult<T> = core::result::Result<T, ContractError>;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    ///////////////
    /// GENERAL ///
    ///////////////

    #[error(transparent)]
    Std(#[from] StdError),

    #[error(transparent)]
    Overflow(#[from] OverflowError),

    #[error(transparent)]
    Payment(#[from] cw_utils::PaymentError),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Timeout Timestamp Less Than Current Timestamp")]
    Timeout,

    ////////////////
    /// FEE SWAP ///
    ////////////////

    #[error("Fee Swap Not Allowed: Post Swap Action Is Not An IBC Transfer")]
    FeeSwapNotAllowed,

    #[error("Fee Swap Operations Empty")]
    FeeSwapOperationsEmpty,

    #[error("Fee Swap Coin In Denom Differs From Coin Sent To Contract")]
    FeeSwapCoinInDenomMismatch,

    #[error("Fee Swap Coin In Denom Differs From First Swap Operation Denom In")]
    FeeSwapOperationsCoinInDenomMismatch,

    #[error("Fee Swap Coin Out Denom Differs From Last Denom Out In Swap Operations")]
    FeeSwapOperationsCoinOutDenomMismatch,

    #[error("Fee Swap Coin Out Does Not Equal IBC Fee Coin")]
    FeeSwapIbcFeeCoinMismatch,

    #[error("IBC Fees Are Provided But Not All The Same Denom")]
    IbcFeesNotOneCoin,

    /////////////////
    /// USER SWAP ///
    /////////////////

    #[error("User Swap Operations Empty")]
    UserSwapOperationsEmpty,

    #[error("User Swap Coin In Denom Differs From Coin Sent To Contract")]
    UserSwapCoinInDenomMismatch,

    #[error("User Swap Coin In Denom Differs From First Swap Operation Denom In")]
    UserSwapOperationsCoinInDenomMismatch,

    #[error("User Swap Last Swap Operation Denom Out Differs From Min Coin Out Denom")]
    UserSwapOperationsMinCoinDenomMismatch,

    #[error("User Swap Coin In Amount Is Greater Than The Remaining Coin Received")]
    UserSwapCoinInGreaterThanRemainingReceived,

    ////////////////////////
    /// POST SWAP ACTION ///
    ////////////////////////

    #[error("Received Less Coin From Swaps Than Minimum Coin Required")]
    ReceivedLessCoinFromSwapsThanMinCoin,

    #[error("Transfer Out Coin Less Than Minimum Required After Affiliate Fees")]
    TransferOutCoinLessThanMinAfterAffiliateFees,

    #[error("Transfer Out Coin Less Than Minimum Required After IBC Fees")]
    TransferOutCoinLessThanMinAfterIbcFees,
}
