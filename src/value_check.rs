//! The module facilitates the compile-time value checking in `where` clauses.
//!
//! Usage example. Allow `myfunction` only for positive values of SOME_PARAM:
//! ```
//! #![feature(const_evaluatable_checked)]
//! #![feature(const_generics)]
//!
//! # use ranged_integers::value_check::*;
//!
//! const fn myfunction_allowed(some_param: i32) -> OperationPossibility {
//!     OperationPossibility::allow_if(some_param>0)
//! }
//!
//! fn myfunction<const SOME_PARAM: i32>()
//! where
//!    Assert<{myfunction_allowed(SOME_PARAM)}>: IsAllowed,
//! {
//!    // code
//! }
//! ```
//!

#[doc(hidden)] pub enum Assert<const COND: OperationPossibility> {}
#[doc(hidden)] pub trait IsAllowed {}
impl IsAllowed for Assert<{OperationPossibility::Allowed}> {}

// This enum is used instead of `bool` for better compile error handling
#[doc(hidden)]
#[derive(PartialEq, Eq)]
pub enum OperationPossibility {
    Forbidden,
    Allowed
}

impl OperationPossibility {
    #[doc(hidden)] pub const fn allow_if(cond: bool)->Self {
        match cond {
            true => Self::Allowed,
            false => Self::Forbidden
        }
    }
}
