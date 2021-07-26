//! # Ranged integers [nightly only]
//!
//! The crate provides an integer type restricted to a compile time defined range with
//! automatic size selection and automatic bounds calulation for arithmetics.
//!
//! [`Ranged<const MIN: i128, const MAX: i128>`](struct.Ranged.html) is bounded to [MIN, MAX] interval **inclusively**.
//!
//! # Usage and examples
//!
//! ## Prerequisites
//!
//! The library usage requires the following Rust feature enabled in the user crate or application:
//!
//! ```
//! // Without this the Ranged arithmetics and conversion functions fail.
//! #![feature(const_generics)]
//! ```
//!
//! ## Integer size
//!
//! The [`Ranged`](struct.Ranged.html) automatically chooses the smallest size possible according
//! to `MIN..=MAX` range.
//! It supports i8, u8, i16, u16, i32, u32, i64, u64 and i128 layouts (u128 is not supported).
//!
//! ```
//! # use ranged_integers::*; fn main(){
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
//! The implementation heavily relies on the optimizer. The optimizer... usually doesn't fail.
//!
//! ## Ranged and primitive types semantics
//!
//! Use `Ranged<MIN, MAX>` type to be sure of the value range:
//!
//! ```
//! # use ranged_integers::*;
//! fn move_player(dice_roll: Ranged<1, 6>) {
//!     let x : i32 = dice_roll.into(); // Convert to int
//! }
//! ```
//!
//! ### Create Ranged from primitive at compile time
//!
//! The `Ranged::create_const` can be used to create a [`Ranged`](struct.Ranged.html) value checking it at compile time.
//! The macro [`r!`](macro.r.html) does the same but a bit shorter.
//!
//! ```
//! # #![feature(const_generics)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
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
//! let a: Ranged<4,4> = x;
//! let b: Ranged<4,4> = y;
//! ```
//!
//! It fails if the bounds are corrupted:
//!
//! ```compile_fail
//! # #![feature(const_generics)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! move_player(r!([1 6] 7)); // Error: Can't store 7 in [1 6] inverval
//! ```
//! ```compile_fail
//! move_player(r!([1 7] 7)); // Error: type mismatch, move_player() requires Ranged<1, 6>
//! ```
//!
//! A special case with the single possible value:
//!
//! ```
//! # #![feature(const_generics)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! let x = r!(4); // Means Ranged<4, 4> with the value 4
//! let y: Ranged<4,4> = x;
//! ```
//!
//! ### Create Ranged from another Ranged
//!
//! ```
//! # #![feature(const_generics)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! let fixed_roll = r!(4);
//! move_player(fixed_roll.expand());  // The original bounds 4..=4 are expanded to 1..=6
//! ```
//!
//! Shrinking is forbidden:
//!
//! ```compile_fail
//! # #![feature(const_generics)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! move_player(r!(7).expand());  // Error: the bounds 7..=7 can't fit in 1..=6
//! ```
//!
//! ### Create Ranged from primitive at runtime
//!
//! Way 1: ensure the bounds with `new(i128)->Option<Ranged>` function
//!
//! ```
//! # #![feature(const_generics)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! let some_i32 = 4;
//! let some_wrong_i32 = 8;
//! assert!(Ranged::<0, 6>::new(some_i32).unwrap() == r!(4));
//! assert!(Ranged::<0, 6>::new(some_wrong_i32) == None);
//!
//! move_player(Ranged::new(4).unwrap());
//! ```
//!
//! Way 2: use the remainder operation with the "const" divisor
//!
//! ```
//! # #![feature(const_generics)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! let x: Ranged<-9, 9> = 15_i32 % r!(10);
//! let y: Ranged<0, 9> = 15_u32 % r!(10);
//! assert!(x == r!(5)); // 15 % 10 == 5
//! assert!(y == r!(5)); // 15 % 10 == 5
//! ```
//!
//! ```compile_fail
//! # #![feature(const_generics)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! let x = 15 % r!(10);
//! let y: Ranged<0, 20> = x;  // Error: x is Ranged<-9, 9>, the interval -9..=-1 doesn't fit
//! ```
//!
//! Way 3: Convert the primitive types to `Ranged` with their native bounds
//!
//! ```
//! # #![feature(const_generics)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! use ranged_integers::AsRanged;
//! let x = 15_u8.as_ranged();  // Ranged<0, 255>
//! let y = 15_i16.as_ranged(); // Ranged<-32768, 32767>
//! ```
//!
//! ### Cast Ranged to primitives
//!
//! Casting to integer types is allowed when the value is proved to
//! fit into the result type:
//!
//! ```
//! # #![feature(const_generics)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! let x = r!([0 200] 20);
//! assert_eq!(20_u8, x.into());
//! ```
//!
//! ```compile_fail
//! # #![feature(const_generics)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! let x = r!([0 200] 20);
//! assert_eq!(20_i8, x.into()); // Error: can't fit the range 128..=200 in i8
//! ```
//!
//! There is also a set of `const` casting functions:
//!
//! ```
//! # #![feature(const_generics)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! let x = r!([0 200] 20);
//! let y = x.u8(); // y is u8
//! let z = x.i16(); // z is i16
//! ```
//!
//! ```compile_fail
//! # #![feature(const_generics)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! let x = r!([0 200] 20);
//! let err = x.i8();  // Error: 0..=200 doesn't fit i8
//! ```
//!
//! ## Comparison
//!
//! Equality and inequality operations between different Ranged types are allowed:
//!
//! ```
//! # #![feature(const_generics)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! assert!(r!([1 6] 4) == r!([1 10] 4));
//! assert!(r!([1 6] 4) != r!([1 6] 5));
//! ```
//!
//! ## Arithmetics
//!
//! Currently addition, subtraction, multiplication, division and negation operations,
//! min() and max() functions are implemented.
//! The bounds of values are automatically recalculated:
//!
//! ```
//! # #![feature(const_generics)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! let x = r!([1 6] 4);
//! let y = r!([1 6] 5);
//!
//! let a = x + y;  // The minimum is (1+1)=2, the maximum is (6+6)=12
//! let check_add: Ranged<2, 12> = a;  // Assertion assignment
//!
//! let s = x - y;  // The minimum is (1-6)=-5, the maximum is (6-1)=5
//! let check_sub: Ranged<-5, 5> = s;  // Assertion assignment
//!
//! let m = x * y;  // The minimum is (1*1)=1, the maximum is (6*6)=36
//! let check_mul: Ranged<1, 36> = m;  // Assertion assignment
//!
//! let d = x / y;  // The minimum is (1/6)=0, the maximum is (6/1)=6
//! let check_div: Ranged<0, 6> = d;  // Assertion assignment
//!
//! let n = -x;
//! let check_neg: Ranged<-6, -1> = n;  // Assertion assignment
//! 
//! let min: Ranged<1,6> = x.min(a);  // x.min(a) is never less than 1 and greater than 6
//! let max: Ranged<2,12> = x.max(a); // x.max(a) is never less than 2 and greater than 12
//! ```
//!
//! The division is allowed only if it's impossible to store "0" in the divisor:
//!
//! ```compile_fail
//! # #![feature(const_generics)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! let x = r!([1 6] 4);
//! let y = r!([0 6] 5);
//! let z = r!([-1 6] 5);
//!
//! let d = x / y; // Error: division is not possible
//! let e = x / z; // Error: division is not possible
//! ```
//!
//! The `Rem` operation is unstable, the better bound calculator is upcoming:
//! ```
//! # #![feature(const_generics)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! let x: Ranged<0, 9> = r!([0 100] 15) % r!(10);
//! let y: Ranged<5, 5> = r!(15) % r!(10);
//! ```

#![no_std]
#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
#![feature(const_panic)]
#![feature(const_trait_impl)]
#![feature(const_raw_ptr_deref)]
#![feature(specialization)]

#![deny(missing_docs)]
#![deny(clippy::nursery)]
#![warn(clippy::pedantic)]

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(test)]
mod tests;

#[doc(hidden)]
pub mod value_check;
use value_check::{Assert, IsAllowed, OperationPossibility};

pub mod holder;
use holder::{IntLayout, NumberBytes};

mod conversions;
mod arithmetics;

pub use conversions::AsRanged;

#[allow(non_camel_case_types)]
type irang = i128;

/// Pick out the "smallest" IntLayout that fits the min..=max range.
/// To be evaluated at compile time.
/// Panics to emit an error when min>max.
#[must_use]
#[doc(hidden)]
pub const fn memlayout(min: irang, max: irang) -> IntLayout {
    macro_rules! crange {
        ($($t:ident)+) => {
            $(
                if core::$t::MIN as irang <= min && max <= core::$t::MAX as irang {return IntLayout::$t}
            )+
        }
    }

    if min == max {
        return IntLayout::Trivial;
    }
    if min > max {
        panic!("Ranged error: MIN cannot be greater than MAX");
    }
    crange! {i8 u8 i16 u16 i32 u32 i64 u64}
    IntLayout::i128
}

/// A value restricted to the given bounds
#[derive(Clone, Copy)]
pub struct Ranged<const MIN: irang, const MAX: irang>
where
    [u8; memlayout(MIN, MAX).bytes()]: ,
{
    v: NumberBytes<{ memlayout(MIN, MAX) }>,
}

#[must_use]
#[doc(hidden)]
pub const fn allow_creation(min: irang, v: irang, max: irang) -> bool {
    min <= v && v <= max
}

impl<const MIN: irang, const MAX: irang> Ranged<MIN, MAX>
where
    [u8; memlayout(MIN, MAX).bytes()]: ,
{
    const unsafe fn __unsafe_new(n: irang) -> Self {
        Self {
            v: NumberBytes::from_irang(n),
        }
    }

    /// Create a Ranged value with a runtime bounds checking
    #[must_use]
    pub const fn new(n: irang) -> Option<Self> {
        if (MIN <= n) && (n <= MAX) {
            Some(unsafe { Self::__unsafe_new(n) })
        } else {
            None
        }
    }

    /// Create a Ranged value with a compile time bounds checking
    #[must_use]
    pub const fn create_const<const V: irang>() -> Self
    where
        Assert<{ OperationPossibility::allow_if(allow_creation(MIN, V, MAX)) }>: IsAllowed,
    {
        unsafe { Self::__unsafe_new(V) }
    }

    /// Convert Ranged to a primitive
    const fn get(self) -> irang {
        if MIN == MAX {MIN}
        else {self.v.to_irang()}
    }
}

/// Create a ranged value at compile time
///
/// **Warning**: ensure `#![feature(const_generics)]` is enabled.
///
/// # Example
///
/// ```
/// # #![feature(const_generics)] use ranged_integers::*;
/// // Explicit bounds:
/// let a = r!([0 42] 23);  // Ranged<0, 42> with a value 23
/// // Type inference:
/// let b: Ranged<0, 100> = r!([] 42);  // Ranged<0, 100> with a value 42
/// // "Constant" value
/// let c = r!(10);  // Ranged<10, 10> with a value 10
/// ```
#[macro_export]
macro_rules! r {
    ([$min:literal $max:literal] $x:expr) => {
        $crate::Ranged::<$min, $max>::create_const::<$x>()
    };
    ([] $v:expr) => {
        $crate::Ranged::create_const::<$v>()
    };
    ($v:expr) => {
        $crate::Ranged::<$v, $v>::create_const::<$v>()
    };
}

impl<const MIN: irang, const MAX: irang> core::fmt::Display for Ranged<MIN, MAX>
where [(); memlayout(MIN, MAX).bytes()]: ,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl<const MIN: irang, const MAX: irang> core::fmt::Debug for Ranged<MIN, MAX>
where [(); memlayout(MIN, MAX).bytes()]: ,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if MIN == MAX {
            write!(f, "r!({})", MIN)
        } else {
            write!(f, "r!([{} {}] {})", MIN, MAX, self.get())
        }
    }
}

#[doc(hidden)] #[must_use]
pub const fn min_irang(x: irang, y: irang) -> irang {
    if x < y {x} else {y}
}
#[doc(hidden)] #[must_use]
pub const fn max_irang(x: irang, y: irang) -> irang {
    if x > y {x} else {y}
}



#[allow(dead_code)]
#[doc(hidden)]
/**
```
# #![feature(const_generics)] #![feature(const_panic)] use ranged_integers::*;
u8::from(r!(0));
```

```compile_fail
# #![feature(const_generics)] #![feature(const_panic)] use ranged_integers::*;
u8::from(r!(-1));
```

```
# #![feature(const_generics)] #![feature(const_panic)] use ranged_integers::*;
u8::from(r!(255));
```

```compile_fail
# #![feature(const_generics)] #![feature(const_panic)] use ranged_integers::*;
u8::from(r!(256));
```

```
# #![feature(const_generics)] #![feature(const_panic)] use ranged_integers::*;
i8::from(r!(-128));
```


```compile_fail
# #![feature(const_generics)] #![feature(const_panic)] use ranged_integers::*;
i8::from(r!(-129));
```

```
# #![feature(const_generics)] #![feature(const_panic)] use ranged_integers::*;
i8::from(r!(127));
```


```compile_fail
# #![feature(const_generics)] #![feature(const_panic)] use ranged_integers::*;
i8::from(r!(128));
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
