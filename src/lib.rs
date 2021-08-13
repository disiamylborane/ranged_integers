//! # Ranged integers [nightly only]
//!
//! The crate provides [an integer type](struct.Ranged.html) restricted to a compile time defined range with
//! automatic size selection and automatic bounds calulation for arithmetics.
//!
//! # Prerequisites
//!
//! The library usage requires the following Rust features enabled in the user crate or application:
//!
//! ```
//! // Without this rustc generates errors and sometimes panics.
//! #![feature(const_generics, const_evaluatable_checked)]
//! ```
//! 
//! # Usage and examples
//! 
//! ## Integer size
//!
//! The [Ranged] automatically chooses the smallest size possible according
//! to `MIN..=MAX` range.
//! It supports i8, u8, i16, u16, i32, u32, i64, u64 and i128 layouts (u128 is not supported),
//! and a special zero-size layout for "constant" values with `MIN==MAX`.
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
//! The implementation heavily relies on the optimizer.
//!
//! ## Ranged and primitive interaction
//!
//! Use `Ranged<MIN, MAX>` type to be sure of the value range:
//!
//! ```
//! # use ranged_integers::*;
//! fn move_player(dice_roll: Ranged<1, 6>) {
//!     let x : i32 = dice_roll.into(); // Conversion is allowed, i32 can store 1..=6
//! }
//! ```
//!
//! ### Create Ranged at compile time
//!
//! The [`Ranged::create_const`] can be used to create a 
//! [`Ranged`] value checking it at compile time.
//! The macro [`r!`] does the same but a bit shorter.
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
//! ```
//!
//! It fails if the bounds are corrupted:
//!
//! ```compile_fail
//! # #![feature(const_generics)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! move_player(r!([] 7)); // Error: Can't store 7 in [1 6] inverval
//! ```
//! ```compile_fail
//! move_player(r!([1 7] 7)); // Error: type mismatch, move_player() requires Ranged<1, 6>
//! ```
//!
//! ### `Ranged` -> `Ranged` conversion
//!
//! The `Ranged` can be converted to the type with different bounds using 
//! [`expand()`](struct.Ranged.html#method.expand) generic method (compile-time check)
//! and [`try_expand()->Option`](struct.Ranged.html#method.try_expand) (runtime check).
//!
//! ```
//! # #![feature(const_generics)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! let expandable: Ranged<4, 5> = r!([] 5);  // Fits Ranged<1,6> accepted by move_player
//! let overlapping: Ranged<4, 9> = r!([] 5);  // Doesn't fit, but the value 5 is acceptable
//! move_player(expandable.expand());
//! move_player(overlapping.try_expand().unwrap());
//! ```
//!
//! Shrinking with `expand()` is forbidden:
//!
//! ```compile_fail
//! # #![feature(const_generics)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! # let overlapping: Ranged<4, 9> = r!([] 5);
//! move_player(overlapping.expand());  // Error: the bounds 4..=9 can't fit in 1..=6
//! ```
//!
//! ### `int` -> `Ranged` conversion
//!
//! Way 1: ensure the bounds with [`Ranged::new(i128) -> Option<Ranged>`](struct.Ranged.html#method.new) function
//!
//! ```
//! # #![feature(const_generics)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! let some_int = 4;
//! let some_wrong_int = 8;
//! assert!(Ranged::<0, 6>::new(some_int) == Some(r!([0 6] 4)));
//! assert!(Ranged::<0, 6>::new(some_wrong_int) == None);
//!
//! move_player(Ranged::new(some_int).unwrap());
//! ```
//!
//! Way 2: use the [`Remainder operation`](struct.Ranged.html#impl-Rem<Ranged<VAL%2C%20VAL>>) with the "const" divisor
//!
//! ```
//! # #![feature(const_generics)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! let x: Ranged<-9, 9> = 15_i32 % r!(10);
//! let y: Ranged<0, 9> = 15_u32 % r!(10);
//! assert!(x == r!(5));
//! assert!(y == r!(5));
//! ```
//!
//! Way 3: Convert the primitive types to `Ranged` with their native bounds using [`AsRanged`]
//!
//! ```
//! # #![feature(const_generics)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
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
//! # #![feature(const_generics)] use ranged_integers::*;
//! let x = r!([0 200] 20);
//! let y: u8 = x.into();  // 0..=200 fits u8
//! ```
//!
//! ```compile_fail
//! # #![feature(const_generics)] use ranged_integers::*;
//! let x = r!([0 200] 20);
//! let y: i8 = x.into();  // 0..=200 doesn't fit i8
//! ```
//!
//! `From` and `Into` operations can't be used in const context.
//! A set of [`const fn`](struct.Ranged.html#method.i8)s allows const conversions to 
//! any integer primitive except for `u128`:
//!
//! ```
//! # #![feature(const_generics)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! let x = r!([0 200] 20);
//! let y = x.u8(); // y is u8
//! let z = x.i16(); // z is i16
//! let w = x.usize(); // w is usize
//! ```
//!
//! ```compile_fail
//! # #![feature(const_generics)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! let x = r!([0 200] 20);
//! let err = x.i8();  // Error: 0..=200 doesn't fit i8
//! ```
//!
//! ## Array indexing
//!
//! The arrays `[T; N]` may be indexed with `Ranged<0, {N-1}>`:
//! ```
//! # #![feature(const_generics)] use ranged_integers::*;
//! let arr = [10, 11, 12, 13, 14];
//! let idx = r!([0 4] 2);
//! assert_eq!(arr[idx], 12);
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
//! The basic arithmetic operations, min() and max() functions are implemented.
//! The bounds of values are automatically recalculated:
//!
//! ```
//! # #![feature(const_generics)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
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
//! ```
//!
//! The division and remainder are allowed only if it's impossible to store "0" in the divisor:
//!
//! ```compile_fail
//! # #![feature(const_generics)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
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
//! # #![feature(const_generics)] use ranged_integers::*;
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
//! # #![feature(const_generics)] use ranged_integers::*; fn move_player(dice_roll: Ranged<1, 6>) {}
//! // In the general case the output is Ranged<1-MAX, MAX-1>, MAX from divisor (by absolute value)
//! let x: Ranged<-9, 9> = (r!([-1000 1000] 500) % r!([1 10] 7));
//! let x: Ranged<-9, 9> = (r!([-1000 1000] 500) % r!([-10 -1] -7));
//! 
//! // If the dividend is nonnegative or nonpositive,
//! // the output range is limited to 0.
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


#![no_std]
#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
#![feature(const_panic)]
#![feature(const_trait_impl)]
#![feature(const_raw_ptr_deref)]
#![feature(specialization)]
#![feature(inline_const)]

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

mod holder;

#[allow(non_camel_case_types)]
type irang = i128;

/// Pick out the "smallest" IntLayout that fits the min..=max range.
/// To be evaluated at compile time.
/// Panics to emit an error when min>max.
#[must_use]
#[doc(hidden)]
pub const fn memlayout(min: irang, max: irang) -> holder::IntLayout {
    macro_rules! layout_variants {
        ($($t:ident)+) => {
            $(   if core::$t::MIN as irang <= min && max <= core::$t::MAX as irang {return holder::IntLayout::$t}   )+
        }
    }
    if min == max {
        return holder::IntLayout::Trivial;
    }
    if min > max {
        panic!("Ranged error: MIN cannot be greater than MAX");
    }
    layout_variants! {i8 u8 i16 u16 i32 u32 i64 u64}
    holder::IntLayout::i128
}

/// A value restricted to the given bounds
#[derive(Clone, Copy)]
pub struct Ranged<const MIN: irang, const MAX: irang>
where [u8; memlayout(MIN, MAX).bytes()]:,
{
    v: holder::NumberBytes<{ memlayout(MIN, MAX) }>,
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
            v: holder::NumberBytes::from_irang(n),
        }
    }

    /// Convert Ranged to a primitive
    const fn get(self) -> irang {
        if MIN == MAX {MIN}
        else {self.v.to_irang()}
    }

    /// Create a Ranged value checking the bounds at runtime
    /// 
    /// # Example
    /// 
    /// ```
    /// # #![feature(const_generics, const_evaluatable_checked)] use ranged_integers::*; 
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
            Some(unsafe { Self::__unsafe_new(n) })
        } else {
            None
        }
    }

    /// Create a Ranged constant checking the bounds at compile time
    /// 
    /// Consider using [`r!`] macro instead
    /// 
    /// # Example
    /// 
    /// ```
    /// # #![feature(const_generics, const_evaluatable_checked)] use ranged_integers::*; 
    /// let a = Ranged::<0, 100>::create_const::<42>();
    /// let a = r!([0 100] 42);
    /// ```
    #[must_use]
    pub const fn create_const<const V: irang>() -> Self
    where Assert<{ OperationPossibility::allow_if(allow_creation(MIN, V, MAX)) }>: IsAllowed,
    {
        unsafe { Self::__unsafe_new(V) }
    }

    /// Iterate up from current value to `Self::MAX` (inclusively) using `Self` as output
    #[must_use]
    pub const fn iter_up(self) -> iter::Iter<MIN, MAX> {
        iter::Iter::<MIN,MAX>{current: Some(self)}
    }
}

mod conversions;
pub use conversions::AsRanged;
mod arithmetics;
mod iter;
pub use iter::range as range;


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
/// // "Constant" value:
/// let c = r!(10);  // Zero-sized Ranged<10, 10> with a value 10
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

#[allow(clippy::cast_sign_loss)]
impl<T, const N: usize> core::ops::Index<Ranged<0, {N as i128 - 1}>> for [T; N]
where 
    [u8; memlayout(0, N as i128 - 1).bytes()]:,
    Assert<{conversions::converter_checkers::usize(0, N as i128 - 1)}>: IsAllowed
{
    type Output = T;
    fn index(&self, index: Ranged<0, {N as i128 - 1}>) -> &Self::Output {
        unsafe{self.get_unchecked(index.usize())}
    }
}

#[allow(clippy::cast_sign_loss)]
impl<T, const N: usize> core::ops::IndexMut<Ranged<0, {N as i128 - 1}>> for [T; N]
where 
    [u8; memlayout(0, N as i128 - 1).bytes()]:,
    Assert<{conversions::converter_checkers::usize(0, N as i128 - 1)}>: IsAllowed
{
    fn index_mut(&mut self, index: Ranged<0, {N as i128 - 1}>) -> &mut Self::Output {
        unsafe{self.get_unchecked_mut(index.usize())}
    }
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
