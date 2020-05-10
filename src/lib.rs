//! # Ranged integers [nightly only]
//! 
//! [`Ranged<MIN, MAX>`](struct.Ranged.html) is an integer-like type that ranges from MIN to MAX **inclusively**.
//! 
//! # Integer size
//!
//! [`Ranged`](struct.Ranged.html) stores a signed value. The type size is automatically adjusted 
//! according to the bounds (maximum 32 bits):
//! ```
//! #![feature(const_if_match)]
//! #![feature(const_panic)]
//! # #[macro_use] extern crate ranged_integers; use ranged_integers::*;
//! 
//! # fn main(){
//! use core::mem::{size_of, align_of};
//! assert_eq!(size_of::<Ranged::<100, 127>>(), 1); // Only i8 is needed to store the value
//! assert_eq!(align_of::<Ranged::<100, 127>>(), 1);
//! assert_eq!(size_of::<Ranged::<100, 128>>(), 2); // Need 16 bits to store +128
//! assert_eq!(align_of::<Ranged::<100, 128>>(), 2);
//! assert_eq!(size_of::<Ranged::<0, 90000>>(), 4); // 4 bytes needed
//! assert_eq!(align_of::<Ranged::<0, 90000>>(), 4);
//! # }
//! ```
//! 
//! # Examples
//! 
//! The library's macro [`ranged!`](macro.ranged.html) requires the following features:
//! ```
//! #![feature(const_if_match)]
//! #![feature(const_panic)]
//! ```
//! 
//! Use `Ranged<MIN, MAX>` as an argument to make the parameter's value compile-time checked:
//! ```
//! # extern crate ranged_integers; use ranged_integers::*;
//! fn move_player(dice_roll: Ranged<1, 6>) {
//!     let x : i32 = dice_roll.get(); // Convert back to int
//! }
//! ```
//! 
//! Create the value at compile-time:
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
//! move_player(ranged!([1 6] 7)); // Can't store 7 in [1 6] inverval
//! move_player(ranged!([1 7] 7)); // Mismatched types, move_player() requires Ranged<1, 6>
//! ```
//! 
//! 
//! A special case with single possible value:
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

#![warn(missing_docs)]

trait Aligner
{
    type A: Copy;
}

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

#[derive(Copy, Clone)]
struct Number<const BYTES: usize>
{
    val: NumberBytes<BYTES>
}

impl<const BYTES: usize> Number<BYTES> {
    #[inline(always)]
    const fn from_i32(v: i32) -> Self {
        unsafe {
            let mut x = Self{val: NumberBytes{bytes: [0; BYTES]}};
            if BYTES == 1 {
                x.val.bytes[0] = v.to_ne_bytes()[0];
            }
            else if BYTES == 2 {
                let v = v.to_ne_bytes();
                x.val.bytes[0] = v[0];
                x.val.bytes[1] = v[1];
            }
            else if BYTES == 4 {
                let v = v.to_ne_bytes();
                x.val.bytes[0] = v[0];
                x.val.bytes[1] = v[1];
                x.val.bytes[2] = v[2];
                x.val.bytes[3] = v[3];
            }

            x
        }
    }
    #[inline(always)]
    const fn to_i32(self) -> i32 {
        unsafe {
            let mut x = 0_i32;
            if BYTES == 1 {
                x = i8::from_ne_bytes([self.val.bytes[0]]) as i32;
            }
            else if BYTES == 2 {
                x = i16::from_ne_bytes([self.val.bytes[0], self.val.bytes[1]]) as i32;
            }
            else if BYTES == 4 {
                x = i32::from_ne_bytes([self.val.bytes[0], self.val.bytes[1], self.val.bytes[2], self.val.bytes[3]]);
            }

            x
        }
    }
}

const fn bytes(val: i32)->usize {
    if (-128 <= val) && (val <= 127) {
        1
    }
    else if (-32768 <= val) && (val <= 32767) {
        2
    }
    else {
        4
    }
}

const fn maxbytes(val1: i32, val2: i32)->usize {
    max_i32(bytes(val1) as i32, bytes(val2) as i32) as usize
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
    v: Number<{maxbytes(MIN, MAX)}>
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
pub fn const_val_i32<const VAL: i32>() -> Ranged<VAL, VAL> {
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
