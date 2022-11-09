// Any arithmetic operation are first of all to recalculate the bounds

use crate::{Assert, IsAllowed, OperationPossibility, Ranged, irang, max_irang, memlayout, min_irang};

impl<const AMIN: irang, const AMAX: irang, const BMIN: irang, const BMAX: irang> const
core::ops::Add<Ranged<BMIN, BMAX>> for Ranged<AMIN, AMAX>
where
    [(); memlayout(AMIN, AMAX).bytes()]:,
    [(); memlayout(BMIN, BMAX).bytes()]:,
    [(); memlayout(AMIN + BMIN, AMAX + BMAX).bytes()]:,
{
    type Output = Ranged<{ AMIN + BMIN }, { AMAX + BMAX }>;

    fn add(self, rhs: Ranged<BMIN, BMAX>) -> Self::Output {
        unsafe { Ranged::__unsafe_new(self.get() + rhs.get()) }
    }
}

impl<const AMIN: irang, const AMAX: irang, const BMIN: irang, const BMAX: irang> const
core::ops::Sub<Ranged<BMIN, BMAX>> for Ranged<AMIN, AMAX>
where
    [(); memlayout(AMIN, AMAX).bytes()]:,
    [(); memlayout(BMIN, BMAX).bytes()]:,
    [(); memlayout(AMIN - BMAX, AMAX - BMIN).bytes()]:,
{
    type Output = Ranged<{ AMIN - BMAX }, { AMAX - BMIN }>;

    fn sub(self, rhs: Ranged<BMIN, BMAX>) -> Self::Output {
        unsafe { Ranged::__unsafe_new(self.get() - rhs.get()) }
    }
}

// The current (10.2022) of Rust are not capable to work with iterators out-of-the-box
// in const environments. The following macros and functions are the ad-hoc replacers
// for min/max functions
macro_rules! reduce {
    ($fn:ident, $a:expr) => ( $a );
    ($fn:ident, $a:expr, $($args:expr),+) => {
        {
            $fn($a, reduce!($fn, $($args),+ ))
        }
    };
}

const fn max_4(vals: (irang, irang, irang, irang)) -> irang {
    reduce!(max_irang, vals.0, vals.1, vals.2, vals.3)
}
const fn min_4(vals: (irang, irang, irang, irang)) -> irang {
    reduce!(min_irang, vals.0, vals.1, vals.2, vals.3)
}
#[must_use]
#[doc(hidden)]
pub const fn max_cross(a_min: irang, a_max: irang, b_min: irang, b_max: irang) -> irang {
    max_4((a_min * b_min, a_min * b_max, a_max * b_min, a_max * b_max))
}
#[must_use]
#[doc(hidden)]
pub const fn min_cross(a_min: irang, a_max: irang, b_min: irang, b_max: irang) -> irang {
    min_4((a_min * b_min, a_min * b_max, a_max * b_min, a_max * b_max))
}

impl<const AMIN: irang, const AMAX: irang, const BMIN: irang, const BMAX: irang> const
    core::ops::Mul<Ranged<BMIN, BMAX>> for Ranged<AMIN, AMAX>
where
    [(); memlayout(AMIN, AMAX).bytes()]: ,
    [(); memlayout(BMIN, BMAX).bytes()]: ,
    [(); memlayout(
        min_cross(AMIN, AMAX, BMIN, BMAX),
        max_cross(AMIN, AMAX, BMIN, BMAX),
    )
    .bytes()]: ,
{
    type Output =
        Ranged<{ min_cross(AMIN, AMAX, BMIN, BMAX) }, { max_cross(AMIN, AMAX, BMIN, BMAX) }>;

    fn mul(self, rhs: Ranged<BMIN, BMAX>) -> Self::Output {
        unsafe { Ranged::__unsafe_new(self.get() * rhs.get()) }
    }
}

#[must_use]
#[doc(hidden)]
pub const fn allow_division(b_min: irang, b_max: irang) -> OperationPossibility {
    if ((b_min > 0) && (b_max > 0)) || ((b_min < 0) && (b_max < 0)) {
        OperationPossibility::Allowed
    } else {
        OperationPossibility::Forbidden
    }
}

#[must_use]
#[doc(hidden)]
pub const fn singleside_div_min(a_min: irang, a_max: irang, b_min: irang, b_max: irang) -> irang {
    min_4((a_min / b_min, a_min / b_max, a_max / b_min, a_max / b_max))
}
#[must_use]
#[doc(hidden)]
pub const fn singleside_div_max(a_min: irang, a_max: irang, b_min: irang, b_max: irang) -> irang {
    max_4((a_min / b_min, a_min / b_max, a_max / b_min, a_max / b_max))
}

impl<const AMIN: irang, const AMAX: irang, const BMIN: irang, const BMAX: irang> const
    core::ops::Div<Ranged<BMIN, BMAX>> for Ranged<AMIN, AMAX>
where
    [(); memlayout(AMIN, AMAX).bytes()]: ,
    [(); memlayout(BMIN, BMAX).bytes()]: ,
    [(); memlayout(
        singleside_div_min(AMIN, AMAX, BMIN, BMAX),
        singleside_div_max(AMIN, AMAX, BMIN, BMAX),
    )
    .bytes()]: ,

    Assert<{ allow_division(BMIN, BMAX) }>: IsAllowed,
{
    type Output = Ranged<
        { singleside_div_min(AMIN, AMAX, BMIN, BMAX) },
        { singleside_div_max(AMIN, AMAX, BMIN, BMAX) },
    >;

    fn div(self, rhs: Ranged<BMIN, BMAX>) -> Self::Output {
        unsafe { Ranged::__unsafe_new(self.get() / rhs.get()) }
    }
}

#[must_use]
#[doc(hidden)]
pub const fn singleside_rem_min(a_min: irang, a_max: irang, b_min: irang, b_max: irang) -> irang {
    // Note that b_min..=b_max must never include 0
    if b_min == b_max {
        if a_min == a_max {return a_min % b_min}
        else if a_min > 0 {
            let base = a_max / b_min.abs() * b_min.abs();
            if a_min >= base {
                return a_min % b_min;
            }
        }
        else if a_max < 0 {
            let base = a_min / b_min.abs() * b_min.abs();
            if a_max <= base {
                return a_min % b_min;
            }
        }
    }

    if a_min >= 0 {0} 
    else {
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





impl<const AMIN: irang, const AMAX: irang, const BMIN: irang, const BMAX: irang> const
    core::ops::Rem<Ranged<BMIN, BMAX>> for Ranged<AMIN, AMAX>
where
    [(); memlayout(AMIN, AMAX).bytes()]: ,
    [(); memlayout(BMIN, BMAX).bytes()]: ,
    [(); memlayout(
        singleside_rem_min(AMIN, AMAX, BMIN, BMAX),
        singleside_rem_max(AMIN, AMAX, BMIN, BMAX),
    )
    .bytes()]: ,

    Assert<{ allow_division(BMIN, BMAX) }>: IsAllowed,
{
    type Output = Ranged<
        { singleside_rem_min(AMIN, AMAX, BMIN, BMAX) },
        { singleside_rem_max(AMIN, AMAX, BMIN, BMAX) },
    >;

    fn rem(self, rhs: Ranged<BMIN, BMAX>) -> Self::Output {
        unsafe { Ranged::__unsafe_new(self.get() % rhs.get()) }
    }
}

impl<const MIN: irang, const MAX: irang> const core::ops::Neg for Ranged<MIN, MAX>
where
    [(); memlayout(MIN, MAX).bytes()]: ,
    [(); memlayout(-MAX, -MIN).bytes()]: ,
{
    type Output = Ranged<{ -MAX }, { -MIN }>;

    fn neg(self) -> Self::Output {
        unsafe { Ranged::__unsafe_new(-self.get()) }
    }
}


#[allow(clippy::use_self)]  // False positive clippy lint
impl<const MIN: irang, const MAX: irang> const
    core::cmp::PartialEq<Ranged<MIN, MAX>> for irang
where
    [(); memlayout(MIN, MAX).bytes()]: ,
{
    fn eq(&self, other: &Ranged<MIN, MAX>) -> bool {
        *self == other.get()
    }

    #[allow(clippy::partialeq_ne_impl)] // Clippy makes a row, but it's mandatory in const trait impl to implement it
    fn ne(&self, other: &Ranged<MIN, MAX>) -> bool {
        !self.eq(other)
    }
}

impl<const MIN: irang, const MAX: irang> const
    core::cmp::PartialEq<irang> for Ranged<MIN, MAX>
where
    [(); memlayout(MIN, MAX).bytes()]: ,
{
    fn eq(&self, other: &irang) -> bool {
        self.get() == *other
    }

    #[allow(clippy::partialeq_ne_impl)] // Clippy makes a row, but it's mandatory in const trait impl to implement it
    fn ne(&self, other: &irang) -> bool {
        !self.eq(other)
    }
}


impl<const AMIN: irang, const AMAX: irang, const BMIN: irang, const BMAX: irang> const
    core::cmp::PartialEq<Ranged<BMIN, BMAX>> for Ranged<AMIN, AMAX>
where
    [(); memlayout(AMIN, AMAX).bytes()]: ,
    [(); memlayout(BMIN, BMAX).bytes()]: ,
{
    fn eq(&self, rhs: &Ranged<BMIN, BMAX>) -> bool {
        self.get() == rhs.get()
    }

    #[allow(clippy::partialeq_ne_impl)] // Clippy makes a row, but it's mandatory in const trait impl to implement it
    fn ne(&self, other: &Ranged<BMIN, BMAX>) -> bool {
        !self.eq(other)
    }
}

impl<const AMIN: irang, const AMAX: irang> core::cmp::Eq for Ranged<AMIN, AMAX>
where [(); memlayout(AMIN, AMAX).bytes()]: {}


impl<const AMIN: irang, const AMAX: irang, const BMIN: irang, const BMAX: irang> const
    core::cmp::PartialOrd<Ranged<BMIN, BMAX>> for Ranged<AMIN, AMAX> 
where
    [(); memlayout(AMIN, AMAX).bytes()]: ,
    [(); memlayout(BMIN, BMAX).bytes()]: ,
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
impl<const AMIN: irang, const AMAX: irang> const core::cmp::Ord for Ranged<AMIN, AMAX>
where
    [(); memlayout(AMIN, AMAX).bytes()]: ,
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
        unsafe { Self::__unsafe_new(max_irang(self.get(), other.get() )) }
    }
    fn min(self, other: Self) -> Self where Self: Sized {
        unsafe { Self::__unsafe_new(min_irang(self.get(), other.get() )) }
    }
    fn clamp(self, min: Self, max: Self) -> Self where Self: Sized {
        <Self as core::cmp::Ord>::min(<Self as core::cmp::Ord>::max(self, min), max)
    }
}

impl<const AMIN: irang, const AMAX: irang> const
    core::cmp::PartialOrd<irang> for Ranged<AMIN, AMAX> 
where
    [(); memlayout(AMIN, AMAX).bytes()]: ,
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
impl<const AMIN: irang, const AMAX: irang> const
    core::cmp::PartialOrd<Ranged<AMIN, AMAX>> for irang
where
    [(); memlayout(AMIN, AMAX).bytes()]: ,
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

#[must_use] #[doc(hidden)]
pub const fn abs_min(min: irang, max: irang) -> irang {
    if min.signum() == max.signum() {min_irang(min.abs(), max.abs())}
    else {0}
}

#[must_use] #[doc(hidden)]
pub const fn abs_max(min: irang, max: irang) -> irang {
    max_irang(min.abs(), max.abs())
}



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


impl<const MIN: irang, const MAX: irang> Ranged<MIN, MAX>
where [u8; memlayout(MIN, MAX).bytes()]:
{
    /// Returns the minimum of two values
    pub const fn min<const BMIN: irang, const BMAX: irang>(self, other: Ranged<BMIN,BMAX>)
             -> Ranged< {min_irang(MIN, BMIN)}, {min_irang(MAX, BMAX)} >
    where 
        [u8; memlayout(BMIN, BMAX).bytes()]:,
        [u8; memlayout(min_irang(MIN, BMIN), min_irang(MAX, BMAX)).bytes()]:
    {
        unsafe { Ranged::__unsafe_new(min_irang(self.get(), other.get() )) }
    }

    /// Returns the maximum of two values
    pub const fn max<const BMIN: irang, const BMAX: irang>(self, other: Ranged<BMIN,BMAX>)
             -> Ranged< {max_irang(MIN, BMIN)}, {max_irang(MAX, BMAX)} >
    where 
        [u8; memlayout(BMIN, BMAX).bytes()]:,
        [u8; memlayout(max_irang(MIN, BMIN), max_irang(MAX, BMAX)).bytes()]:
    {
        unsafe { Ranged::__unsafe_new(max_irang(self.get(), other.get() )) }
    }

    /// Computes the absolute value of `self`
    pub const fn abs(self) -> Ranged< {abs_min(MIN, MAX)}, {abs_max(MIN, MAX)} >
    where [u8; memlayout(abs_min(MIN, MAX), abs_max(MIN, MAX)).bytes()]:
    {
        unsafe { Ranged::__unsafe_new(self.get().abs()) }
    }

    /// Calculates the quotient of Euclidean division of self by rhs
    pub const fn div_euclid<const BMIN: irang, const BMAX: irang>(self, rhs: Ranged<BMIN,BMAX>) 
        -> Ranged< {singleside_div_euclid_min(MIN, MAX, BMIN, BMAX)}, {singleside_div_euclid_max(MIN, MAX, BMIN, BMAX)} > 
    where 
        [u8; memlayout(BMIN, BMAX).bytes()]:,
        [u8; memlayout(singleside_div_euclid_min(MIN, MAX, BMIN, BMAX), singleside_div_euclid_max(MIN, MAX, BMIN, BMAX)).bytes()]:,
        Assert<{ allow_division(BMIN, BMAX) }>: IsAllowed,
    {
        unsafe { Ranged::__unsafe_new(self.get().div_euclid(rhs.get())) }
    }

    /// Calculates the Euclidean mod of self by rhs
    pub const fn rem_euclid<const BMIN: irang, const BMAX: irang>(self, rhs: Ranged<BMIN,BMAX>) 
        -> Ranged< {singleside_rem_euclid_min(MIN, MAX, BMIN, BMAX)}, {singleside_rem_euclid_max(MIN, MAX, BMIN, BMAX)} > 
    where
        [u8; memlayout(BMIN, BMAX).bytes()]:,
        [u8; memlayout(singleside_rem_euclid_min(MIN, MAX, BMIN, BMAX), singleside_rem_euclid_max(MIN, MAX, BMIN, BMAX)).bytes()]:,
        Assert<{ allow_division(BMIN, BMAX) }>: IsAllowed,
    {
        unsafe { Ranged::__unsafe_new(self.get().rem_euclid(rhs.get())) }
    }
}
