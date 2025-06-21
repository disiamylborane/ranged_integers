//! The compile-time infrastructure for Ranged.
//! 
//! The entities inside are public, but are not intended to be used by
//! the end-user, unless the user is implementing a generic logics over Ranged.

use crate::irang;

/// The helper type allowing to restrict the const generic input parameters
/// 
/// Usage example. Allow `myfunction` only for positive values of `SOME_PARAM`:
/// ```
/// #![feature(generic_const_exprs)]
/// #![feature(adt_const_params)]
/// # use ranged_integers::value_check::*;
///
/// const fn myfunction_allowed(some_param: i32) -> OperationPossibility {
///     allow_if(some_param>0)
/// }
///
/// fn myfunction<const SOME_PARAM: i32>()
/// where Assert<{myfunction_allowed(SOME_PARAM)}>: IsAllowed,
/// {
///    // code
/// }
/// ```
pub enum Assert<const COND: OperationPossibility> {}

/// A trait to be used with [`Assert`] type to restrict the const generic input parameters
pub trait IsAllowed {}
impl IsAllowed for Assert<{OperationPossibility::Allowed}> {}

/// Used with the [`Assert`] and [`IsAllowed`] trait to restrict the const generic input parameters
/// 
/// The reason to use this enum instead of just a simple bool is that the error
/// messages generated when the `Assert: IsAllowed` bounds are violated are
/// non-informative when a bool is used.
#[derive(PartialEq, Eq, core::marker::ConstParamTy)]
pub enum OperationPossibility {
    /// `Assert<Forbidden>` does not satisfy `IsAllowed`
    Forbidden,

    /// `Assert<Allowed>` satisfies `IsAllowed`
    Allowed
}

/// Convert bool to [`OperationPossibility`]
#[must_use]
pub const fn allow_if(cond: bool)->OperationPossibility {
    if cond { OperationPossibility::Allowed } else { OperationPossibility::Forbidden }
}

/// The layout selector for [`Ranged`](crate::Ranged) based on bounds
/// 
/// Pick out the "smallest" layout that fits the min..=max range.
/// To be evaluated at compile time. Used with the [`allow_range`] function
#[must_use]
pub const fn memlayout(min: i128, max: i128) -> usize {
    macro_rules! layout_variants {
        ($($t:ident:$n:literal)+) => {
            $(   if $t::MIN as i128 <= min && max <= $t::MAX as i128 {return $n}   )+
        }
    }
    if min > max {
        return 16;  // This will be forbidden by allow_range constraint
    }
    if min == max {return 0;}
    layout_variants! {u8:1 i8:1 u16:2 i16:2 u32:4 i32:4 u64:8 i64:8}
    16  // This will be forbidden by allow_range constraint
}


/// Top-level constraint for [`Ranged`](crate::Ranged) min/max bounds.
/// 
/// The [`Ranged`](crate::Ranged) is constrained to `allow_range(memlayout(MIN, MAX))`,
/// which forbids the integers wider than 8 byte and also bounds the
/// `memlayout(MIN, MAX)` constant, so the compiler is satisfied to use
/// the internal representation of Ranged.
#[must_use]
pub const fn allow_range(sz: usize) -> OperationPossibility {
    allow_if(sz <= 8)
}

/// Constraint of [`crate::Ranged::create_const`] method.
/// 
/// Checks if `v` is in range between `min` and `max`
#[must_use]
pub const fn allow_creation(min: irang, v: irang, max: irang) -> OperationPossibility {
    allow_if(min <= v && v <= max)
}