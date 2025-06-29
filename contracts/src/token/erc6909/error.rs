#![no_std]

/// Typed error codes for ERC-6909 operations.
///
/// These correspond to revert reasons in the Solidity reference contract,
/// plus an explicit overflow error for checked arithmetic.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    /// Caller tried to approve from the zero address.
    InvalidApprover,
    /// Caller tried to transfer/mint to the zero address.
    InvalidReceiver,
    /// Caller tried to burn/transfer from the zero address.
    InvalidSender,
    /// Caller tried to set operator to the zero address.
    InvalidSpender,
    /// Balance was insufficient for the requested operation.
    InsufficientBalance,
    /// Allowance was insufficient for the requested spend.
    InsufficientAllowance,
    /// An addition or subtraction overflow/underflow was detected.
    ArithmeticOverflow,
}
