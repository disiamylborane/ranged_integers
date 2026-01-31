// Contains the arithmetic functions, comparison and order traits
// The main feature of arithmetic operations is automatic bounds calculation

use crate::{Assert, IsAllowed, OperationPossibility, Ranged, allow_range, irang, memlayout, value_check::allow_if};

// UNARY MINUS

// The operation gets a negative from Ranged
impl<const MIN: irang, const MAX: irang>  // The Ranged contains 2 const generic parameters
core::ops::Neg
for Ranged<MIN, MAX>
where
    // This constraint is required because the operand requires it (it is specified in Ranged structure).
    Assert<{allow_range(memlayout(MIN, MAX))}>: IsAllowed,
    // This constraint is required because the Output type requires it.
    Assert<{allow_range(memlayout(-MAX, -MIN))}>: IsAllowed,
{
    // if we invert a number in A..=B interval, we get the number in (-B)..=(-A) interval.
    type Output = Ranged<{ -MAX }, { -MIN }>;
    fn neg(self) -> Self::Output {Self::neg(self)}
}


// BINARY OPERATION TRAITS

impl<const AMIN: irang,  // Arithmetic operations take 2 variables, each have MIN and
     const AMAX: irang,  // MAX bounds. Any 2 Ranged may be added till the underlying
     const BMIN: irang,  // type can contain the whole range of the output values. We need
     const BMAX: irang>  // 4 compile-time variables to store the bounds for addition.
core::ops::Add<Ranged<BMIN, BMAX>>
for Ranged<AMIN, AMAX>
where
    // This constraint is required because the first argument (self) contains it.
    Assert<{allow_range(memlayout(AMIN, AMAX))}>: IsAllowed,
    // This constraint is required because the second argument (rhs) contains it.
    Assert<{allow_range(memlayout(BMIN, BMAX))}>: IsAllowed,
    // This constraint is required because the result contains it. Also
    // this is a check for overflow: if we add numbers too large for Ranged, it will generate the error
    Assert<{allow_range(memlayout(AMIN + BMIN, AMAX + BMAX))}>: IsAllowed,
{
    // There is a math proof: for any integers in ranges AMIN..=AMAX and BMIN..=BMAX, the
    // result is within the (AMIN + BMIN)..=(AMAX + BMAX) range. We do not prove it here,
    // instead we claim it.
    type Output = Ranged<{ AMIN + BMIN }, { AMAX + BMAX }>;

    fn add(self, rhs: Ranged<BMIN, BMAX>) -> Self::Output {
        // There is a const version of this function, call it
        Self::add(self, rhs)
    }
}


// Refer to Add trait for detailed comments
impl<const AMIN: irang, const AMAX: irang, const BMIN: irang, const BMAX: irang>
core::ops::Sub<Ranged<BMIN, BMAX>> for Ranged<AMIN, AMAX>
where
    Assert<{allow_range(memlayout(AMIN, AMAX))}>: IsAllowed,
    Assert<{allow_range(memlayout(BMIN, BMAX))}>: IsAllowed,
    Assert<{allow_range(memlayout(AMIN - BMAX, AMAX - BMIN))}>: IsAllowed,
{
    type Output = Ranged<{ AMIN - BMAX }, { AMAX - BMIN }>;

    fn sub(self, rhs: Ranged<BMIN, BMAX>) -> Self::Output {Self::sub(self, rhs) }
}


// The next items are helpers for Mul trait. The arrays are flawed in the const
// contexts, so, such operations as 'find max of 4 items' should be handled recursively.
// We use the helper macro `reduce!` for it.


macro_rules! reduce {
    ($fn:ident, $a:expr) => ( $a );
    ($fn:ident, $a:expr, $($args:expr),+) => {
        {
            $fn($a, reduce!($fn, $($args),+ ))
        }
    };
}

// Find minimum of 2 numbers
#[doc(hidden)] #[must_use]
pub const fn min_irang(x: irang, y: irang) -> irang {
    if x < y {x} else {y}
}
// Find maximum of 2 numbers
#[doc(hidden)] #[must_use]
pub const fn max_irang(x: irang, y: irang) -> irang {
    if x > y {x} else {y}
}

// Find maximum of 4 numbers
const fn max_4(vals: (irang, irang, irang, irang)) -> irang {
    reduce!(max_irang, vals.0, vals.1, vals.2, vals.3)
}
// Find minimum of 4 numbers
const fn min_4(vals: (irang, irang, irang, irang)) -> irang {
    reduce!(min_irang, vals.0, vals.1, vals.2, vals.3)
}

// These functions must be public, because they are used in public interface, namely the return
// type of Mul trait. However, the users should not use these functions, so they are taken away
// from the documentation.

#[must_use] #[doc(hidden)]
pub const fn max_cross(a_min: irang, a_max: irang, b_min: irang, b_max: irang) -> irang {
    max_4((a_min * b_min, a_min * b_max, a_max * b_min, a_max * b_max))
}
#[must_use] #[doc(hidden)]
pub const fn min_cross(a_min: irang, a_max: irang, b_min: irang, b_max: irang) -> irang {
    min_4((a_min * b_min, a_min * b_max, a_max * b_min, a_max * b_max))
}

// The bounds calculation for multiplication is harder than in add/sub operations in signed integers,
// because minus times minus is plus. So, decreasing the operand may increase the result.
// However, the multiplication is monotonical over both the operands. So, to find the bounds of the
// result, we should just compare four cases: min*min, min*max, max*min and max*max. One of these cases
// will be lower bound, the other will be upper.

impl<const AMIN: irang, const AMAX: irang, const BMIN: irang, const BMAX: irang>
    core::ops::Mul<Ranged<BMIN, BMAX>> for Ranged<AMIN, AMAX>
where
    Assert<{allow_range(memlayout(AMIN, AMAX))}>: IsAllowed,
    Assert<{allow_range(memlayout(BMIN, BMAX))}>: IsAllowed,
    Assert<{allow_range(memlayout(min_cross(AMIN, AMAX, BMIN, BMAX), max_cross(AMIN, AMAX, BMIN, BMAX)))}>: IsAllowed,
{
    type Output =
        Ranged<{ min_cross(AMIN, AMAX, BMIN, BMAX) }, { max_cross(AMIN, AMAX, BMIN, BMAX) }>;

    fn mul(self, rhs: Ranged<BMIN, BMAX>) -> Self::Output {Self::mul(self, rhs)}
}


// Division by zero is forbidden. Division by Ranged including zero is forbidden. This function
// defines, whether the division by Ranged<b_min, b_max> is allowed. It is used in Div and Rem
// operations
#[must_use]
#[doc(hidden)]
pub const fn allow_division(b_min: irang, b_max: irang) -> OperationPossibility {
    if ((b_min > 0) && (b_max > 0)) || ((b_min < 0) && (b_max < 0)) {
        OperationPossibility::Allowed
    } else {
        OperationPossibility::Forbidden
    }
}

// Calculates lower division bound IF b_min and b_max have the same sign (they must for division to be allowed)
#[must_use]
#[doc(hidden)]
pub const fn singleside_div_min(a_min: irang, a_max: irang, b_min: irang, b_max: irang) -> irang {
    min_4((a_min / b_min, a_min / b_max, a_max / b_min, a_max / b_max))
}
// Calculates upper division bound IF b_min and b_max have the same sign (they must for division to be allowed)
#[must_use]
#[doc(hidden)]
pub const fn singleside_div_max(a_min: irang, a_max: irang, b_min: irang, b_max: irang) -> irang {
    max_4((a_min / b_min, a_min / b_max, a_max / b_min, a_max / b_max))
}

// Calculation bounds for division is not easy because it is non-monotonic over the second operand.
// However, while the second operand preserves the sign, the result changes monotonically. And the division
// is forbidden for the cases when the sign is changed, because it means the possibility to divide by 0.
// So, we check that the 0 is not contained in range, and use the monotonic min-max check to calculate
// the bounds
impl<const AMIN: irang, const AMAX: irang, const BMIN: irang, const BMAX: irang>
    core::ops::Div<Ranged<BMIN, BMAX>> for Ranged<AMIN, AMAX>
where
    Assert<{allow_range(memlayout(AMIN, AMAX))}>: IsAllowed, // Constraint for operand 1
    Assert<{allow_range(memlayout(BMIN, BMAX))}>: IsAllowed, // Constraint for operand 2
    // Constraint for result
    Assert<{allow_range(memlayout(singleside_div_min(AMIN, AMAX, BMIN, BMAX), singleside_div_max(AMIN, AMAX, BMIN, BMAX)))}>: IsAllowed,
    // The additional requirement: the 2nd operand must not be able to contain 0
    Assert<{ allow_division(BMIN, BMAX) }>: IsAllowed,
{
    type Output = Ranged<{singleside_div_min(AMIN, AMAX, BMIN, BMAX) }, {singleside_div_max(AMIN, AMAX, BMIN, BMAX) }>;

    fn div(self, rhs: Ranged<BMIN, BMAX>) -> Self::Output { Self::div(self, rhs) }
}

// The hardest part of arithmetics. Calculating the Rem operation bounds

#[must_use]
#[doc(hidden)]
pub const fn singleside_rem_min(a_min: irang, a_max: irang, b_min: irang, b_max: irang) -> irang {
    // Note that b_min..=b_max must never include 0
    if b_min == b_max { // Special cases if we are taking remainder with constant (which is typical)
        if a_min == a_max {return a_min % b_min} // just 2 consts
        else if a_min > 0 {
            // The dividend is positive, the remainder is positive
            let base = a_max / b_min.abs() * b_min.abs();
            if a_min >= base {
                return a_min % b_min;
            }
        }
        else if a_max < 0 {
            // The dividend is negative, the remainder is negative
            let base = a_min / b_min.abs() * b_min.abs();
            if a_max <= base {
                return a_min % b_min;
            }
        }
    }

    if a_min >= 0 {
        0 // The dividend is positive, this is 0..=some_value case
    } 
    else {
        // The dividend is positive, this is -some_value..=0 case
        max_irang(1 - max_irang(b_max.abs(), b_min.abs()), a_min)
    }
}


#[must_use]
#[doc(hidden)]
pub const fn singleside_rem_max(a_min: irang, a_max: irang, b_min: irang, b_max: irang) -> irang {
    if b_min == b_max {
        if a_min == a_max {return a_min % b_min}
        else if a_min > 0 {
            let base = a_max / b_min.abs() * b_min.abs();
            if a_min >= base {
                return a_max % b_min;
            }
        }
        else if a_max < 0 {
            let base = a_min / b_min.abs() * b_min.abs();
            if a_max <= base {
                return a_max % b_min;
            }
        }
    }

    if a_max <= 0 {0}
    else {
        min_irang(max_irang(b_max.abs(), b_min.abs()) - 1, a_max)
    }
}



impl<const AMIN: irang, const AMAX: irang, const BMIN: irang, const BMAX: irang>
    core::ops::Rem<Ranged<BMIN, BMAX>> for Ranged<AMIN, AMAX>
where
    Assert<{allow_range(memlayout(AMIN, AMAX))}>: IsAllowed,
    Assert<{allow_range(memlayout(BMIN, BMAX))}>: IsAllowed,
    Assert<{allow_range(memlayout(singleside_rem_min(AMIN, AMAX, BMIN, BMAX), singleside_rem_max(AMIN, AMAX, BMIN, BMAX)))}>: IsAllowed,

    Assert<{ allow_division(BMIN, BMAX) }>: IsAllowed,
    Assert<{ allow_if(true) }>: IsAllowed,
{
    type Output = Ranged<
        { singleside_rem_min(AMIN, AMAX, BMIN, BMAX) },
        { singleside_rem_max(AMIN, AMAX, BMIN, BMAX) },
    >;

    fn rem(self, rhs: Ranged<BMIN, BMAX>) -> Self::Output { Self::rem(self, rhs) }
}


// There are no std::ops traits for euclidean division/remainder, but these operations are implemented as functions.
// The following fns are the helpers for div_euclid and rem_euclid.


#[must_use]
#[doc(hidden)]
pub const fn singleside_div_euclid_min(a_min: irang, a_max: irang, b_min: irang, b_max: irang) -> irang {
    min_4((a_min.div_euclid(b_min), a_min.div_euclid(b_max), a_max.div_euclid(b_min), a_max.div_euclid(b_max)))
}
#[must_use]
#[doc(hidden)]
pub const fn singleside_div_euclid_max(a_min: irang, a_max: irang, b_min: irang, b_max: irang) -> irang {
    max_4((a_min.div_euclid(b_min), a_min.div_euclid(b_max), a_max.div_euclid(b_min), a_max.div_euclid(b_max)))
}


#[must_use]
#[doc(hidden)]
pub const fn singleside_rem_euclid_min(a_min: irang, a_max: irang, b_min: irang, b_max: irang) -> irang {
    // Note that b_min..=b_max must never include 0
    if b_min == b_max {
        if a_min == a_max {return a_min.rem_euclid(b_min)}

        let base_min = a_min.div_euclid(b_min);
        let base_max = a_max.div_euclid(b_min);
        if base_min==base_max {
            return a_min.rem_euclid(b_min);
        }
    }

    0
}

#[must_use]
#[doc(hidden)]
pub const fn singleside_rem_euclid_max(a_min: irang, a_max: irang, b_min: irang, b_max: irang) -> irang {
    // Note that b_min..=b_max must never include 0
    if b_min == b_max {
        if a_min == a_max {return a_min.rem_euclid(b_min)}

        let base_min = a_min.div_euclid(b_min);
        let base_max = a_max.div_euclid(b_min);
        if base_min==base_max {
            return a_max.rem_euclid(b_min);
        }
    }

    let absb_max = max_irang(b_min.abs(), b_max.abs());

    if a_min > 0 && a_max < absb_max {a_max}
    else {absb_max-1}
}

// The last block of helper functions is for abs() method

#[must_use] #[doc(hidden)]
pub const fn abs_min(min: irang, max: irang) -> irang {
    if min.signum() == max.signum() {min_irang(min.abs(), max.abs())}
    else {0}
}

#[must_use] #[doc(hidden)]
pub const fn abs_max(min: irang, max: irang) -> irang {
    max_irang(min.abs(), max.abs())
}

// Const arithmetic operations and the other arithmetic functions

impl<const MIN: irang, const MAX: irang> Ranged<MIN, MAX>
where Assert<{allow_range(memlayout(MIN, MAX))}>: IsAllowed,
{
    /// Adds two ranged integers, proves the result bounds
    // refer to Add trait for detailed comments
    #[must_use]
    pub const fn add<const BMIN: irang, const BMAX: irang>
        (self, rhs: Ranged<BMIN, BMAX>)
        -> Ranged<{ MIN + BMIN }, { MAX + BMAX }>
    where
        Assert<{allow_range(memlayout(BMIN, BMAX))}>: IsAllowed,
        Assert<{allow_range(memlayout(MIN + BMIN, MAX + BMAX))}>: IsAllowed,
    {
        unsafe { Ranged::unchecked_new(self.get() + rhs.get()) }
    }

    /// Subtracts two ranged integers, proves the result bounds
    // Refer to Add trait for detailed comments
    #[must_use]
    pub const fn sub<const BMIN: irang, const BMAX: irang>(self, rhs: Ranged<BMIN, BMAX>)
        -> Ranged<{ MIN - BMAX }, { MAX - BMIN }>
    where
        Assert<{allow_range(memlayout(BMIN, BMAX))}>: IsAllowed,
        Assert<{allow_range(memlayout(MIN - BMAX, MAX - BMIN))}>: IsAllowed,
    {
        unsafe { Ranged::unchecked_new(self.get() - rhs.get()) }
    }

    /// Multiplies two ranged integers, proves the result bounds
    #[must_use]
    pub const fn mul<const BMIN: irang, const BMAX: irang>(self, rhs: Ranged<BMIN, BMAX>)
        -> Ranged<{ min_cross(MIN, MAX, BMIN, BMAX) }, { max_cross(MIN, MAX, BMIN, BMAX) }>
    where
        Assert<{allow_range(memlayout(BMIN, BMAX))}>: IsAllowed,
        Assert<{allow_range(memlayout(min_cross(MIN, MAX, BMIN, BMAX), max_cross(MIN, MAX, BMIN, BMAX)))}>: IsAllowed,
    {
        unsafe { Ranged::unchecked_new(self.get() * rhs.get()) }
    }

    /// Divides two ranged integers, proves the result bounds
    #[must_use]
    pub const fn div<const BMIN: irang, const BMAX: irang>(self, rhs: Ranged<BMIN, BMAX>) 
        -> Ranged<{ singleside_div_min(MIN, MAX, BMIN, BMAX) }, { singleside_div_max(MIN, MAX, BMIN, BMAX) }>
    where
        Assert<{allow_range(memlayout(BMIN, BMAX))}>: IsAllowed,
        Assert<{allow_range(memlayout(singleside_div_min(MIN, MAX, BMIN, BMAX), singleside_div_max(MIN, MAX, BMIN, BMAX)))}>: IsAllowed,
        Assert<{ allow_division(BMIN, BMAX) }>: IsAllowed,
    {
        unsafe { Ranged::unchecked_new(self.get() / rhs.get()) }
    }

    /// Takes a remainder of two ranged integers division, proves the result bounds
    #[must_use]
    pub const fn rem<const BMIN: irang, const BMAX: irang>(self, rhs: Ranged<BMIN, BMAX>)
        -> Ranged<{ singleside_rem_min(MIN, MAX, BMIN, BMAX) }, { singleside_rem_max(MIN, MAX, BMIN, BMAX) }>
    where
        Assert<{allow_range(memlayout(BMIN, BMAX))}>: IsAllowed,
        Assert<{allow_range(memlayout(singleside_rem_min(MIN, MAX, BMIN, BMAX), singleside_rem_max(MIN, MAX, BMIN, BMAX)))}>: IsAllowed,
        Assert<{ allow_division(BMIN, BMAX) }>: IsAllowed,
    {
        unsafe { Ranged::unchecked_new(self.get() % rhs.get()) }
    }


    /// Returns the minimum of two values
    #[must_use]
    pub const fn min<const BMIN: irang, const BMAX: irang>(self, other: Ranged<BMIN,BMAX>)
             -> Ranged< {min_irang(MIN, BMIN)}, {min_irang(MAX, BMAX)} >
    where 
        Assert<{allow_range(memlayout(BMIN, BMAX))}>: IsAllowed,
        Assert<{allow_range(memlayout(min_irang(MIN, BMIN), min_irang(MAX, BMAX)))}>: IsAllowed,
    {
        unsafe { Ranged::unchecked_new(min_irang(self.get(), other.get() )) }
    }

    /// Returns the maximum of two values
    #[must_use]
    pub const fn max<const BMIN: irang, const BMAX: irang>(self, other: Ranged<BMIN,BMAX>)
             -> Ranged< {max_irang(MIN, BMIN)}, {max_irang(MAX, BMAX)} >
    where 
        Assert<{allow_range(memlayout(BMIN, BMAX))}>: IsAllowed,
        Assert<{allow_range(memlayout(max_irang(MIN, BMIN), max_irang(MAX, BMAX)))}>: IsAllowed,
    {
        unsafe { Ranged::unchecked_new(max_irang(self.get(), other.get() )) }
    }

    /// Computes the absolute value of `self`
    #[must_use]
    pub const fn abs(self) -> Ranged< {abs_min(MIN, MAX)}, {abs_max(MIN, MAX)} >
    where Assert<{allow_range(memlayout(abs_min(MIN, MAX), abs_max(MIN, MAX)))}>: IsAllowed,
    {
        unsafe { Ranged::unchecked_new(self.get().abs()) }
    }

    /// Computes the negative of `self`
    #[must_use]
    pub const fn neg(self) -> Ranged<{ -MAX }, { -MIN }>
    where
        Assert<{allow_range(memlayout(-MAX, -MIN))}>: IsAllowed ,
    {
        unsafe { Ranged::unchecked_new(-self.get()) }
    }

    /// Calculates the quotient of Euclidean division of self by rhs
    #[must_use]
    pub const fn div_euclid<const BMIN: irang, const BMAX: irang>(self, rhs: Ranged<BMIN,BMAX>) 
        -> Ranged< {singleside_div_euclid_min(MIN, MAX, BMIN, BMAX)}, {singleside_div_euclid_max(MIN, MAX, BMIN, BMAX)} > 
    where 
        Assert<{allow_range(memlayout(BMIN, BMAX))}>: IsAllowed,
        Assert<{allow_range(memlayout(singleside_div_euclid_min(MIN, MAX, BMIN, BMAX), singleside_div_euclid_max(MIN, MAX, BMIN, BMAX)))}>: IsAllowed,
        Assert<{ allow_division(BMIN, BMAX) }>: IsAllowed,
    Assert<{ allow_if(true) }>: IsAllowed,
    {
        unsafe { Ranged::unchecked_new(self.get().div_euclid(rhs.get())) }
    }

    /// Calculates the Euclidean mod of self by rhs
    #[must_use]
    pub const fn rem_euclid<const BMIN: irang, const BMAX: irang>(self, rhs: Ranged<BMIN,BMAX>) 
        -> Ranged< {singleside_rem_euclid_min(MIN, MAX, BMIN, BMAX)}, {singleside_rem_euclid_max(MIN, MAX, BMIN, BMAX)} > 
    where
        Assert<{allow_range(memlayout(BMIN, BMAX))}>: IsAllowed,
        Assert<{allow_range(memlayout(singleside_rem_euclid_min(MIN, MAX, BMIN, BMAX), singleside_rem_euclid_max(MIN, MAX, BMIN, BMAX)))}>: IsAllowed,
        Assert<{allow_division(BMIN, BMAX)}>: IsAllowed,
        Assert<{allow_if(true)}>: IsAllowed,
    {
        unsafe { Ranged::unchecked_new(self.get().rem_euclid(rhs.get())) }
    }

    /// Checks if two numbers are equal
    #[must_use]
    pub const fn eq<const BMIN: irang, const BMAX: irang>(&self, rhs: &Ranged<BMIN, BMAX>) -> bool 
    where Assert<{allow_range(memlayout(BMIN, BMAX))}>: IsAllowed
    {
        self.get() == rhs.get()
    }

    /// Checks if two numbers are NOT equal
    #[must_use]
    pub const fn ne<const BMIN: irang, const BMAX: irang>(&self, other: &Ranged<BMIN, BMAX>) -> bool
    where Assert<{allow_range(memlayout(BMIN, BMAX))}>: IsAllowed
    {
        !self.eq(other)
    }
}

// Equality and order traits

#[allow(clippy::use_self)]  // False positive clippy lint
impl<const MIN: irang, const MAX: irang>
    core::cmp::PartialEq<Ranged<MIN, MAX>> for irang
where
    Assert<{allow_range(memlayout(MIN, MAX))}>: IsAllowed ,
{
    fn eq(&self, other: &Ranged<MIN, MAX>) -> bool {
        *self == other.get()
    }

    #[allow(clippy::partialeq_ne_impl)] // Clippy makes a row, but it's mandatory in const trait impl to implement it
    fn ne(&self, other: &Ranged<MIN, MAX>) -> bool {
        !self.eq(other)
    }
}

impl<const MIN: irang, const MAX: irang>
    core::cmp::PartialEq<irang> for Ranged<MIN, MAX>
where
    Assert<{allow_range(memlayout(MIN, MAX))}>: IsAllowed ,
{
    fn eq(&self, other: &irang) -> bool {
        self.get() == *other
    }
}


impl<const AMIN: irang, const AMAX: irang, const BMIN: irang, const BMAX: irang>
    core::cmp::PartialEq<Ranged<BMIN, BMAX>> for Ranged<AMIN, AMAX>
where
    Assert<{allow_range(memlayout(AMIN, AMAX))}>: IsAllowed ,
    Assert<{allow_range(memlayout(BMIN, BMAX))}>: IsAllowed ,
{
    fn eq(&self, rhs: &Ranged<BMIN, BMAX>) -> bool {
        Self::eq(self, rhs)
    }
}

impl<const AMIN: irang, const AMAX: irang> core::cmp::Eq for Ranged<AMIN, AMAX>
where Assert<{allow_range(memlayout(AMIN, AMAX))}>: IsAllowed ,
{}


impl<const AMIN: irang, const AMAX: irang, const BMIN: irang, const BMAX: irang>
    core::cmp::PartialOrd<Ranged<BMIN, BMAX>> for Ranged<AMIN, AMAX> 
where
    Assert<{allow_range(memlayout(AMIN, AMAX))}>: IsAllowed ,
    Assert<{allow_range(memlayout(BMIN, BMAX))}>: IsAllowed ,
{
    fn partial_cmp(&self, other: &Ranged<BMIN, BMAX>) -> Option<core::cmp::Ordering> {
        let s = self.get();
        let o = other.get();
        #[allow(clippy::comparison_chain)]
        Some(if s>o {core::cmp::Ordering::Greater}
        else if s==o {core::cmp::Ordering::Equal}
        else {core::cmp::Ordering::Less})
    }

    fn lt(&self, other: &Ranged<BMIN, BMAX>) -> bool {
        self.get() < other.get()
    }
    fn le(&self, other: &Ranged<BMIN, BMAX>) -> bool {
        self.get() <= other.get()
    }
    fn gt(&self, other: &Ranged<BMIN, BMAX>) -> bool {
        self.get() > other.get()
    }
    fn ge(&self, other: &Ranged<BMIN, BMAX>) -> bool {
        self.get() >= other.get()
    }
}
impl<const AMIN: irang, const AMAX: irang> core::cmp::Ord for Ranged<AMIN, AMAX>
where
    Assert<{allow_range(memlayout(AMIN, AMAX))}>: IsAllowed ,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let s = self.get();
        let o = other.get();
        #[allow(clippy::comparison_chain)]
        if s>o {core::cmp::Ordering::Greater}
        else if s==o {core::cmp::Ordering::Equal}
        else {core::cmp::Ordering::Less}
    }
    fn max(self, other: Self) -> Self where Self: Sized {
        unsafe { Self::unchecked_new(max_irang(self.get(), other.get() )) }
    }
    fn min(self, other: Self) -> Self where Self: Sized {
        unsafe { Self::unchecked_new(min_irang(self.get(), other.get() )) }
    }
    fn clamp(self, min: Self, max: Self) -> Self where Self: Sized {
        <Self as core::cmp::Ord>::min(<Self as core::cmp::Ord>::max(self, min), max)
    }
}

impl<const AMIN: irang, const AMAX: irang>
    core::cmp::PartialOrd<irang> for Ranged<AMIN, AMAX> 
where
    Assert<{allow_range(memlayout(AMIN, AMAX))}>: IsAllowed ,
{
    fn partial_cmp(&self, other: &irang) -> Option<core::cmp::Ordering> {
        let s = self.get();
        #[allow(clippy::comparison_chain)]
        Some(if s>*other {core::cmp::Ordering::Greater}
        else if s==*other {core::cmp::Ordering::Equal}
        else {core::cmp::Ordering::Less})
    }

    fn lt(&self, other: &irang) -> bool {
        matches!(self.partial_cmp(other), Some(core::cmp::Ordering::Less))
    }
    fn le(&self, other: &irang) -> bool {
        !matches!(self.partial_cmp(other), None | Some(core::cmp::Ordering::Greater))
    }
    fn gt(&self, other: &irang) -> bool {
        matches!(self.partial_cmp(other), Some(core::cmp::Ordering::Greater))
    }
    fn ge(&self, other: &irang) -> bool {
        matches!(self.partial_cmp(other), Some(core::cmp::Ordering::Greater | core::cmp::Ordering::Equal))
    }
}

#[allow(clippy::use_self)]  // False positive clippy lint
impl<const AMIN: irang, const AMAX: irang>
    core::cmp::PartialOrd<Ranged<AMIN, AMAX>> for irang
where
    Assert<{allow_range(memlayout(AMIN, AMAX))}>: IsAllowed ,
{
    fn partial_cmp(&self, other: &Ranged<AMIN, AMAX>) -> Option<core::cmp::Ordering> {
        let o = other.get();
        #[allow(clippy::comparison_chain)]
        Some(if *self>o {core::cmp::Ordering::Greater}
        else if *self==o {core::cmp::Ordering::Equal}
        else {core::cmp::Ordering::Less})
    }

    fn lt(&self, other: &Ranged<AMIN, AMAX>) -> bool {
        matches!(self.partial_cmp(other), Some(core::cmp::Ordering::Less))
    }
    fn le(&self, other: &Ranged<AMIN, AMAX>) -> bool {
        !matches!(self.partial_cmp(other), None | Some(core::cmp::Ordering::Greater))
    }
    fn gt(&self, other: &Ranged<AMIN, AMAX>) -> bool {
        matches!(self.partial_cmp(other), Some(core::cmp::Ordering::Greater))
    }
    fn ge(&self, other: &Ranged<AMIN, AMAX>) -> bool {
        matches!(self.partial_cmp(other), Some(core::cmp::Ordering::Greater | core::cmp::Ordering::Equal))
    }
}
