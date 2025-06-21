//! # Ranged integers [nightly only]
//!
//! The crate provides [an integer type](struct.Ranged.html) restricted to a compile time defined range with
//! automatic data size selection, automatic bounds calculation for arithmetics and the possibility
//! of fixed-size array indexing and range iteration.
//! 
//! The library relies to the unstable const generics-related features.
//! It causes ICEs on some Rust toolchains and may once stop working. 
//! It will require nightly Rust until the `generic-const-exprs` feature is stabilized.
//! Consider the other stable ranged integers crates, allowing to ensure the bounds and
//! perform the arithmetic operations withing the type:
//! - [deranged](https://crates.io/crates/deranged). It provides checked/saturating and unsafe
//!   unchecked arithmetics within the specified bounds, equalities and comparisons. There is a
//!   procedural macro to select a type of ranged based on literal bounds.
//! - [light_ranged_integers](https://crates.io/crates/light-ranged-integers) provides
//!   a nice way to choose the type of arithmetic operation (wrapping/saturating/panicking)
//!   with the type-level mark.
//! 
//! # Version info
//! 
//! The version 0.10.0 was built for nightly-2025-06-19 toolchain.
//!
//! The const arithmetics were added. The const fn-s are used for compile-time arithmetics, since
//! the `std::opts` traits are mostly non-const. The `i128` representation was removed from Ranged.
//! This has been done because the bounds violation generates an unpleasant error in which the
//! place of occurrence is not specified, only the violated line inside `ranged_integers` crate. So,
//! the possibility to violate the `i128` bounds during the automatic bounds calculation is eliminated.
//! The bounds of i64/u64 are explicitly written as a trait constraint, so it generates a
//! much more helpful error message.
//!
//! # Prerequisites
//!
//! The library usage requires the following Rust features enabled in the user crate or application:
//!
//! ```
//! // Without this rustc generates errors and sometimes panics.
//! #![feature(adt_const_params, generic_const_exprs)]
//! ```
//! 
//! Note that the features needed depend on the package version.
//! 
//! # Usage and examples
//! 
//! ## Ranged semantics
//! 
//! Use `Ranged<MIN, MAX>` type to be sure of the value range:
//!
//! ```
//! # #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
//! fn move_player(dice_roll: Ranged<1, 6>) {  // Ranged<1, 6> is automatically of 1 byte
//!     let x : i32 = dice_roll.into(); // Conversion is allowed, i32 can store 1..=6
//! }
//! ```
//!
//! ## Contents
//! 
//! * [Data layout paradigm](#data-layout-paradigm)
//! * [Ranged and integer primitives](#ranged-and-integer-primitives)
//!    - [Creation of Ranged at compile time](#creation-of-ranged-at-compile-time)
//!    - [Ranged -> Ranged conversion](#ranged---ranged-conversion)
//!    - [int -> Ranged conversion](#int---ranged-conversion)
//!    - [Ranged -> int conversion](#ranged---int-conversion)
//! * [Array indexing, slicing and iteration](#array-indexing-slicing-and-iteration)
//! * [Comparison](#comparison)
//! * [Arithmetics](#arithmetics)
//! * [Pattern matching](#pattern-matching)
//! 
//! ## Data layout paradigm
//!
//! The [Ranged] automatically chooses the smallest size possible according to `MIN..=MAX` range.
//! It supports i8, u8, i16, u16, i32, u32, i64 and u64 layouts (i128 and u128 are not supported),
//! and a special zero-size layout for "constant" values with `MIN==MAX`.
//!
//! ```
//! # #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*; fn main(){
//! use core::mem::size_of;
//! assert_eq!(size_of::< Ranged<42, 42> >(), 0); // It's always 42, no need to store it
//! 
//! assert_eq!(size_of::< Ranged<-1, 127> >(), 1); // Fits i8
//! assert_eq!(size_of::< Ranged<0,  200> >(), 1); // Fits u8
//! assert_eq!(size_of::< Ranged<-1, 200> >(), 2); // Fits i16, doesn't fit i8 or u8
//! 
//! assert_eq!(size_of::< Ranged<0, 90000> >(),  4); // The range fits i32
//! # }
//! ```
//!
//! The implementation heavily relies on the optimizer.
//!
//! ## Ranged and integer primitives
//!
//! ### Creation of `Ranged` at compile time
//!
//! The [`Ranged::create_const`] can be used to create a 
//! [`Ranged`] value checking it at compile time.
//! The macro [`r!`] does the same but a bit shorter.
//!
//! ```
//! # #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! // Way 1: specify the bounds explicitly
//! move_player(Ranged::<1,6>::create_const::<4>());
//! move_player(r!([1 6] 4));  // Same thing
//!
//! // Way 2: do not specify the bounds when possible
//! move_player(Ranged::create_const::<4>());
//! move_player(r!([] 4));  // Same thing
//! let x: Ranged::<0, 100> = Ranged::create_const::<42>();
//! let y: Ranged::<0, 100> = r!([] 42);  // Same thing
//!
//! // Way 3: a special case with the single possible value
//! let x = Ranged::<4, 4>::create_const::<4>();
//! let y = r!(4);  // Same thing
//! ```
//!
//! It fails if the bounds are corrupted:
//!
//! ```compile_fail
//! # #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! move_player(r!([] 7)); // Error: Can't store 7 in [1 6] interval
//! ```
//! ```compile_fail
//! move_player(r!([1 7] 7)); // Error: type mismatch, move_player() requires Ranged<1, 6>
//! ```
//!
//! ### `Ranged` -> `Ranged` conversion
//!
//! The `Ranged` can be converted to the type with different bounds using 
//! [`expand()`](struct.Ranged.html#method.expand) generic method (compile-time check)
//! and the methods [`fit()`](struct.Ranged.html#method.fit), [`fit_min()`](struct.Ranged.html#method.fit_min),
//! [`fit_max()`](struct.Ranged.html#method.fit_max) for runtime check. The `Ranged` values may be compared
//! to each other yielding the shrunk bounds using 
//! [`fit_less_than()`](struct.Ranged.html#method.fit_less_than),
//! [`fit_less_eq()`](struct.Ranged.html#method.fit_less_eq),
//! [`fit_greater_than()`](struct.Ranged.html#method.fit_greater_than), and 
//! [`fit_greater_eq()`](struct.Ranged.html#method.fit_greater_eq) methods.
//!
//! ```
//! # #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! let expandable: Ranged<4, 5> = r!([] 5);  // Fits Ranged<1,6> accepted by move_player
//! let overlapping: Ranged<4, 9> = r!([] 5);  // Doesn't fit, but the value 5 is acceptable
//! move_player(expandable.expand());
//! move_player(overlapping.fit().unwrap());
//! ```
//!
//! Shrinking with `expand()` is forbidden:
//!
//! ```compile_fail
//! # #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! # let overlapping: Ranged<4, 9> = r!([] 5);
//! move_player(overlapping.expand());  // Error: the bounds 4..=9 can't fit in 1..=6
//! ```
//!
//! ### `int` -> `Ranged` conversion
//!
//! Way 1: ensure the bounds with [`Ranged::new(i128) -> Option<Ranged>`](struct.Ranged.html#method.new) function.
//! 
//! **Note**: due to incomplete features limitations, the function may fail to compile with
//! the 'trait bound is not satisfied' if the bounds are not explicitly specified.
//!
//! ```
//! # #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! let some_int = 4;
//! let some_wrong_int = 8;
//! assert!(Ranged::<0, 6>::new(some_int) == Some(r!([0 6] 4)));
//! assert!(Ranged::<0, 6>::new(some_wrong_int) == None);
//!
//! move_player(Ranged::new(some_int).unwrap());  // this call may fail to compile
//!                                               // use Ranged::<0, 6>::new instead
//! ```
//!
//! Way 2: use the [`Remainder operation`](struct.Ranged.html#impl-Rem<Ranged<VAL%2C%20VAL>>) with the "const" divisor
//!
//! ```
//! # #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! let x: Ranged<-9, 9> = 15_i32 % r!(10);
//! let y: Ranged<0, 9> = 15_u32 % r!(10);
//! assert!(x == r!(5));
//! assert!(y == r!(5));
//! ```
//!
//! Way 3: Convert the primitive types to `Ranged` with their native bounds using [`AsRanged`]
//!
//! ```
//! # #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! use ranged_integers::AsRanged;
//! let x = 15_u8.as_ranged();  // Ranged<0, 255>
//! let y = 15_i16.as_ranged(); // Ranged<-32768, 32767>
//! ```
//!
//! ### `Ranged` -> `int` conversion
//!
//! `int::From` trait is implemented when the value is proved to
//! fit into the result type:
//!
//! ```
//! # #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
//! let x = r!([0 200] 20);
//! let y: u8 = x.into();  // 0..=200 fits u8
//! ```
//!
//! ```compile_fail
//! # #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
//! let x = r!([0 200] 20);
//! let y: i8 = x.into();  // 0..=200 doesn't fit i8
//! ```
//!
//! `From` and `Into` operations can't be used in const context.
//! A set of [`const fn`](struct.Ranged.html#method.i8)s allows const conversions to 
//! any integer primitive except for `u128`:
//!
//! ```
//! # #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! let x = r!([0 200] 20);
//! let y = x.u8(); // y is u8
//! let z = x.i16(); // z is i16
//! let w = x.usize(); // w is usize
//! ```
//!
//! ```compile_fail
//! # #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! let x = r!([0 200] 20);
//! let err = x.i8();  // Error: 0..=200 doesn't fit i8
//! ```
//!
//! ## Array indexing, slicing and iteration
//!
//! The [`ConstInclusiveRange<MIN,MAX>`] zero-size type is a range `MIN..=MAX`
//! capable to create the iterator ([`IntoIterator`] trait implemented)
//! with `Ranged<MIN, MAX>` output type. The [`r!`] macro can be used instead.
//!
//! The [`Ranged::iter_up`](struct.Ranged.html#method.iter_up) method creates an
//! iterator from the current value up to `MAX`.
//!
//! The arrays `[T; N]` may be indexed with `Ranged<0, {N-1}>` and sliced
//! with `r!(MIN..END)` range with a reference to fixed-size array output.
//!
//! ```
//! # #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! let arr = [r!([1 6] 2), r!([] 3), r!([] 4), r!([] 5)];
//!
//! assert_eq!(arr[r!(1..3)], [3,4]);  // Slicing with array reference output
//! assert_eq!(arr[r!([0 3] 1)], 3);  // Slicing with array reference output
//!
//! // Not recommended to use this:
//!     for i in ConstInclusiveRange::<0, 3> {
//!         move_player(arr[i])  // iters through 0,1,2,3
//!     }
//! for i in r!(0..4) {
//!     move_player(arr[i])  // iters through 0,1,2,3
//! }
//! for i in r!(0..=3) {
//!     move_player(arr[i])  // iters through 0,1,2,3
//! }
//! for mv in r!([1 6] 3).iter_up() {
//!     move_player(mv)  // calls with 3,4,5,6
//! }
//! ```
//!
//! ## Comparison
//!
//! All `Eq` and `Ord` operations between different Ranged types are allowed,
//! so as `Ranged` vs integer comparisons:
//!
//! ```
//! # #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! assert!(r!([1 6] 4) == r!([1 10] 4));
//! assert!(r!([1 6] 4) != r!([1 6] 5));
//! assert!(r!(4) == 4);
//! assert!(5 != r!([1 6] 4));
//!
//! assert!(r!(5) > r!([1 6] 4));
//! assert!(4 < r!([1 6] 5));
//! ```
//! 
//! To constrain the output type ruled by comparison, one may use [`Ranged::fit_less_than`]
//! function and its siblings [`Ranged::fit_less_eq`], [`Ranged::fit_greater_than`], and [`Ranged::fit_greater_eq`].
//!
//! ## Arithmetics
//!
//! The bounds of arithmetic operations results are automatically recalculated.
//!
//! Currently supported:
//! * The basic arithmetic operations (+, -, *, /), not available in const context
//! * The const arithmetic functions ([`add`](Ranged::add), [`sub`](Ranged::sub), [`mul`](Ranged::mul), [`div`](Ranged::div))
//! * [`div_euclid`](Ranged::div_euclid) and [`rem_euclid`](Ranged::div_euclid)
//! * [`min`](Ranged::min) and [`max`](Ranged::max)
//! * [`abs`](Ranged::abs) and [`neg`](Ranged::neg)
//!
//! ```
//! # #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! let x = r!([1 6] 5);
//! let y = r!([1 6] 4);
//!
//! let a = x + y;  // The minimum is (1+1)=2, the maximum is (6+6)=12
//! let check_add: Ranged<2, 12> = a;  // Range assertion assignment
//! assert_eq!(check_add, r!(9));
//!
//! let s = x - y;  // The minimum is (1-6)=-5, the maximum is (6-1)=5
//! let check_sub: Ranged<-5, 5> = s;  // Range assertion assignment
//! assert_eq!(check_sub, r!(1));
//!
//! let m = x * y;  // The minimum is (1*1)=1, the maximum is (6*6)=36
//! let check_mul: Ranged<1, 36> = m;  // Range assertion assignment
//! assert_eq!(check_mul, r!(20));
//!
//! let d = x / y;  // The minimum is (1/6)=0, the maximum is (6/1)=6
//! let check_div: Ranged<0, 6> = d;  // Range assertion assignment
//! assert_eq!(check_div, r!(1));
//!
//! let r = x % y;
//! let check_rem: Ranged<0, 5> = r;  // Range assertion assignment
//! assert_eq!(check_rem, r!(1));
//!
//! let n = -x;
//! let check_neg: Ranged<-6, -1> = n;  // Range assertion assignment
//! assert_eq!(check_neg, r!(-5));
//! 
//! let min: Ranged<1,6> = x.min(a);
//! let max: Ranged<2,12> = x.max(a);
//! let abs: Ranged<0,6> = r!([-1 6] -1).abs();
//! let neg: Ranged<-6,1> = r!([-1 6] -1).neg();
//! ```
//!
//! The division and remainder are allowed only if it's impossible to store "0" in the divisor:
//!
//! ```compile_fail
//! # #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! let x = r!([1 6] 4);
//! let y = r!([0 6] 5);
//! let z = r!([-1 6] 5);
//!
//! let d = x / y; // Error: y can be 0
//! let e = x % z; // Error: z can be 0
//! ```
//!
//! The true bounds calculation routine for `Rem` operation is far too complex.
//! In this library the calculated bounds will never exceed `1-DMAXABS..=DMAXABS-1` where `DMAXABS` is the 
//! maximum of the divisor absolute value.
//!
//! This kind of `Rem` followed by `expand` is available for any dividend:
//! ```
//! # #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
//! let x = r!([-1000 1000] 500);
//! let y = r!([-1 1000] 500);
//! let d = r!([1 10] 7);
//!
//! let r: Ranged<-9, 9> = (x%d).expand();
//! // In this case, it expands just from Ranged<-9, 9> to itself
//!
//! let r: Ranged<-9, 9> = (y%d).expand();
//! // In this case, it expands from Ranged<-1, 9>
//! ```
//!
//! But the actual calculation routine can produce smaller bounds:
//!
//! ```
//! # #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! // In the general case the output is Ranged<1-MAX, MAX-1>, MAX from divisor (by absolute value)
//! let x: Ranged<-9, 9> = (r!([-1000 1000] 500) % r!([1 10] 7));
//! let x: Ranged<-9, 9> = (r!([-1000 1000] 500) % r!([-10 -1] -7));
//! 
//! // If the dividend is nonnegative or nonpositive,
//! // the output range is limited to 0 from one side.
//! let x: Ranged<0, 9> = r!([0 100] 15) % r!(10);
//! let x: Ranged<-9, 0> = r!([-100 0] -15) % r!(10);
//!
//! // The limit can't exceed the dividend's MIN(if negative) or MAX(if positive):
//! let x: Ranged<-10, 10> = r!([-10 10] 4) % r!([1 1000] 70);
//!
//! // If the divisor is "constant", the output bounds are the true bounds:
//! let x: Ranged<4, 7> = r!([14 17] 15) % r!(10);
//!
//! // In particular, if both operands are "constant", the result is "constant"
//! let x: Ranged<5, 5> = r!(15) % r!(10);
//! ```
//!
//! Following these rules, the calculated bounds may be wider than the true ones, like 
//! `Ranged<36289, 36292> % Ranged<6, 9> = Ranged<0, 8>` while the
//! result never exceeds `Ranged<1, 4>`.
//!
//! ## Pattern matching
//! 
//! A limited version is implemented with [`rmatch!`] macro.
//! 


#![no_std]
#![allow(incomplete_features)]

#![feature(generic_const_exprs)]  // The whole idea is based on this
#![feature(adt_const_params)]  // This is mostly for OperationPossibility
#![feature(specialization)]  // It allows to decrease a number of constraints in Ranged
#![feature(freeze)]  // If it is enabled in user crate, the Ranged should implement Freeze trait

#![deny(missing_docs)]
#![deny(clippy::nursery)]
#![warn(clippy::pedantic)]


// An alias integer representing the public interface of Ranged constants. Introduced
// to easily change when necessary.
#[allow(non_camel_case_types)]
type irang = i128;


#[cfg(test)] #[macro_use] extern crate std;
#[cfg(test)] mod tests;

pub mod value_check;  // Compile-time infrastructure for Ranged

mod holder;  // Internal representation of ranged
mod conversions;  // Conversions to/from Ranged
mod arithmetics;  // Arithmetic operations
mod iter;  // Iterating over a constant range
mod arrays;  // Implementing Ranged-related logics for arrays indexing and slicing

pub use conversions::AsRanged;
pub use iter::ConstInclusiveRange;

use value_check::{Assert, IsAllowed, OperationPossibility, memlayout, allow_range, allow_if, allow_creation};

/// A value restricted to the given bounds
#[repr(transparent)]
#[derive(Clone, Copy, Hash)]
pub struct Ranged<const MIN: irang, const MAX: irang>
where
    // This restriction kills two birds with one stone: first, it forbids to
    // work with the Ranged beyond u64 or i64 layout, second, it bounds the
    // memlayout(MIN, MAX) constant, so it can be used in the inner type. All
    // the generic calls to Ranged<...> have to satisfy this constraint.
    Assert<{allow_range(memlayout(MIN, MAX))}>: IsAllowed,
{
    v: holder::RangedRepr<{ memlayout(MIN, MAX) }>,
}


impl<const MIN: irang, const MAX: irang> Ranged<MIN, MAX>
where
    Assert<{allow_range(memlayout(MIN, MAX))}>: IsAllowed,
{
    #[allow(clippy::inline_always)] #[must_use] #[inline(always)]
    const unsafe fn unchecked_new(n: irang) -> Self {
        Self {
            v: holder::RangedRepr::from_irang(n),
        }
    }

    // Convert Ranged to a primitive
    #[allow(clippy::inline_always)] #[must_use] #[inline(always)]
    const fn get(self) -> irang {
        if MIN == MAX {MIN}
        else {self.v.to_irang(const{MIN>=0})}
    }

    /// Create a Ranged value checking the bounds at runtime
    /// 
    /// **Note**: due to incomplete features limitations, the function may fail to compile with
    /// the 'trait bound is not satisfied' if the bounds are not explicitly specified.
    /// 
    /// # Example
    /// 
    /// ```
    /// # #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*; 
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let input = "42".to_string();
    /// let user_input = input.parse()?;
    /// if let Some(input) = Ranged::<1, 100>::new(user_input){
    ///     println!("The value is in range 1..=100")
    /// }
    /// else {
    ///     println!("The value is too high :(")
    /// }
    /// # Ok(()) }
    /// ```
    #[must_use]
    pub const fn new(n: irang) -> Option<Self> {
        if (MIN <= n) && (n <= MAX) {
            Some(unsafe { Self::unchecked_new(n) })
        } else {
            None
        }
    }
}

impl<const MIN: irang, const MAX: irang> Ranged<MIN, MAX>
where
    Assert<{allow_range(memlayout(MIN, MAX))}>: IsAllowed,
{
    /// Create a Ranged constant checking the bounds at compile time
    /// 
    /// Consider using [`r!`] macro instead
    /// 
    /// # Example
    /// 
    /// ```
    /// # #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*; 
    /// let a = Ranged::<0, 100>::create_const::<42>();
    /// let a = r!([0 100] 42);
    /// ```
    #[must_use]
    pub const fn create_const<const V: irang>() -> Self
    where Assert<{ allow_creation(MIN, V, MAX) }>: IsAllowed,
    {
        unsafe { Self::unchecked_new(V) }
    }

    /// Iterate up from current value to `Self::MAX` (inclusively) using `Self` as output
    #[must_use]
    pub const fn iter_up(self) -> iter::Iter<MIN, MAX> {
        iter::Iter::<MIN,MAX>{current: Some(self)}
    }
}

/// Create a ranged value or a range at compile time
///
/// **Warning**: ensure `#![feature(adt_const_params)]` is enabled.
///
/// # Example
///
/// ```
/// # #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
/// // Explicit bounds:
/// let a = r!([0 42] 23);  // Ranged<0, 42> with a value 23
/// // Type inference:
/// let b: Ranged<0, 100> = r!([] 42);  // Ranged<0, 100> with a value 42
/// // "Constant" value:
/// let c = r!(10);  // Zero-sized Ranged<10, 10> with a value 10
/// //Range:
/// for i in r!(0..10){
///     let v: Ranged<0,9> = i; 
/// }
/// ```
#[macro_export]
macro_rules! r {
    ([$min:literal $max:literal] $x:expr) => {
        $crate::Ranged::<$min, $max>::create_const::<$x>()
    };
    ([] $v:expr) => {
        $crate::Ranged::create_const::<$v>()
    };
    ($min:tt..$end:tt) => {
        $crate::ConstInclusiveRange::<{$min}, {$end-1}>
    };
    (-$min:tt..-$end:tt) => {
        $crate::ConstInclusiveRange::<{-$min}, {-$end-1}>
    };
    (-$min:tt..$end:tt) => {
        $crate::ConstInclusiveRange::<{-$min}, {$end-1}>
    };
    (-$min:tt..=$max:tt) => {
        $crate::ConstInclusiveRange::<{-$min}, {$max}>
    };
    (-$min:tt..=-$max:tt) => {
        $crate::ConstInclusiveRange::<{-$min}, {-$max}>
    };
    ($min:tt..=$max:tt) => {
        $crate::ConstInclusiveRange::<{$min}, {$max}>
    };
    ($v:literal) => {
        $crate::Ranged::<$v, $v>::create_const::<$v>()
    };
    ($v:tt) => {
        $crate::Ranged::<$v, $v>::create_const::<{$v}>()
    };
}

impl<const MIN: irang, const MAX: irang> core::fmt::Display for Ranged<MIN, MAX>
where Assert<{allow_range(memlayout(MIN, MAX))}>: IsAllowed,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl<const MIN: irang, const MAX: irang> core::fmt::Debug for Ranged<MIN, MAX>
where Assert<{allow_range(memlayout(MIN, MAX))}>: IsAllowed,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if MIN == MAX {
            write!(f, "r!({MIN})")
        } else {
            write!(f, "r!([{MIN} {MAX}] {})", self.get())
        }
    }
}


/// Ranged pattern matching macro
/// 
/// Allows to match a [`Ranged`] value over a range it covers. The feature has
/// the following limitations:
/// - The bounds must be explicitly specified; they are checked, but not inferred
/// - The macro syntax supports a subset of Rust pattern matching syntax
/// - `i128::MIN` and `i128::MAX` values must not be in range
/// - The unclear error reporting
/// 
/// ```
/// # #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
/// fn ranged_to_bool(r: Ranged<0,1>) -> bool {
///     rmatch!{[0 1] r  // Bounds and expression (token tree, 
///                      // complex expressions must be in parentheses)
///         0 => {false}
///         1 => {true}  // Complex patterns like 1..5 | 12..14 are supported
///     }
/// }
/// ```
#[macro_export]
macro_rules! rmatch {
    ([$min:literal $max:literal] $val:tt
        $( 
            $p:pat => { $e:expr }
        )*
    ) => {
        {
            #[allow(renamed_and_removed_lints)]
            #[deny(const_err)]
            const _MINF: i128 = $min - 1;
            #[allow(renamed_and_removed_lints)]
            #[deny(const_err)]
            const _PINF: i128 = $max + 1;
            let _v: Ranged<$min, $max> = $val;
            match _v.i128() {
                i128::MIN..=_MINF => unsafe {core::hint::unreachable_unchecked()}
                _PINF..=i128::MAX => unsafe {core::hint::unreachable_unchecked()}
                $( $p => { $e } )*
            }
        }
    };
}


// Failtests: the tests that should fail or generate an error.


#[allow(dead_code)]
#[doc(hidden)]
/**
```
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
u8::from(r!(0));
```

```compile_fail
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
u8::from(r!(-1));
```

```
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
u8::from(r!(255));
```

```compile_fail
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
u8::from(r!(256));
```

```
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
i8::from(r!(-128));
```


```compile_fail
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
i8::from(r!(-129));
```

```
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
i8::from(r!(127));
```


```compile_fail
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
i8::from(r!(128));
```


```
# #![feature(adt_const_params, generic_const_exprs)]
# #[macro_use] extern crate ranged_integers; use ranged_integers::*;
let a = r![[100 1000] 500] / r![[1 6] 5];
```
```compile_fail
# #![feature(adt_const_params, generic_const_exprs)]
# #[macro_use] extern crate ranged_integers; use ranged_integers::*;
let a = r![[100 1000] 500] / r![[0 6] 5];
```
```compile_fail
# #![feature(adt_const_params, generic_const_exprs)]
# #[macro_use] extern crate ranged_integers; use ranged_integers::*;
let a = r![[100 1000] 500] / r![[-1 6] 5];
```

```
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
Ranged::<0,1>::new(1);
```


```compile_fail
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
Ranged::<1,0>::new(1);
```


```
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
let x: Ranged::<0,1> = Ranged::<0,1>::new(1).unwrap();
```


```
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
let x: Ranged::<0,1> = Ranged::<0,1>::new(1).unwrap();
```


```
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
let a = r!([-10 10] 0);
rmatch!{[-10 10] a
    _ => {()}
}
```

```compile_fail
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
let a = r!([-170141183460469231731687303715884105728 10] 0);
rmatch!{[-170141183460469231731687303715884105728 10] a
    _ => {()}
}
```

```compile_fail
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
let a = r!([-10 170141183460469231731687303715884105727] 0);
rmatch!{[-10 170141183460469231731687303715884105727] a
    _ => {()}
}
```

```
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
assert!(  r!([10 50] 30).fit_less_than(r!([10 45] 40)) == Some(r!([10 45] 30))  );
```

```
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
assert_eq!(r!([10 50] 39).fit_less_than( r!([0 45] 40) ), Some(r!([10 45] 39)));
```

```
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
assert_eq!(r!([10 50] 39).fit_less_than( r!([30 45] 40) ), Some(r!([10 45] 39)));
```

```
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
assert_eq!(r!([10 50] 40).fit_less_than( r!([30 45] 40) ), None);
```

```compile_fail
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
assert_eq!(r!([10 40] 39).fit_less_than( r!([30 45] 40) ), Some(r!([10 45] 39)));
```

```compile_fail
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
assert_eq!(r!([45 50] 47).fit_less_than( r!([0 45] 40) ), None);
```

```compile_fail
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
assert_eq!(r!([10 20] 10).fit_less_than( r!([30 45] 40) ), Some(r!([10 45] 10)));
```



```
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
assert!(  r!([10 50] 40).fit_less_eq(r!([10 45] 40)) == Some(r!([10 45] 40))  );
```

```
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
assert_eq!(r!([10 50] 40).fit_less_eq( r!([0 45] 40) ), Some(r!([10 45] 40)));
```

```
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
assert_eq!(r!([10 50] 40).fit_less_eq( r!([30 45] 40) ), Some(r!([10 45] 40)));
```

```
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
assert_eq!(r!([10 50] 40).fit_less_eq( r!([40 45] 40) ), Some(r!([10 45] 40)));
```

```
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
assert_eq!(r!([10 50] 41).fit_less_eq( r!([30 45] 40) ), None);
```

```compile_fail
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
assert!(  r!([10 40] 40).fit_less_eq(r!([40 45] 40)) == Some(r!([10 40] 40))  );
```
```
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
assert!(  r!([10 50] 10).fit_less_eq(r!([5 10] 10)) == Some(r!([10 10] 10))  );
```


```
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
assert!(  r!([10 50] 40).fit_greater_than(r!([20 40] 39)) == Some(r!([20 50] 40))  );
```

```
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
assert!(  r!([10 50] 40).fit_greater_than(r!([20 50] 39)) == Some(r!([20 50] 40))  );
```

```
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
assert!(  r!([10 50] 40).fit_greater_than(r!([20 100] 39)) == Some(r!([20 50] 40))  );
```

```
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
assert_eq!(r!([10 50] 40).fit_greater_than( r!([30 45] 40) ), None);
```

```compile_fail
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
assert!(  r!([10 50] 40).fit_greater_than(r!([10 40] 39)) == Some(r!([10 50] 40))  );
```

```compile_fail
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
assert!(  r!([10 50] 50).fit_greater_than(r!([50 55] 50)) == Some(r!([50 50] 50))  );
```

```compile_fail
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
assert!(  r!([10 50] 40).fit_greater_than(r!([0 10] 5)) == Some(r!([10 50] 40))  );
```


```
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
assert!(  r!([10 50] 40).fit_greater_eq(r!([20 40] 40)) == Some(r!([20 50] 40))  );
```

```
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
assert!(  r!([10 50] 40).fit_greater_eq(r!([20 50] 40)) == Some(r!([20 50] 40))  );
```

```
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
assert!(  r!([10 50] 40).fit_greater_eq(r!([20 100] 40)) == Some(r!([20 50] 40))  );
```

```
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
assert_eq!(r!([10 50] 40).fit_greater_eq( r!([30 45] 41) ), None);
```

```compile_fail
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
assert!(  r!([10 50] 40).fit_greater_eq(r!([10 40] 39)) == Some(r!([10 50] 40))  );
```

```
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
assert!(  r!([10 50] 50).fit_greater_eq(r!([50 55] 50)) == Some(r!([50 50] 50))  );
```

```compile_fail
# #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*;
assert!(  r!([10 50] 40).fit_greater_eq(r!([0 10] 5)) == Some(r!([10 50] 40))  );
```



*/
struct Failtests;


// Compile-time test for sizes and alignments of Ranged
#[allow(dead_code)]
const SIZE_ALIGN_CHECK: () = {
    use core::mem::{align_of, size_of};

    macro_rules! sz_align {
        ($sz:ty, $t:ty) => {
            assert!(size_of::<$t>() == size_of::<$sz>());
            assert!(align_of::<$t>() == align_of::<$sz>());
        };
    }

    sz_align!((), Ranged<0,0>);
    sz_align!((), Ranged<1,1>);
    sz_align!((), Ranged<-1,-1>);
    sz_align!((), Ranged<100, 100>);
    sz_align!((), Ranged<-100, -100>);
    sz_align!((), Ranged<500, 500>);
    sz_align!((), Ranged<-500, -500>);
    sz_align!((), Ranged<100_000, 100_000>);
    sz_align!((), Ranged<-100_000, -100_000>);
    sz_align!((), Ranged<10_000_000_000, 10_000_000_000>);
    sz_align!((), Ranged<-10_000_000_000, -10_000_000_000>);

    sz_align!((), Ranged<-32768, -32768>);
    sz_align!((), Ranged<32767, 32767>);
    sz_align!((), Ranged<65535, 65535>);
    sz_align!((), Ranged<65536, 65536>);

    sz_align!(i8, Ranged<10,11>);
    sz_align!(i8, Ranged<254,255>);
    sz_align!(i8, Ranged<126,127>);
    sz_align!(i8, Ranged<-128, -127>);
    sz_align!(i8, Ranged<0,10>);
    sz_align!(i8, Ranged<0,127>);
    sz_align!(i8, Ranged<0,255>);
    sz_align!(i8, Ranged<127,255>);
    sz_align!(i8, Ranged<-128, 127>);

    sz_align!(i16, Ranged<-128, 128>);

    sz_align!(i16, Ranged<-32768, 32767>);
    sz_align!(i16, Ranged<0, 32768>);
    sz_align!(i16, Ranged<0, 65535>);
    sz_align!(i16, Ranged<-32768, -32767>);
    sz_align!(i16, Ranged<32766, 32767>);
    sz_align!(i16, Ranged<65534, 65535>);

    sz_align!(i32, Ranged<-32768, 32768>);
    sz_align!(i32, Ranged<0, 65536>);

    sz_align!(i32, Ranged<0, 4_294_967_295>);
    sz_align!(i32, Ranged<-2_147_483_648, 2_147_483_647>);
    sz_align!(i32, Ranged<100, 10_000_000>);
    sz_align!(i32, Ranged<-100, 10_000_000>);
    sz_align!(i32, Ranged<100, 2_147_483_647>);
    sz_align!(i32, Ranged<-100, 2_147_483_647>);

    sz_align!(i64, Ranged<-1, 4_294_967_295>);
    sz_align!(i64, Ranged<0, 4_294_967_296>);
    sz_align!(i64, Ranged<-2_147_483_649, 2_147_483_647>);
    sz_align!(i64, Ranged<-2_147_483_648, 2_147_483_648>);

    sz_align!(i64, Ranged<0, 18_446_744_073_709_551_615>);
    sz_align!(i64, Ranged<-9_223_372_036_854_775_808, 9_223_372_036_854_775_807>);
};
