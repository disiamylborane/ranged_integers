//! # Ranged integers [nightly only]
//! 
//! **Note: WIP, the API may change.**
//! 
//! The crate provides an integer-like type restricted to a compile-time defined range.
//! 
//! [`Ranged<MIN, MAX>`](struct.Ranged.html) is bounded to [MIN, MAX] interval **inclusively**.
//! Parametrized by `i128`.
//! 
//! ## Integer size
//! 
//! When MIN and MAX are provided, the
//! [`Ranged`](struct.Ranged.html) automatically chooses the signedness
//! and the size. It supports i8, u8, i16, u16, i32, u32, i64, u64 and i128 layouts.
//! The value may be casted to these types.
//! Due to the fact the `Ranged` is parametrized by `i128`, `u128` layout is not supported.
//! 
//! ```
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! # fn main(){
//! use core::mem::size_of;
//! assert_eq!(size_of::<Ranged::<-100, 127>>(), 1); // The range fits i8
//! assert_eq!(size_of::<Ranged::<0, 200>>(), 1); // The range fits u8
//! assert_eq!(size_of::<Ranged::<-100, 200>>(), 2); // The range fits i16
//! assert_eq!(size_of::<Ranged::<0, 90000>>(), 4); // i32 is needed
//! # }
//! ```
//! 
//! The implementation heavily relies on the optimizer.
//! 
//! # Usage and examples
//! 
//! ## Prerequisites
//! 
//! The library's macro [`ranged!`](macro.ranged.html) requires the following Rust features enabled:
//! 
//! ```
//! #![feature(const_if_match)]
//! #![feature(const_panic)]
//! ```
//! 
//! ## Ranged semantics
//! 
//! Use `Ranged<MIN, MAX>` as a function argument to ensure the parameter range:
//! 
//! ```
//! # extern crate ranged_integers; use ranged_integers::*;
//! fn move_player(dice_roll: Ranged<1, 6>) {
//!     let x : i32 = dice_roll.into(); // Convert to int
//! }
//! ```
//! 
//! ## Create Ranged at compile time
//! 
//! The macro `ranged!([MIN MAX] VALUE)` creates the const Ranged:
//! 
//! ```
//! # #![feature(const_if_match)] #![feature(const_panic)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! # fn move_player(dice_roll: Ranged<1, 6>) {
//! #     let x : i32 = dice_roll.into(); // Convert back to int
//! # }
//! move_player(ranged!([1 6] 4));
//! ```
//! 
//! It fails if the bounds are corrupted:
//! 
//! ```compile_fail
//! # #![feature(const_if_match)] #![feature(const_panic)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! # fn move_player(dice_roll: Ranged<1, 6>) {
//! #     let x : i32 = dice_roll.get(); // Convert back to int
//! # }
//! move_player(ranged!([1 6] 7)); // Error: Can't store 7 in [1 6] inverval
//! move_player(ranged!([1 7] 7)); // Error: type mismatch, move_player() requires Ranged<1, 6>
//! ```
//! 
//! A special case with the single possible value:
//! 
//! ```
//! # #![feature(const_if_match)] #![feature(const_panic)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! let x = ranged![4]; // Means Ranged<4, 4> with the value 4
//! let y: Ranged<4,4> = x;
//! ```
//! 
//! ## Cast to Ranged at runtime
//! 
//! ### Way 1: Ensure the bounds with `new` method:
//! 
//! ```
//! # #![feature(const_if_match)] #![feature(const_panic)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! let some_i32 = 4;
//! let some_wrong_i32 = 8;
//! assert!(Ranged::<0, 6>::new(some_i32).unwrap() == ranged![4]);
//! assert!(Ranged::<0, 6>::new(some_wrong_i32) == None);
//! ```
//! 
//! The user must always specify the bounds with "turbofish" operator when
//! uses `new()` method (doesn't compile otherwise). This is related to a
//! compile-time check for MIN<MAX.
//! This is to be fixed when possible.
//! 
//! ```
//! # #![feature(const_if_match)] #![feature(const_panic)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! let a : Ranged::<0, 6> = Ranged::<0, 6>::new(1).unwrap();  // Ok
//! ```
//! 
//! ```compile_fail
//! # #![feature(const_if_match)] #![feature(const_panic)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! let a : Ranged::<0, 6> = Ranged::new(1).unwrap();  // Currently fails
//! ```
//! 
//! ### Way 2: use remainder operation with the const divisor as a way to create Ranged:
//! 
//! ```
//! # #![feature(const_if_match)] #![feature(const_panic)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! let x = 15 % ranged![10];
//! let y: Ranged<0, 9> = x;
//! assert!(y == ranged![5]); // 15 % 10 == 5
//! ```
//! 
//! ```compile_fail
//! # #![feature(const_if_match)] #![feature(const_panic)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! let x = 15 % ranged![10];
//! let y: Ranged<0, 20> = x;  // Error: x is Ranged<0, 9>
//! ```
//! 
//! ### Way 3: Convert the primitive types to `Ranged` with their native bounds:
//! 
//! ```
//! # #![feature(const_if_match)] #![feature(const_panic)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! let x = 15_u8.as_ranged(); // Ranged<0, 255>
//!                            // Trait AsRanged must be in scope
//! assert!(x == ranged![15]);
//! ```
//! 
//! ## Expand Ranged bounds
//! 
//! Use `Expand` helper to widen the Ranged bounds:
//! 
//! ```
//! # #![feature(const_if_match)] #![feature(const_panic)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! let x = ranged!([0 100] 20);
//! let y = Expand::<0,100,-5,200>(x).expand(); // From [0 100] to [-5 200]
//! let check: Ranged<-5, 200> = y;
//! ```
//! 
//! Shrinking is not allowed:
//! 
//! ```compile_fail
//! # #![feature(const_if_match)] #![feature(const_panic)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! let x = ranged!([0 100] 20);
//! let y = Expand::<0,100,5,200>(x).expand(); // Error: must contain [0 4]
//! ```
//! 
//! ## Cast from Ranged
//! 
//! Casting to integer types is allowed when the value is proven to
//! fit in the result type:
//! 
//! ```
//! # #![feature(const_if_match)] #![feature(const_panic)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! let x = ranged!([0 200] 20);
//! assert_eq!(20_u8, x.into()); // Impossible in const fns
//! assert_eq!(20_u8, x.u8());   // Possible in const fns
//! ```
//! 
//! ```compile_fail
//! # #![feature(const_if_match)] #![feature(const_panic)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! let x = ranged!([0 200] 20);
//! assert_eq!(20_i8, x.into()); // Error: can't fit the range 128..=200 in i8
//! ```
//! 
//! ## MIN and MAX checks
//! 
//! It's unluckily possible to create something like `Ranged<1, 0>` type where
//! min and max are jumbled. However, it's impossible to use the documented creation
//! and int-conversion features with such types.
//! 
//! ```
//! # extern crate ranged_integers; use ranged_integers::*;
//! type T = Ranged<1, 0>; // Works
//! ```
//! 
//! ```compile_fail
//! # extern crate ranged_integers; use ranged_integers::*;
//! let x = Ranged::<1, 0>::new(1); // Compile error: MAX<MIN is weird
//! ```
//! 
//! ```compile_fail
//! # extern crate ranged_integers; use ranged_integers::*;
//! let x = ranged!([1 0] 1); // Compile error
//! ```
//! 
//! ## Comparison
//! 
//! Comparison between different types is allowed:
//! 
//! ```
//! # #![feature(const_if_match)] #![feature(const_panic)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! #
//! assert!(ranged!([1 6] 4) == ranged!([1 10] 4));
//! assert!(ranged!([1 6] 4) != ranged!([1 6] 5));
//! ```
//! 
//! ## Arithmetics
//! 
//! Currently addition, subtraction, multiplication and division are possible.
//! The bounds of values are automatically recalculated:
//! 
//! ```compile_fail
//! # #![feature(const_if_match)] #![feature(const_panic)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! let x = ranged!([0 6] 4);
//! let y: Ranged<0, 10> = x*x; // Error: wrong type, must be Ranged<0, 36>
//! ```
//! 
//! ### Addition
//! 
//! ```
//! # #![feature(const_if_match)] #![feature(const_panic)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! let a = ranged!([1 6] 4);
//! let b = ranged!([1 6] 5);
//! let c = a + b;     // Impossible in const fns
//! let d = a.add(b);  // Possible in const fns
//! let check: Ranged<2, 12> = c;  // 2 = 1+1, 12 = 6+6
//! ```
//! 
//! ### Subtraction
//! 
//! ```
//! # #![feature(const_if_match)] #![feature(const_panic)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! let a = ranged!([1 6] 4);
//! let b = ranged!([1 6] 5);
//! let c = a - b;     // Impossible in const fns
//! let d = a.sub(b);  // Possible in const fns
//! let check: Ranged<-5, 5> = c;  // -5 = 1-6, 5 = 6-1
//! ```
//! 
//! ### Multiplication
//! 
//! ```
//! # #![feature(const_if_match)] #![feature(const_panic)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! let a = ranged!([1 6] 4);
//! let b = ranged!([1 6] 5);
//! let c = a * b;     // Impossible in const fns
//! let d = a.mul(b);  // Possible in const fns
//! let check: Ranged<1, 36> = c;  // 1 = 1*1, 36 = 6*6
//! ```
//! 
//! ### Division
//!
//! Allowed if the range of second operand doesn't include 0.
//! The syntax is a bit non-trivial.
//! 
//! ```
//! # #![feature(const_if_match)] #![feature(const_panic)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! let a = ranged!([1 6] 4);
//! let b = ranged!([1 6] 5);
//! let c = RDiv(a, b).div(); // Possible in const fns
//! let check: Ranged<0, 6> = c;  // 0 = 1/6, 6 = 6/1
//! ```
//! 
//! ```compile_fail
//! # #![feature(const_if_match)] #![feature(const_panic)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! let a = ranged!([1 6] 4);
//! let b = ranged!([-1 6] 5);
//! let x = RDiv(a, b).div(); // Disallowed, the second operand may be 0
//! ```


#![no_std]
#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(const_if_match)]
#![feature(const_panic)]
#![feature(const_fn)]
#![feature(specialization)]
#![feature(const_fn_union)]
#![feature(trivial_bounds)]

#![warn(missing_docs)]


#[allow(non_camel_case_types)]
type irang = i128;


trait Aligner
{
    type A: Copy;
}

// FIXME: using IntLayout in AlignWrap causes stack overflow when called from another crate
#[derive(Copy, Clone)]
struct AlignWrap<const N: usize>;

impl<const N: usize> Aligner for AlignWrap<N> {
    default type A = irang;
}

impl Aligner for AlignWrap<1> { type A = i8; }
impl Aligner for AlignWrap<2> { type A = i16; }
impl Aligner for AlignWrap<4> { type A = i32; }
impl Aligner for AlignWrap<8> { type A = i64; }


// The storage for a number
#[derive(Copy, Clone)]
#[repr(C)]
union NumberBytes<const BYTES: usize> {
    // Ensure the alignment
    val: <AlignWrap<BYTES> as Aligner>::A,
    // Bytewise access
    bytes: [u8; BYTES],
}

impl<const BYTES: usize> NumberBytes<BYTES> {
    const fn new() -> Self {
        Self{bytes: [0; BYTES]}
    }
}

// The number with tunable size
#[derive(Copy, Clone)]
struct Number<const LAYOUT: IntLayout>
{
    val: NumberBytes<{LAYOUT.bytes()}>
}

impl<const LAYOUT: IntLayout> Number<LAYOUT> {
    #[inline(always)]
    const fn from_irang(v: irang) -> Self {
        unsafe {
            let mut x = Self{val: NumberBytes::new()};
            match LAYOUT {
                IntLayout::i8 => {
                    let bytes = (v as i8).to_ne_bytes();
                    x.val.bytes[0] = bytes[0];
                }
                IntLayout::u8 => {
                    let bytes = (v as u8).to_ne_bytes();
                    x.val.bytes[0] = bytes[0];
                }
                IntLayout::i16 => {
                    let v = (v as i16).to_ne_bytes();
                    x.val.bytes[0] = v[0];
                    x.val.bytes[1] = v[1];
                }
                IntLayout::u16 => {
                    let v = (v as u16).to_ne_bytes();
                    x.val.bytes[0] = v[0];
                    x.val.bytes[1] = v[1];
                }
                IntLayout::i32 => {
                    let v = (v as i32).to_ne_bytes();
                    x.val.bytes[0] = v[0];
                    x.val.bytes[1] = v[1];
                    x.val.bytes[2] = v[2];
                    x.val.bytes[3] = v[3];
                }
                IntLayout::u32 => {
                    let v = (v as u32).to_ne_bytes();
                    x.val.bytes[0] = v[0];
                    x.val.bytes[1] = v[1];
                    x.val.bytes[2] = v[2];
                    x.val.bytes[3] = v[3];
                }
                IntLayout::i64 => {
                    let v = (v as i64).to_ne_bytes();
                    x.val.bytes[0] = v[0];
                    x.val.bytes[1] = v[1];
                    x.val.bytes[2] = v[2];
                    x.val.bytes[3] = v[3];
                    x.val.bytes[4] = v[4];
                    x.val.bytes[5] = v[5];
                    x.val.bytes[6] = v[6];
                    x.val.bytes[7] = v[7];
                }
                IntLayout::u64 => {
                    let v = (v as u64).to_ne_bytes();
                    x.val.bytes[0] = v[0];
                    x.val.bytes[1] = v[1];
                    x.val.bytes[2] = v[2];
                    x.val.bytes[3] = v[3];
                    x.val.bytes[4] = v[4];
                    x.val.bytes[5] = v[5];
                    x.val.bytes[6] = v[6];
                    x.val.bytes[7] = v[7];
                }
                IntLayout::i128 => {
                    let v = (v as i128).to_ne_bytes();
                    x.val.bytes[0] = v[0];
                    x.val.bytes[1] = v[1];
                    x.val.bytes[2] = v[2];
                    x.val.bytes[3] = v[3];
                    x.val.bytes[4] = v[4];
                    x.val.bytes[5] = v[5];
                    x.val.bytes[6] = v[6];
                    x.val.bytes[7] = v[7];
                    x.val.bytes[8] = v[8];
                    x.val.bytes[9] = v[9];
                    x.val.bytes[10] = v[10];
                    x.val.bytes[11] = v[11];
                    x.val.bytes[12] = v[12];
                    x.val.bytes[13] = v[13];
                    x.val.bytes[14] = v[14];
                    x.val.bytes[15] = v[15];
                }
            }

            x
        }
    }
    #[inline(always)]
    const fn to_irang(self) -> irang {
        unsafe {
            let b = self.val.bytes;
            match LAYOUT {
                IntLayout::i8 => {
                    i8::from_ne_bytes([b[0]]) as irang
                }
                IntLayout::u8 => {
                    u8::from_ne_bytes([b[0]]) as irang
                }
                IntLayout::i16 => {
                    i16::from_ne_bytes([b[0], b[1]]) as irang
                }
                IntLayout::u16 => {
                    u16::from_ne_bytes([b[0], b[1]]) as irang
                }
                IntLayout::i32 => {
                    i32::from_ne_bytes([b[0], b[1], b[2], b[3]]) as irang
                }
                IntLayout::u32 => {
                    u32::from_ne_bytes([b[0], b[1], b[2], b[3]]) as irang
                }
                IntLayout::i64 => {
                    i64::from_ne_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]) as irang
                }
                IntLayout::u64 => {
                    u64::from_ne_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]) as irang
                }
                IntLayout::i128 => {
                    i128::from_ne_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7], b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]]) as irang
                }
            }
        }
    }
}

// Types of layout that Number can have
#[derive(PartialEq, Eq, Copy, Clone)]
#[allow(non_camel_case_types)]
enum IntLayout {
    i8, u8, i16, u16, i32, u32, i64, u64, i128
}

// Get the layout from value bounds
const fn memlayout(min: irang, max: irang) -> IntLayout {
    macro_rules! crange {
        ($t:ident) => {
            core::$t::MIN as irang <= min && max <= core::$t::MAX as irang
        }
    };

    if      crange!{i8} { IntLayout::i8 }
    else if crange!{u8} { IntLayout::u8 }
    else if crange!{i16} { IntLayout::i16 }
    else if crange!{u16} { IntLayout::u16 }
    else if crange!{i32} { IntLayout::i32 }
    else if crange!{u32} { IntLayout::u32 }
    else if crange!{i64} { IntLayout::i64 }
    else if crange!{u64} { IntLayout::u64 }
    else {
        IntLayout::i128
    }
}

impl IntLayout {
    const fn bytes(self) -> usize {
        match self {
            Self::i8 => 1,
            Self::u8 => 1,
            Self::i16 => 2,
            Self::u16 => 2,
            Self::i32 => 4,
            Self::u32 => 4,
            Self::i64 => 8,
            Self::u64 => 8,
            Self::i128 => 16,
        }
    }
}


/// Create a ranged value at compile-time
/// ```
/// #![feature(const_if_match)]
/// #![feature(const_panic)]
/// # #[macro_use] extern crate ranged_integers;
/// use ranged_integers::*;
/// 
/// # fn main() {
/// let a = ranged![10];  // Ranged<10, 10> with a value 10
/// let b = ranged!([0 42] 23);  // Ranged<0, 42> with a value 23
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
/// let c = ranged!([0 23] 42);  // Compile error: Ranged<0, 23> with a value 42 is not possible
/// # }
/// ```
#[macro_export]
macro_rules! ranged {
    ([$min:literal $max:literal] $x:expr) => {
        {
            const __Z : Ranged<$min, $max> = {
                assert!( ($min <= $x) && ($x <= $max) );
                unsafe {Ranged::__unsafe_new($x)}
            };
            __Z
        }
    };

    ($v:expr) => {
        {unsafe{Ranged::<{$v}, {$v}>::__unsafe_new($v)}}
    };
}


// Check the MIN and MAX are allowed to be used in Ranged
// This means MIN <= MAX
#[doc(hidden)]
pub struct RangeCheck<const MIN: irang, const MAX: irang> {}

impl<const MIN: irang, const MAX: irang> 
OperationCheck for RangeCheck<MIN, MAX> {
    type Acc = AllowOperation<{(MIN <= MAX)as u8}>;
}

/// A value restricted to a given bounds
/// 
/// See [crate doc](index.html)
#[derive(Copy, Clone)]
pub struct Ranged<const MIN: irang, const MAX: irang>
{
    v: Number<{memlayout(MIN, MAX)}>
}


#[doc(hidden)]
// FIXME: The struct is parametrized by u8 because bool
// and enum both cause stack overflow error
// 
// Implements OperationPossible when ACC == 1
pub struct AllowOperation<const ACC: u8>;

impl<const ACC: u8> core::fmt::Debug for AllowOperation<ACC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "AllowOperation<{}>", ACC)
    }
}

#[doc(hidden)]
pub trait OperationPossible{}
impl OperationPossible for AllowOperation<1> {}


#[doc(hidden)]
pub trait OperationCheck{
    type Acc;
}

#[allow(unused_macros)]
macro_rules! make_converters {
    ($($typ:ident $doc:expr,)+) => {
        #[doc(hidden)]
        pub mod convertors {
            use super::irang;
            mod bound_checkers {
                use super::irang;
                $(
                    pub const fn $typ(min: irang, max: irang) -> u8 {
                        (min >= core::$typ::MIN as irang && max <= core::$typ::MAX as irang) as u8
                    }
                )+
            }

            $(
                #[allow(non_camel_case_types)]
                pub struct $typ<const MIN: irang, const MAX: irang> {}
                impl<const MIN: irang, const MAX: irang> crate::OperationCheck for $typ<MIN,MAX> {
                    type Acc = crate::AllowOperation<{bound_checkers::$typ(MIN,MAX)}>;
                }
            )+
        }

        $(
            impl<const MIN: irang, const MAX: irang> From<Ranged<MIN,MAX>> for $typ
            where <convertors::$typ<MIN,MAX> as OperationCheck>::Acc: OperationPossible,
                  <RangeCheck<MIN,MAX> as OperationCheck>::Acc: OperationPossible
            {
                fn from(s: Ranged<MIN,MAX>) -> $typ {
                    s.v.to_irang() as $typ
                }
            }

            impl<const MIN: irang, const MAX: irang> Ranged<MIN,MAX>
            where <convertors::$typ<MIN,MAX> as OperationCheck>::Acc: OperationPossible,
                  <RangeCheck<MIN,MAX> as OperationCheck>::Acc: OperationPossible
            {
                #[doc = $doc]
                pub const fn $typ(self) -> $typ {
                    self.v.to_irang() as $typ
                }
            }

            impl AsRanged for $typ {
                type R = Ranged<{core::$typ::MIN as irang},{core::$typ::MAX as irang}>;
                fn as_ranged(self) -> Self::R {
                    unsafe {Self::R::__unsafe_new(self as irang)}
                }
            }
        )+

    };
}

// The code for type casting
make_converters!{
    i8 "Cast to `i8` (enabled if fits)",
    u8 "Cast to `u8` (enabled if fits)",
    i16 "Cast to `i16` (enabled if fits)",
    u16 "Cast to `u16` (enabled if fits)",
    i32 "Cast to `i32` (enabled if fits)",
    u32 "Cast to `u32` (enabled if fits)",
    i64 "Cast to `i64` (enabled if fits)",
    u64 "Cast to `u64` (enabled if fits)",
    i128 "Cast to `i128` (enabled if fits)",
}

/*
macro_rules! fn_from {
    ($typ:ty) => {
        fn from(s: $typ) -> Self {
            unsafe {Self::__unsafe_new(s as irang)}
        }
    };
}

// FIXME: Causes stack overflow when braces are used (aka Ranged<{0}, {255}>) and
// called from another crate
impl From<i8> for Ranged<-128, 127> { fn_from!{i8} }
impl From<u8> for Ranged<0, 255> { fn_from!{u8} }
impl From<i16> for Ranged<-32768, 32767> { fn_from!{i16} }
impl From<u16> for Ranged<0, 65535> { fn_from!{u16} }
impl From<i32> for Ranged<-2147483648, 2147483647> { fn_from!{i32} }
impl From<u32> for Ranged<0, 4294967295> { fn_from!{u32} }
impl From<i64> for Ranged<-9223372036854775808, 9223372036854775807> { fn_from!{i64} }
impl From<u64> for Ranged<0, 18446744073709551615> { fn_from!{u64} }
impl From<i128> for Ranged<-170141183460469231731687303715884105728, 170141183460469231731687303715884105727> { fn_from!{i128} }
*/

/// Convert int value to Ranged according to its bounds
///
/// Implemented for integer primitives.
pub trait AsRanged {
    /// Conversion output
    type R;

    /// Convert to Ranged
    fn as_ranged(self) -> Self::R;
}


impl<const MIN: irang, const MAX: irang> core::fmt::Display for Ranged<MIN,MAX>
where <RangeCheck<MIN,MAX> as OperationCheck>::Acc: OperationPossible
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.get())
    }
}


impl<const MIN: irang, const MAX: irang> core::fmt::Debug for Ranged<MIN,MAX>
where <RangeCheck<MIN,MAX> as OperationCheck>::Acc: OperationPossible
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Ranged<{}, {}> {{ _val: {} }}", MIN, MAX, self.get())
    }
}


impl<const MIN: irang, const MAX: irang> Ranged<MIN, MAX> {
    #[doc(hidden)]
    /// Create the value without bounds checking
    ///
    /// For compile-time values use safe [ranged!](macro.ranged.html) macro instead
    pub const unsafe fn __unsafe_new(n: irang) -> Self {
        Ranged{v: Number::from_irang(n)}
    }

    const fn get(self) -> irang {
        self.v.to_irang()
    }
}

impl<const MIN: irang, const MAX: irang> Ranged<MIN, MAX> 
where <RangeCheck<MIN,MAX> as OperationCheck>::Acc: OperationPossible
{
    /// Check and create the value from i128
    pub const fn new(n: irang) -> Option<Self> {
        if (MIN <= n) && (n <= MAX) {
            Some( unsafe {Self::__unsafe_new(n)} )
        }
        else {
            None
        }
    }
}

#[doc(hidden)]
pub struct ExpandCheck<const MIN: irang, const MAX: irang, const NMIN: irang, const NMAX: irang> {}

impl<const MIN: irang, const MAX: irang, const NMIN: irang, const NMAX: irang> 
OperationCheck for ExpandCheck<MIN, MAX, NMIN, NMAX> {
    type Acc = AllowOperation<{(NMIN<=MIN && MAX<=NMAX) as u8}>;
}

/// A wrapper for Ranged bounds expansion
pub struct Expand<const MIN: irang, const MAX: irang, const NMIN: irang, const NMAX: irang>(
    pub Ranged<MIN, MAX> 
)
where <ExpandCheck<MIN,MAX,NMIN,NMAX> as OperationCheck>::Acc: OperationPossible;

impl<const MIN: irang, const MAX: irang, const NMIN: irang, const NMAX: irang> Expand<MIN, MAX, NMIN, NMAX> 
where <ExpandCheck<MIN,MAX,NMIN,NMAX> as OperationCheck>::Acc: OperationPossible
{
    /// Widen the bounds for a Ranged
    pub const fn expand(self) -> Ranged<NMIN,NMAX> {
        unsafe {Ranged::__unsafe_new(self.0.get())}
    }
}

const fn min_irang(x: irang, y: irang) -> irang {
    if x<y {x} else {y}
}
const fn max_irang(x: irang, y: irang) -> irang {
    if x>y {x} else {y}
}


macro_rules! reduce {
    ($fn:ident, $a:expr) => ( $a );
    ($fn:ident, $a:expr, $($args:expr),+) => {
        {
            $fn($a, reduce!($fn, $($args),+ ))
        }
    };
}

macro_rules! rem_trait {
    ($tp:ty) => {
        impl<const VAL: irang> core::ops::Rem<Ranged<VAL, VAL>> for $tp {
            type Output = Ranged<0, {VAL.abs()-1}>;
        
            fn rem(self, _rhs: Ranged<VAL, VAL>) -> Self::Output {
                unsafe { Ranged::__unsafe_new((irang::from(self) % VAL)) }
            }
        }
    };
}


rem_trait!{irang}
rem_trait!{i8}
rem_trait!{u8}
rem_trait!{i16}
rem_trait!{u16}
rem_trait!{i32}
rem_trait!{u32}
rem_trait!{i64}
rem_trait!{u64}


impl<const AMIN: irang, 
     const AMAX: irang,
     const BMIN: irang, 
     const BMAX: irang> core::ops::Add<Ranged<BMIN, BMAX>> for Ranged<AMIN, AMAX> {

        type Output = Ranged<{AMIN+BMIN}, {AMAX+BMAX}>;

        fn add(self, rhs: Ranged<BMIN, BMAX>) -> Self::Output {
            unsafe {Self::Output::__unsafe_new(self.get() + rhs.get())}
        }
     }

impl<const AMIN: irang, 
     const AMAX: irang> Ranged<AMIN, AMAX> {
        /// Add a ranged (const fn)
        pub const fn add<const BMIN: irang, const BMAX: irang>(self, rhs: Ranged<BMIN, BMAX>) -> Ranged<{AMIN+BMIN}, {AMAX+BMAX}> {
            unsafe {Ranged::__unsafe_new(self.get() + rhs.get())}
        }
     }


impl<const AMIN: irang, 
     const AMAX: irang,
     const BMIN: irang, 
     const BMAX: irang> core::ops::Sub<Ranged<BMIN, BMAX>> for Ranged<AMIN, AMAX>
     {

        type Output = Ranged<{AMIN-BMAX}, {AMAX-BMIN}>;

        fn sub(self, rhs: Ranged<BMIN, BMAX>) -> Self::Output {
            unsafe {Self::Output::__unsafe_new(self.get() - rhs.get())}
        }
    }

impl<const AMIN: irang, 
     const AMAX: irang> Ranged<AMIN, AMAX> {
        /// Subtract a ranged (const fn)
        pub const fn sub<const BMIN: irang, const BMAX: irang>(self, rhs: Ranged<BMIN, BMAX>) -> Ranged<{AMIN-BMAX}, {AMAX-BMIN}> {
            unsafe {Ranged::__unsafe_new(self.get() - rhs.get())}
        }
     }


const fn cross_mul(amin: irang, amax: irang, bmin: irang, bmax: irang)-> (irang,irang,irang,irang) {
    (amin*bmin, amin*bmax, amax*bmin, amax*bmax)
}

const fn max_4(vals: (irang,irang,irang,irang))->irang {
    reduce!(max_irang, vals.0, vals.1, vals.2, vals.3)
}
const fn min_4(vals: (irang,irang,irang,irang))->irang {
    reduce!(min_irang, vals.0, vals.1, vals.2, vals.3)
}



impl<const AMIN: irang, 
     const AMAX: irang,
     const BMIN: irang, 
     const BMAX: irang> core::ops::Mul<Ranged<BMIN, BMAX>> for Ranged<AMIN, AMAX> {

        type Output = Ranged<{min_4(cross_mul(AMIN, AMAX, BMIN, BMAX))}, 
                             {max_4(cross_mul(AMIN, AMAX, BMIN, BMAX))}>;

        fn mul(self, rhs: Ranged<BMIN, BMAX>) -> Self::Output {
            unsafe {Self::Output::__unsafe_new(self.get() * rhs.get())}
        }
    }


impl<const AMIN: irang, 
     const AMAX: irang> Ranged<AMIN, AMAX> {
        /// Multiply a ranged (const fn)
        pub const fn mul<const BMIN: irang, const BMAX: irang>(self, rhs: Ranged<BMIN, BMAX>) 
                                                -> Ranged<{min_4(cross_mul(AMIN, AMAX, BMIN, BMAX))}, 
                                                          {max_4(cross_mul(AMIN, AMAX, BMIN, BMAX))}> {
            unsafe {Ranged::__unsafe_new(self.get() * rhs.get())}
        }
     }

/// A wrapper for Ranged division
pub struct RDiv<const AMIN: irang, const AMAX: irang, const BMIN: irang, const BMAX: irang>(
    pub Ranged<AMIN,AMAX>,
    pub Ranged<BMIN,BMAX>
);
impl<const AMIN: irang, const AMAX: irang, const BMIN: irang, const BMAX: irang> RDiv<AMIN,AMAX,BMIN,BMAX>
where <DivCheck<BMIN, BMAX> as OperationCheck>::Acc: OperationPossible
{
    /// Perform division
    pub const fn div(self) -> Ranged<{div_min(AMIN, AMAX, BMIN, BMAX)}, {div_max(AMIN, AMAX, BMIN, BMAX)}>
    { unsafe {Ranged::__unsafe_new(self.0.get() / self.1.get())} }
}

#[doc(hidden)]
pub const fn allow_division(bmin: irang, bmax: irang) -> u8 {
    (((bmin > 0) && (bmax > 0)) || ((bmin < 0) && (bmax < 0))) as u8
}

const fn div_min(amin: irang, amax: irang, bmin: irang, bmax: irang) -> irang {
    min_4((amin/bmin, amin/bmax, amax/bmin, amax/bmax))
}
const fn div_max(amin: irang, amax: irang, bmin: irang, bmax: irang) -> irang {
    max_4((amin/bmin, amin/bmax, amax/bmin, amax/bmax))
}

#[doc(hidden)]
pub struct DivCheck<const BMIN: irang, const BMAX: irang> {}

impl<const BMIN: irang, const BMAX: irang> 
OperationCheck for DivCheck<BMIN, BMAX> {
    type Acc = AllowOperation<{allow_division(BMIN, BMAX)}>;
}





impl<const AMIN: irang, 
     const AMAX: irang,
     const BMIN: irang, 
     const BMAX: irang> core::cmp::PartialEq<Ranged<BMIN, BMAX>> for Ranged<AMIN, AMAX> {

        fn eq(&self, rhs: &Ranged<BMIN, BMAX>) -> bool {
            self.get() == rhs.get()
        }
    }

impl<const AMIN: irang, 
     const AMAX: irang> core::cmp::Eq for Ranged<AMIN, AMAX>
         {}


#[cfg(test)]
extern crate alloc;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
#[doc(hidden)]
/** 
```
# #![feature(const_if_match)] #![feature(const_panic)] use ranged_integers::*;
u8::from(ranged![0]);
```

```compile_fail
# #![feature(const_if_match)] #![feature(const_panic)] use ranged_integers::*;
u8::from(ranged![-1]);
```

```
# #![feature(const_if_match)] #![feature(const_panic)] use ranged_integers::*;
u8::from(ranged![255]);
```

```compile_fail
# #![feature(const_if_match)] #![feature(const_panic)] use ranged_integers::*;
u8::from(ranged![256]);
```

```
# #![feature(const_if_match)] #![feature(const_panic)] use ranged_integers::*;
i8::from(ranged![-128]);
```


```compile_fail
# #![feature(const_if_match)] #![feature(const_panic)] use ranged_integers::*;
i8::from(ranged![-129]);
```

```
# #![feature(const_if_match)] #![feature(const_panic)] use ranged_integers::*;
i8::from(ranged![127]);
```


```compile_fail
# #![feature(const_if_match)] #![feature(const_panic)] use ranged_integers::*;
i8::from(ranged![128]);
```


```
# #![feature(const_if_match)] #![feature(const_panic)] use ranged_integers::*;
RDiv(ranged![[0 100] 20], ranged![[1 100] 20]).div();
RDiv(ranged![[0 100] 20], ranged![[-100 -1] -20]).div();
```


```compile_fail
# #![feature(const_if_match)] #![feature(const_panic)] use ranged_integers::*;
RDiv(ranged![[0 100] 20], ranged![[0 100] 20]).div();
```


```compile_fail
# #![feature(const_if_match)] #![feature(const_panic)] use ranged_integers::*;
RDiv(ranged![[0 100] 20], ranged![[-100 0] -20]).div();
```


```compile_fail
# #![feature(const_if_match)] #![feature(const_panic)] use ranged_integers::*;
RDiv(ranged![[0 100] 20], ranged![[-100 100] 20]).div();
```


```
# #![feature(const_if_match)] #![feature(const_panic)] use ranged_integers::*;
Ranged::<0,1>::new(1);
```


```compile_fail
# #![feature(const_if_match)] #![feature(const_panic)] use ranged_integers::*;
Ranged::<1,0>::new(1);
```


```
# #![feature(const_if_match)] #![feature(const_panic)] use ranged_integers::*;
let x: Ranged::<0,1> = Ranged::<0,1>::new(1).unwrap();
```


```compile_fail
# #![feature(const_if_match)] #![feature(const_panic)] use ranged_integers::*;
let x: Ranged::<0,1> = Ranged::new(1).unwrap();
```

*/
struct Failtests;
