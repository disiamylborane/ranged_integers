//! # Ranged integers [nightly only]
//! 
//! [`Ranged<MIN, MAX>`](struct.Ranged.html) is an integer-like type that ranges from MIN to MAX **inclusively**.
//! 
//! # Integer size
//!
//! The [`Ranged`](struct.Ranged.html) automatically chooses between signed/unsigned
//! and the data type (maximum signed 32 bits):
//! ```
//! #![feature(const_if_match)]
//! #![feature(const_panic)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! 
//! # fn main(){
//! use core::mem::{size_of, align_of};
//! 
//! assert_eq!(size_of::<Ranged::<-100, 127>>(), 1); // The range fits i8
//! assert_eq!(align_of::<Ranged::<-100, 127>>(), 1);
//! assert_eq!(size_of::<Ranged::<0, 200>>(), 1); // The range fits u8
//! assert_eq!(align_of::<Ranged::<0, 200>>(), 1);
//! assert_eq!(size_of::<Ranged::<-100, 200>>(), 2); // The range fits i16
//! assert_eq!(align_of::<Ranged::<-100, 200>>(), 2);
//! assert_eq!(size_of::<Ranged::<0, 90000>>(), 4); // i32 is needed
//! assert_eq!(align_of::<Ranged::<0, 90000>>(), 4);
//! # }
//! ```
//! 
//! # Examples
//! 
//! The library's macro [`ranged!`](macro.ranged.html) requires the following features enabled:
//! ```
//! #![feature(const_if_match)]
//! #![feature(const_panic)]
//! ```
//! 
//! Use `Ranged<MIN, MAX>` as an argument to ensure the parameter range at compile-time.
//!
//! Release i32 from Ranged with `Ranged::get()`:
//!
//! ```
//! # extern crate ranged_integers; use ranged_integers::*;
//! fn move_player(dice_roll: Ranged<1, 6>) {
//!     let x : i32 = dice_roll.get(); // Convert back to int
//! }
//! ```
//! 
//! Create the value at compile-time with `ranged!([MIN MAX] VALUE)`:
//! ```
//! # #![feature(const_if_match)] #![feature(const_panic)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*; 
//! # fn move_player(dice_roll: Ranged<1, 6>) {
//! #     let x : i32 = dice_roll.get(); // Convert back to int
//! # }
//! move_player(ranged!([1 6] 4));
//! ```
//! 
//! It fails if the bounds are corrupted:
//! ```compile_fail
//! # #![feature(const_if_match)] #![feature(const_panic)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*; 
//! # fn move_player(dice_roll: Ranged<1, 6>) {
//! #     let x : i32 = dice_roll.get(); // Convert back to int
//! # }
//! move_player(ranged!([1 6] 7)); // Error: Can't store 7 in [1 6] inverval
//! move_player(ranged!([1 7] 7)); // Error: Mismatched types, move_player() requires Ranged<1, 6>
//! ```
//! 
//! 
//! A special case with the single possible value:
//! ```
//! # #![feature(const_if_match)] #![feature(const_panic)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*; 
//! let x = ranged![4]; // Means Ranged<4, 4> with the value 4
//! let y: Ranged<4,4> = x;
//! ```
//! 
//! 
//! Comparison between different types is allowed:
//! ```
//! # #![feature(const_if_match)] #![feature(const_panic)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! #
//! assert!(ranged!([1 6] 4) == ranged!([1 10] 4));
//! assert!(ranged!([1 6] 4) != ranged!([1 6] 5));
//! ```
//! 
//! 
//! Ensure the bounds at runtime:
//! ```
//! # #![feature(const_if_match)] #![feature(const_panic)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*; 
//! let some_i32 = 4;
//! let some_wrong_i32 = 8;
//! assert!(Ranged::<0, 6>::new(some_i32).unwrap() == ranged![4]);
//! assert!(Ranged::<0, 6>::new(some_wrong_i32) == None);
//! ```
//! 
//! Use remainder operation with the const divisor as a way to create Ranged:
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
//! 
//! Arithmetics: the bounds are automatically recalculated
//! ```
//! # #![feature(const_if_match)] #![feature(const_panic)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*; 
//! let x = ranged![4];
//! assert!(x+x == ranged![8]);
//! assert!(x-x == ranged![0]);
//! assert!(x*ranged![2] == ranged![8]);
//! ```
//! 
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


trait Aligner
{
    type A: Copy;
}

// BUG: using IntLayout in AlignWrap causes stack overflow
#[derive(Copy, Clone)]
struct AlignWrap<const N: usize>;

impl<const N: usize> Aligner for AlignWrap<N> {
    default type A = i32;
}

impl Aligner for AlignWrap<1> {
    type A = i8;
}

impl Aligner for AlignWrap<2> {
    type A = i16;
}


#[derive(Copy, Clone)]
#[repr(C)]
union NumberBytes<const BYTES: usize> {
    val: <AlignWrap<BYTES> as Aligner>::A,
    bytes: [u8; BYTES],
}

impl<const BYTES: usize> NumberBytes<BYTES> {
    const fn new() -> Self {
        Self{bytes: [0; BYTES]}
    }
}

#[derive(Copy, Clone)]
struct Number<const LAYOUT: IntLayout>
{
    val: NumberBytes<{LAYOUT.bytes()}>
}

impl<const LAYOUT: IntLayout> Number<LAYOUT> {
    #[inline(always)]
    const fn from_i32(v: i32) -> Self {
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
            }

            x
        }
    }
    #[inline(always)]
    const fn to_i32(self) -> i32 {
        unsafe {
            match LAYOUT {
                IntLayout::i8 => {
                    i8::from_ne_bytes([self.val.bytes[0]]) as i32
                }
                IntLayout::u8 => {
                    u8::from_ne_bytes([self.val.bytes[0]]) as i32
                }
                IntLayout::i16 => {
                    i16::from_ne_bytes([self.val.bytes[0], self.val.bytes[1]]) as i32
                }
                IntLayout::u16 => {
                    u16::from_ne_bytes([self.val.bytes[0], self.val.bytes[1]]) as i32
                }
                IntLayout::i32 => {
                    i32::from_ne_bytes([self.val.bytes[0], self.val.bytes[1], self.val.bytes[2], self.val.bytes[3]])
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
#[allow(non_camel_case_types)]
enum IntLayout {
    i8, u8, i16, u16, i32
}

const fn memlayout(min: i32, max: i32) -> IntLayout {
    if -128 <= min && max <= 127 {
        IntLayout::i8
    }
    else if 0<=min && max<=255 {
        IntLayout::u8
    }
    else if -32768 <= min && max <= 32767 {
        IntLayout::i16
    }
    else if 0<=min && max<=65535 {
        IntLayout::u16
    }
    else {
        IntLayout::i32
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
        }
    }
}


/// Create a ranged value at compile-time:
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
/// It will fail if the bound checking fails
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
        const_val_i32::<{$v}>()
    };
}


/// A value in MIN..=MAX range
#[derive(Copy, Clone)]
pub struct Ranged<const MIN: i32, const MAX: i32>
{
    v: Number<{memlayout(MIN, MAX)}>
}


impl<const MIN: i32, const MAX: i32> core::fmt::Display for Ranged<MIN,MAX> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.get())
    }
}


impl<const MIN: i32, const MAX: i32> core::fmt::Debug for Ranged<MIN,MAX> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Ranged<{}, {}> {{ _val: {} }}", MIN, MAX, self.get())
    }
}


impl<const MIN: i32, const MAX: i32> Ranged<MIN, MAX> {
    #[doc(hidden)]
    /// Create the value without bounds checking
    ///
    /// For compile-time values use safe [ranged!](macro.ranged.html) macro instead
    pub const unsafe fn __unsafe_new(n: i32) -> Self {
        Ranged{v: Number::from_i32(n)}
    }

    /// Check and create the value from i32
    pub const fn new(n: i32) -> Option<Self> {
        if (MIN <= n) && (n <= MAX) {
            Some( unsafe {Self::__unsafe_new(n)} )
        }
        else {
            None
        }
    }

    /// Get the value as i32
    pub const fn get(self) -> i32 {
        self.v.to_i32()
    }
}


#[doc(hidden)]
/// Create a fixed-valued Ranged at compile time
pub const fn const_val_i32<const VAL: i32>() -> Ranged<VAL, VAL> {
    unsafe {Ranged::__unsafe_new(VAL)}
}


const fn min_i32(x: i32, y: i32) -> i32 {
    if x<y {x} else {y}
}
const fn max_i32(x: i32, y: i32) -> i32 {
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


impl<const VAL: i32> core::ops::Rem<Ranged<VAL, VAL>> for i32 {
    type Output = Ranged<0, {VAL.abs()-1}>;

    fn rem(self, _rhs: Ranged<VAL, VAL>) -> Self::Output {
        unsafe { Ranged::__unsafe_new(self % VAL) }
    }
}

impl<const AMIN: i32, 
     const AMAX: i32,
     const BMIN: i32, 
     const BMAX: i32> core::ops::Add<Ranged<BMIN, BMAX>> for Ranged<AMIN, AMAX> {

        type Output = Ranged<{AMIN+BMIN}, {AMAX+BMAX}>;

        fn add(self, rhs: Ranged<BMIN, BMAX>) -> Self::Output {
            unsafe {Self::Output::__unsafe_new(self.get() + rhs.get())}
        }
     }


impl<const AMIN: i32, 
     const AMAX: i32,
     const BMIN: i32, 
     const BMAX: i32> core::ops::Sub<Ranged<BMIN, BMAX>> for Ranged<AMIN, AMAX> {

        type Output = Ranged<{AMIN-BMIN}, {AMAX-BMAX}>;

        fn sub(self, rhs: Ranged<BMIN, BMAX>) -> Self::Output {
            unsafe {Self::Output::__unsafe_new(self.get() - rhs.get())}
        }
    }


const fn cross_mul(amin: i32, amax: i32, bmin: i32, bmax: i32)-> (i32,i32,i32,i32) {
    (amin*bmin, amin*bmax, amax*bmin, amax*bmax)
}

const fn max_4(vals: (i32,i32,i32,i32))->i32 {
    reduce!(max_i32, vals.0, vals.1, vals.2, vals.3)
}
const fn min_4(vals: (i32,i32,i32,i32))->i32 {
    reduce!(min_i32, vals.0, vals.1, vals.2, vals.3)
}



impl<const AMIN: i32, 
     const AMAX: i32,
     const BMIN: i32, 
     const BMAX: i32> core::ops::Mul<Ranged<BMIN, BMAX>> for Ranged<AMIN, AMAX> {

        type Output = Ranged<{min_4(cross_mul(AMIN, AMAX, BMIN, BMAX))}, 
                             {max_4(cross_mul(AMIN, AMAX, BMIN, BMAX))}>;

        fn mul(self, rhs: Ranged<BMIN, BMAX>) -> Self::Output {
            unsafe {Self::Output::__unsafe_new(self.get() * rhs.get())}
        }
    }



impl<const AMIN: i32, 
    const AMAX: i32,
    const BMIN: i32, 
    const BMAX: i32> core::cmp::PartialEq<Ranged<BMIN, BMAX>> for Ranged<AMIN, AMAX> {

        fn eq(&self, rhs: &Ranged<BMIN, BMAX>) -> bool {
            self.get() == rhs.get()
        }
    }

impl<const AMIN: i32, 
     const AMAX: i32> core::cmp::Eq for Ranged<AMIN, AMAX>
         {}


#[cfg(test)]
extern crate alloc;

#[cfg(test)]
mod tests;

