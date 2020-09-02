//! Ranged: a small wrapper around u8 to make sure
//! of the value bounds.
//! 
//! ```ignore
//! fn consume_ranged(a: Ranged<2,31>) {
//!     // {a} is between 2 and 31 inclusively
//!     let x = a.u8(); // Convert back to int
//! }
//! 
//! // Compile time checking:
//! consume_ranged( ranged!(<2 31> 2) );  // Success
//! consume_ranged( ranged!(<2 31> 1) );  // Compile error: invalid bounds
//! consume_ranged( ranged!(<1 31> 1) );  // Compile error: wrong type
//! 
//! // Runtime checking:
//! let x_input : u8 = user_input_u8();
//! let x = Ranged::try_new(x_input);
//! if let Some(xval) = x {
//!     consume_ranged(xval);
//! }
//! ```

#![no_std]

#![deny(missing_docs)]
#![allow(incomplete_features)]
#![feature(const_generics)]

#![feature(const_fn)]
#![feature(const_panic)]


/// An integer claimed to be in a specified range
#[derive(Clone, Copy)]
pub struct Ranged<const MIN: u8, const MAX: u8>
{
    _val: u8
}



impl<const MIN: u8, const MAX: u8> Ranged<MIN, MAX> {
    /// Use [ranged!()](macro.ranged.html) macro instead
    /// * Returns the constructed integer if `N` satisfies the range;
    /// * Panics otherwise
    #[allow(non_snake_case)]
    pub const unsafe fn __unsafe__(N: u8) -> Self {
        Self{_val: N}
    }
    

    /// Use [ranged!()](macro.ranged.html) macro instead
    /// * Returns the constructed integer if `N` satisfies the range;
    /// * Panics otherwise
    #[allow(non_snake_case)]
    pub const fn __as_const(N: u8) -> Self {
        if !(MIN <= N && N <= MAX) {
            panic!("Ranged integer bounds mismatch")
        }
        Self{_val: N}
    }

    /// Convert an integer into `Ranged`
    pub const fn new(val: u8) -> Option<Self> {
        if MIN <= val && val <= MAX {
            return Some(Self{_val: val});
        }
        return None;
    }

    /// Convert `Ranged` into u8
    pub const fn u8(self) -> u8 {
        self._val
    }

    /// Convert `Ranged` into i16
    pub const fn i16(self) -> i16 {
        self._val as i16
    }

    /// Convert `Ranged` into u16
    pub const fn u16(self) -> u16 {
        self._val as u16
    }
}

/// The compile-time checked constant Ranged value
/// 
/// ```
/// # #[macro_use] extern crate ranged_integers;
/// use ranged_integers::*;
///
/// let d = ranged!([1 4] 2);    // Create a 1..=4 with the value 2
/// let d = ranged!([0 63] 17);  // Create a 0..=63 with the value 17
/// //let w = ranged!(<0 15> 17);  // Compile error: attempt to create a 0..=15 with the value 17
/// ```
#[macro_export]
macro_rules! ranged {
    ($v:literal) => {
        {
            const __Z : Ranged<$v, $v> = Ranged::__as_const($v);
            __Z
        }
    };
    ([$min:literal $max:literal] $x:literal) => {
        {
            const __Z : Ranged<$min, $max> = Ranged::__as_const($x);
            __Z
        }
    };
}


#[test]
fn tst() {
    let x : Ranged<3,5>;
    x = ranged!([3 5] 4);
    assert!(x.u8() == 4);
}


