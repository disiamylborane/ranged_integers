//! # Ranged integers [nightly only]
//! 
//! **Note: WIP, the API may change.**
//! 
//! The crate provides an integer type restricted to a compile time defined range.
//! Auto size and compile time checked arithmetics are included.
//! 
//! [`Ranged<const MIN: i128, const MAX: i128>`](struct.Ranged.html) is bounded to [MIN, MAX] interval **inclusively**.
//! 
//! # Usage and examples
//! 
//! ## Prerequisites
//! 
//! The library's macro [`r!`](macro.ranged.html) requires the following Rust features enabled:
//! 
//! ```
//! // Without this the Ranged usage now fails with some unclear 
//! // "trait bound is not satisfied" errors:
//! #![feature(const_generics)] 
//! #![feature(const_evaluatable_checked)]
//!
//! // This is needed for r! macro usage:
//! #![feature(const_panic)]
//! ```
//! 
//! ## Ranged semantics
//! 
//! Use `Ranged<MIN, MAX>` type to be sure of the parameter range:
//! 
//! ```
//! # extern crate ranged_integers; use ranged_integers::*;
//! fn move_player(dice_roll: Ranged<1, 6>) {
//!     let x : i32 = dice_roll.into(); // Convert to int
//! }
//! ```
//! 
//! ## Compile time Ranged creation
//! 
//! The macro `r!([MIN MAX] VALUE)` creates the const Ranged:
//! 
//! ```
//! # #![feature(const_panic)] #![feature(const_generics)] #![feature(const_evaluatable_checked)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! # fn move_player(dice_roll: Ranged<1, 6>) {
//! #     let x : i32 = dice_roll.into(); // Convert back to int
//! # }
//! move_player(r!([1 6] 4));
//! ```
//! 
//! It fails if the bounds are corrupted:
//! 
//! ```compile_fail
//! # #![feature(const_panic)] #![feature(const_generics)] #![feature(const_evaluatable_checked)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! # fn move_player(dice_roll: Ranged<1, 6>) {
//! #     let x : i32 = dice_roll.into(); // Convert back to int
//! # }
//! move_player(r!([1 6] 7)); // Error: Can't store 7 in [1 6] inverval
//! move_player(r!([1 7] 7)); // Error: type mismatch, move_player() requires Ranged<1, 6>
//! ```
//! 
//! A special case with the single possible value:
//! 
//! ```
//! # #![feature(const_panic)] #![feature(const_generics)] #![feature(const_evaluatable_checked)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! let x = r![4]; // Means Ranged<4, 4> with the value 4
//! let y: Ranged<4,4> = x;
//! ```
//! 
//! ## Casting to Ranged at runtime
//! 
//! ### Way 1: ensure the bounds with `new(int)->Option` method
//! 
//! ```
//! # #![feature(const_panic)] #![feature(const_generics)] #![feature(const_evaluatable_checked)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! # fn move_player(dice_roll: Ranged<1, 6>) {
//! #     let x : i32 = dice_roll.into(); // Convert back to int
//! # }
//! let some_i32 = 4;
//! let some_wrong_i32 = 8;
//! assert!(Ranged::<0, 6>::new(some_i32).unwrap() == r![4]);
//! assert!(Ranged::<0, 6>::new(some_wrong_i32) == None);
//!
//! move_player(Ranged::new(4).unwrap());
//! ```
//! 
//! ### Way 2: use the remainder operation with the "const" divisor
//! 
//! ```
//! # #![feature(const_panic)] #![feature(const_generics)] #![feature(const_evaluatable_checked)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! let x = 15 % r![10];
//! let y: Ranged<0, 9> = x;
//! assert!(y == r![5]); // 15 % 10 == 5
//! ```
//! 
//! ```compile_fail
//! # #![feature(const_panic)] #![feature(const_generics)] #![feature(const_evaluatable_checked)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! let x = 15 % r![10];
//! let y: Ranged<0, 20> = x;  // Error: x is Ranged<0, 9>
//! ```
//! 
//! ### Way 3: Convert the primitive types to `Ranged` with their native bounds
//! 
//! ```
//! # #![feature(const_panic)] #![feature(const_generics)] #![feature(const_evaluatable_checked)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! let x = 15_u8.as_ranged(); // Ranged<0, 255>
//!                            // Trait AsRanged must be in scope
//! assert!(x == r![15]);
//! ```
//! 
//! ## Bounds expansion
//! 
//! Expand the bounds freely if needed:
//! 
//! ```
//! # #![feature(const_panic)] #![feature(const_generics)] #![feature(const_evaluatable_checked)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! let x = r!([0 100] 20);
//! let y : Ranged<-5, 200> = x.expand(); // From [0 100] to [-5 200]
//! let z = x.expand::<-5, 200>(); // Also [0 100] -> [-5 200]
//! ```
//! 
//! Shrinking is not allowed:
//! 
//! ```compile_fail
//! # #![feature(const_panic)] #![feature(const_generics)] #![feature(const_evaluatable_checked)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! let x = r!([0 100] 20);
//! let y : Ranged<1, 200> = x.expand(); // Error: x can be 0
//! ```
//! 
//! ## Cast Ranged to primitives
//! 
//! Casting to integer types is allowed when the value is proven to
//! fit into the result type:
//! 
//! ```
//! # #![feature(const_panic)] #![feature(const_generics)] #![feature(const_evaluatable_checked)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! let x = r!([0 200] 20);
//! assert_eq!(20_u8, x.into());
//! ```
//! 
//! ```compile_fail
//! # #![feature(const_panic)] #![feature(const_generics)] #![feature(const_evaluatable_checked)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! let x = r!([0 200] 20);
//! assert_eq!(20_i8, x.into()); // Error: can't fit the range 128..=200 in i8
//! ```
//! 
//! There is also a set of const functions for Ranged to primitive casting:
//! 
//! ```
//! # #![feature(const_panic)] #![feature(const_generics)] #![feature(const_evaluatable_checked)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! let x = r!([0 200] 20);
//! let y = x.u8(); // y is u8
//! let z = x.i16(); // z is i16
//! ```
//! 
//! ## Comparison
//! 
//! Comparison between different Ranged types is allowed:
//! 
//! ```
//! # #![feature(const_panic)] #![feature(const_generics)] #![feature(const_evaluatable_checked)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! #
//! assert!(r!([1 6] 4) == r!([1 10] 4));
//! assert!(r!([1 6] 4) != r!([1 6] 5));
//! ```
//! 
//! ## Arithmetics
//! 
//! Currently addition, subtraction, multiplication and division operations are implemented.
//! The bounds of values are automatically recalculated:
//! 
//! ```
//! # #![feature(const_panic)] #![feature(const_generics)] #![feature(const_evaluatable_checked)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! let x = r!([1 6] 4);
//! let y = r!([1 6] 5);
//! 
//! let a = x + y;
//! let check_add: Ranged<2, 12> = a;
//! 
//! let s = x - y;
//! let check_sub: Ranged<-5, 5> = s;
//! 
//! let m = x * y;
//! let check_mul: Ranged<1, 36> = m;
//! 
//! let d = x / y;
//! let check_div: Ranged<0, 6> = d;
//! ```
//! 
//! The division is allowed only if it's impossible to store "0" in the divisor:
//! 
//! ```compile_fail
//! # #![feature(const_panic)] #![feature(const_generics)] #![feature(const_evaluatable_checked)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! let x = r!([1 6] 4);
//! let y = r!([0 6] 5);
//! let z = r!([-1 6] 5);
//! 
//! let d = x / y; // Error: division is not possible
//! let e = x / z; // Error: division is not possible
//! ```
//! 
//! ## Integer size
//! 
//! When MIN and MAX are provided, the
//! [`Ranged`](struct.Ranged.html) automatically chooses the signedness
//! and the size. It supports i8, u8, i16, u16, i32, u32, i64, u64 and i128 layouts (u128 is omitted).
//! 
//! ```
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! # fn main(){
//! use core::mem::size_of;
//! assert_eq!(size_of::<Ranged::<-100, 127>>(), 1); // The range fits i8
//! assert_eq!(size_of::<Ranged::<0, 200>>(), 1); // The range fits u8
//! assert_eq!(size_of::<Ranged::<-100, 200>>(), 2); // The range fits i16
//! assert_eq!(size_of::<Ranged::<0, 90000>>(), 4); // The range fits i32
//! # }
//! ```
//! 
//! The implementation heavily relies on the optimizer.



#![no_std]

#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
#![feature(const_panic)]
#![feature(const_trait_impl)]
#![feature(const_raw_ptr_deref)]
#![feature(specialization)]

#![warn(missing_docs)]


#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(test)]
mod tests;

#[doc(hidden)]
pub mod value_check;
use value_check::*;

pub mod holder;
use holder::*;


#[allow(non_camel_case_types)]
type irang = i128;


// Pick out the "smallest" IntLayout that fits the min..=max range
#[doc(hidden)]
pub const fn memlayout(min: irang, max: irang) -> IntLayout {
    macro_rules! crange {
        ($($t:ident)+) => {
            $(
                if core::$t::MIN as irang <= min && max <= core::$t::MAX as irang {return IntLayout::$t}
            )+
        }
    }

    if min>max {
        panic!("Ranged error: MIN cannot be greater than MAX")
    }

    crange!{i8 u8 i16 u16 i32 u32 i64 u64}
    IntLayout::i128
}

/// A value restricted to the given bounds
#[derive(Clone, Copy)]
pub struct Ranged<const MIN: irang, const MAX: irang>
where 
    [u8; memlayout(MIN, MAX).bytes()]:,
{
    v: NumberBytes<{memlayout(MIN, MAX)}>
}

#[doc(hidden)]
pub const fn allow_creation(min: irang, v: irang, max: irang) -> bool {
    min <= v && v <= max
}

impl<const MIN: irang, const MAX: irang> Ranged<MIN, MAX> 
where
[u8; memlayout(MIN, MAX).bytes()]:
{
    #[doc(hidden)]
    pub const unsafe fn __unsafe_new(n: irang) -> Self {
        Ranged{v: NumberBytes::from_irang(n)}
    }

    /// Create a Ranged value with a runtime bounds checking
    pub const fn new(n: irang) -> Option<Self>
    {
        if (MIN <= n) && (n <= MAX) {
            Some( unsafe {Self::__unsafe_new(n)} )
        }
        else {
            None
        }
    }

    /// Create a Ranged value with a compile time bounds checking
    pub const fn create_const<const V: irang>() -> Self
    where 
        Assert<{OperationPossibility::allow_if(allow_creation(MIN,V,MAX))}>: IsAllowed,
    {
        unsafe {Self::__unsafe_new(V)}
    }


    /// Convert Ranged to a primitive
    const fn get(self) -> irang {
        self.v.to_irang()
    }
}


/// Create a ranged value at compile time
/// ```
/// #![feature(const_generics)]
/// #![feature(const_if_match)]
/// #![feature(const_panic)]
/// # #[macro_use] extern crate ranged_integers;
/// use ranged_integers::*;
/// 
/// # fn main() {
/// let a = r![10];  // Ranged<10, 10> with a value 10
/// let b = r!([0 42] 23);  // Ranged<0, 42> with a value 23
/// # }
/// ```
/// 
/// It will fail if the bound checking fails:
/// ```compile_fail
/// #![feature(const_if_match)]
/// #![feature(const_panic)]
/// # #[macro_use] extern crate ranged_integers;
/// use ranged_integers::*;
/// 
/// # fn main() {
/// let c = r!([0 23] 42);  // Compile error: Ranged<0, 23> with a value 42 is not possible
/// # }
/// ```
#[macro_export]
macro_rules! r {
    ([$min:literal $max:literal] $x:expr) => {
        {
            Ranged::<{$min}, {$max}>::create_const::<$x>()
        }
    };

    ($v:literal) => {
        {
            const __Z : Ranged<$v, $v> = {
                unsafe {Ranged::__unsafe_new($v)}
            };
            __Z
        }
    };
}

impl<const MIN: irang, const MAX: irang> core::fmt::Display for Ranged<MIN, MAX> 
where
[(); memlayout(MIN, MAX).bytes()]:,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.clone().get())
    }
}

impl<const MIN: irang, const MAX: irang> core::fmt::Debug for Ranged<MIN, MAX> 
where
[(); memlayout(MIN, MAX).bytes()]:,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if MIN==MAX {
            write!(f, "r![{}]", MIN)
        }
        else {
            write!(f, "r![[{} {}] {}]", MIN, MAX, self.clone().get())
        }
    }
}


impl<const AMIN: irang, const AMAX: irang, const BMIN: irang, const BMAX: irang>
const core::ops::Add<Ranged<BMIN, BMAX>> for Ranged<AMIN, AMAX>
where
    [(); memlayout(AMIN, AMAX).bytes()]:,
    [(); memlayout(BMIN, BMAX).bytes()]:,
    [(); memlayout(AMIN+BMIN, AMAX+BMAX).bytes()]:,
{
    type Output = Ranged<{AMIN+BMIN}, {AMAX+BMAX}>;

    fn add(self, rhs: Ranged<BMIN, BMAX>) -> Self::Output {
        unsafe{ Ranged::__unsafe_new(self.get()+rhs.get()) }
    }
}



impl<const AMIN: irang, const AMAX: irang, const BMIN: irang, const BMAX: irang>
const core::ops::Sub<Ranged<BMIN, BMAX>> for Ranged<AMIN, AMAX>
where
    [(); memlayout(AMIN, AMAX).bytes()]:,
    [(); memlayout(BMIN, BMAX).bytes()]:,
    [(); memlayout(AMIN-BMAX, AMAX-BMIN).bytes()]:,
{
    type Output = Ranged<{AMIN-BMAX}, {AMAX-BMIN}>;

    fn sub(self, rhs: Ranged<BMIN, BMAX>) -> Self::Output {
        unsafe{ Ranged::__unsafe_new(self.get()-rhs.get()) }
    }
}

macro_rules! reduce {
    ($fn:ident, $a:expr) => ( $a );
    ($fn:ident, $a:expr, $($args:expr),+) => {
        {
            $fn($a, reduce!($fn, $($args),+ ))
        }
    };
}
const fn min_irang(x: irang, y: irang) -> irang {
    if x<y {x} else {y}
}
const fn max_irang(x: irang, y: irang) -> irang {
    if x>y {x} else {y}
}
const fn max_4(vals: (irang,irang,irang,irang))->irang {
    reduce!(max_irang, vals.0, vals.1, vals.2, vals.3)
}
const fn min_4(vals: (irang,irang,irang,irang))->irang {
    reduce!(min_irang, vals.0, vals.1, vals.2, vals.3)
}
const fn max_2(vals: (irang,irang))->irang {
    reduce!(max_irang, vals.0, vals.1)
}
const fn min_2(vals: (irang,irang))->irang {
    reduce!(min_irang, vals.0, vals.1)
}

#[doc(hidden)] pub const fn max_cross(amin: irang, amax: irang, bmin: irang, bmax: irang)->irang {
    max_4((amin*bmin, amin*bmax, amax*bmin, amax*bmax))
}
#[doc(hidden)] pub const fn min_cross(amin: irang, amax: irang, bmin: irang, bmax: irang)->irang {
    min_4((amin*bmin, amin*bmax, amax*bmin, amax*bmax))
}


impl<const AMIN: irang, const AMAX: irang, const BMIN: irang, const BMAX: irang>
const core::ops::Mul<Ranged<BMIN, BMAX>> for Ranged<AMIN, AMAX>
where
    [(); memlayout(AMIN, AMAX).bytes()]:,
    [(); memlayout(BMIN, BMAX).bytes()]:,
    [(); memlayout(min_cross(AMIN, AMAX, BMIN, BMAX), max_cross(AMIN, AMAX, BMIN, BMAX)).bytes()]:,
{
    type Output = Ranged<{min_cross(AMIN, AMAX, BMIN, BMAX)}, {max_cross(AMIN, AMAX, BMIN, BMAX)}>;

    fn mul(self, rhs: Ranged<BMIN, BMAX>) -> Self::Output {
        unsafe{ Ranged::__unsafe_new(self.get() * rhs.get()) }
    }
}


#[doc(hidden)]
pub const fn allow_division(bmin: irang, bmax: irang) -> OperationPossibility {
    if ((bmin > 0) && (bmax > 0)) || ((bmin < 0) && (bmax < 0)) {OperationPossibility::Allowed} else {OperationPossibility::Forbidden}
}

#[doc(hidden)] pub const fn singleside_div_min(amin: irang, amax: irang, bmin: irang, bmax: irang) -> irang {
    min_4((amin/bmin, amin/bmax, amax/bmin, amax/bmax))
}
#[doc(hidden)] pub const fn singleside_div_max(amin: irang, amax: irang, bmin: irang, bmax: irang) -> irang {
    max_4((amin/bmin, amin/bmax, amax/bmin, amax/bmax))
}

impl<const AMIN: irang, const AMAX: irang, const BMIN: irang, const BMAX: irang>
const core::ops::Div<Ranged<BMIN, BMAX>> for Ranged<AMIN, AMAX>
where
    [(); memlayout(AMIN, AMAX).bytes()]:,
    [(); memlayout(BMIN, BMAX).bytes()]:,
    [(); memlayout(singleside_div_min(AMIN, AMAX, BMIN, BMAX), singleside_div_max(AMIN, AMAX, BMIN, BMAX)).bytes()]:,

    Assert<{allow_division(BMIN, BMAX)}>: IsAllowed
{
    type Output = Ranged<{singleside_div_min(AMIN, AMAX, BMIN, BMAX)}, {singleside_div_max(AMIN, AMAX, BMIN, BMAX)}>;

    fn div(self, rhs: Ranged<BMIN, BMAX>) -> Self::Output {
        unsafe{ Ranged::__unsafe_new(self.get() / rhs.get()) }
    }
}

#[doc(hidden)] pub const fn singleside_rem_min(amin: irang, amax: irang, bmin: irang, bmax: irang) -> irang {
    if amin == amax && bmin == bmax { amin%bmin } 
    else if amin >= 0 { 0 } 
    else { 1 - max_2((bmax.abs(), bmin.abs())) }
}
#[doc(hidden)] pub const fn singleside_rem_max(amin: irang, amax: irang, bmin: irang, bmax: irang) -> irang {
    if amin == amax && bmin == bmax { amin%bmin } 
    else if amax <= 0 { 0 } 
    else { max_2((bmax.abs(), bmin.abs()))-1 }
}

impl<const AMIN: irang, const AMAX: irang, const BMIN: irang, const BMAX: irang>
const core::ops::Rem<Ranged<BMIN, BMAX>> for Ranged<AMIN, AMAX>
where
    [(); memlayout(AMIN, AMAX).bytes()]:,
    [(); memlayout(BMIN, BMAX).bytes()]:,
    [(); memlayout(singleside_rem_min(AMIN, AMAX, BMIN, BMAX), singleside_rem_max(AMIN, AMAX, BMIN, BMAX)).bytes()]:,

    Assert<{allow_division(BMIN, BMAX)}>: IsAllowed
{
    type Output = Ranged<{singleside_rem_min(AMIN, AMAX, BMIN, BMAX)}, {singleside_rem_max(AMIN, AMAX, BMIN, BMAX)}>;

    fn rem(self, rhs: Ranged<BMIN, BMAX>) -> Self::Output {
        unsafe{ Ranged::__unsafe_new(self.get() % rhs.get()) }
    }
}

/*
...
    let a:Ranged<-10,20>  = -r![[-20 10] 10];
leads to an error

    Ranged::<{$min}, {$max}>::create_const::<$x>()
    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `-10_i128`, found `-20_i128`
...
*/
impl<const MIN: irang, const MAX: irang>
const core::ops::Neg for Ranged<MIN, MAX>
where
    [(); memlayout(MIN, MAX).bytes()]:,
    [(); memlayout(-MAX, -MIN).bytes()]:,
{
    type Output = Ranged<{-MAX}, {-MIN}>;

    fn neg(self) -> Self::Output {
        unsafe{ Ranged::__unsafe_new(-self.get()) }
    }
}



impl<const AMIN: irang, const AMAX: irang, const BMIN: irang, const BMAX: irang>
const core::cmp::PartialEq<Ranged<BMIN, BMAX>> for Ranged<AMIN, AMAX>
where
    [(); memlayout(AMIN, AMAX).bytes()]:,
    [(); memlayout(BMIN, BMAX).bytes()]:,
{
    fn eq(&self, rhs: &Ranged<BMIN, BMAX>) -> bool {
        self.get() == rhs.get()
    }
}

impl<const AMIN: irang, 
const AMAX: irang> core::cmp::Eq for Ranged<AMIN, AMAX>
where 
    [(); memlayout(AMIN, AMAX).bytes()]:,
    {}


/// Convert int value to Ranged according to its bounds
///
/// Implemented for integer primitives.
pub trait AsRanged {
    /// Conversion output
    type Res;

    /// Convert to Ranged
    fn as_ranged(self) -> Self::Res;
}


macro_rules! int_ranged_converters {
    ($($t: ident)+) => {
        
        #[doc(hidden)]
        pub mod converter_checkers {
            use super::OperationPossibility;
            use super::irang;

            $(
                #[doc(hidden)] pub const fn $t (min: irang, max: irang)->OperationPossibility {
                    OperationPossibility::allow_if(min>=core::$t::MIN as irang && max<=core::$t::MAX as irang)
                }
            )+
        }

        $(
            impl<const MIN: irang, const MAX: irang> Ranged<MIN, MAX> 
            where
            [u8; memlayout(MIN, MAX).bytes()]:,
            {
                #[doc=concat!("Convert a Ranged into ", stringify!($t), " value")]
                pub const fn $t(self) -> $t 
                where Assert<{converter_checkers::$t(MIN, MAX)}>: IsAllowed
                {
                    self.get() as $t
                }
            }

            impl<const MIN: irang, const MAX: irang> From<Ranged<MIN, MAX>> for $t
            where
                [u8; memlayout(MIN, MAX).bytes()]:,
                Assert<{converter_checkers::$t(MIN, MAX)}>: IsAllowed,
            {
                fn from(a: Ranged<MIN, MAX>) -> Self { a.$t() }
            }

            impl AsRanged for $t {
                type Res = Ranged<{core::$t::MIN as irang},{core::$t::MAX as irang}>;
                fn as_ranged(self) -> Self::Res {
                    unsafe {Self::Res::__unsafe_new(self as irang)}
                }
            }

            impl<const VAL: irang> const core::ops::Rem<Ranged<VAL, VAL>> for $t
            where
                [(); memlayout(VAL, VAL).bytes()]:,
                [(); memlayout(0, VAL.abs()-1).bytes()]:,
            {
                type Output = Ranged<0, {VAL.abs()-1}>;

                fn rem(self, _rhs: Ranged<VAL, VAL>) -> Self::Output {
                    unsafe { Ranged::__unsafe_new(self as irang % VAL) }
                }
            }
        )+
    };
}

int_ranged_converters!{i8 u8 i16 u16 i32 u32 i64 u64 i128}


#[doc(hidden)] pub const fn expansion_possible(smin: irang, smax: irang, rmin: irang, rmax: irang) -> OperationPossibility {
    OperationPossibility::allow_if(rmin <= smin && rmax >= smax)
}

impl<const MIN: irang, const MAX: irang> Ranged<MIN, MAX> 
where
[u8; memlayout(MIN, MAX).bytes()]:,
{
    /// Expand the range conserving the value
    pub const fn expand<const RMIN: irang, const RMAX: irang>(self) -> Ranged<RMIN, RMAX>
    where
        [u8; memlayout(RMIN, RMAX).bytes()]:,
        Assert<{expansion_possible(MIN, MAX, RMIN, RMAX)}>: IsAllowed,
    {
        unsafe{  Ranged::__unsafe_new(self.get())  }
    }
}

#[allow(dead_code)]
#[doc(hidden)]
/** 
```
# #![feature(const_generics)] #![feature(const_panic)] use ranged_integers::*;
u8::from(r![0]);
```

```compile_fail
# #![feature(const_generics)] #![feature(const_panic)] use ranged_integers::*;
u8::from(r![-1]);
```

```
# #![feature(const_generics)] #![feature(const_panic)] use ranged_integers::*;
u8::from(r![255]);
```

```compile_fail
# #![feature(const_generics)] #![feature(const_panic)] use ranged_integers::*;
u8::from(r![256]);
```

```
# #![feature(const_generics)] #![feature(const_panic)] use ranged_integers::*;
i8::from(r![-128]);
```


```compile_fail
# #![feature(const_generics)] #![feature(const_panic)] use ranged_integers::*;
i8::from(r![-129]);
```

```
# #![feature(const_generics)] #![feature(const_panic)] use ranged_integers::*;
i8::from(r![127]);
```


```compile_fail
# #![feature(const_generics)] #![feature(const_panic)] use ranged_integers::*;
i8::from(r![128]);
```


```
# #![feature(const_panic)] #![feature(const_generics)] #![feature(const_evaluatable_checked)]
# #[macro_use] extern crate ranged_integers; use ranged_integers::*;
let a = r![[100 1000] 500] / r![[1 6] 5];
```
```compile_fail
# #![feature(const_panic)] #![feature(const_generics)] #![feature(const_evaluatable_checked)]
# #[macro_use] extern crate ranged_integers; use ranged_integers::*;
let a = r![[100 1000] 500] / r![[0 6] 5];
```
```compile_fail
# #![feature(const_panic)] #![feature(const_generics)] #![feature(const_evaluatable_checked)]
# #[macro_use] extern crate ranged_integers; use ranged_integers::*;
let a = r![[100 1000] 500] / r![[-1 6] 5];
```

```
# #![feature(const_generics)] #![feature(const_panic)] use ranged_integers::*;
Ranged::<0,1>::new(1);
```


```compile_fail
# #![feature(const_generics)] #![feature(const_panic)] use ranged_integers::*;
Ranged::<1,0>::new(1);
```


```
# #![feature(const_generics)] #![feature(const_panic)] use ranged_integers::*;
let x: Ranged::<0,1> = Ranged::<0,1>::new(1).unwrap();
```


```
# #![feature(const_generics)] #![feature(const_panic)] use ranged_integers::*;
let x: Ranged::<0,1> = Ranged::new(1).unwrap();
```
*/

struct Failtests;
