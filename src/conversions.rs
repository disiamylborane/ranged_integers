use core::str::FromStr;
use crate::allow_range;
use crate::arithmetics::{max_irang, min_irang};
use crate::value_check::allow_if;
use crate::{Assert, IsAllowed, OperationPossibility, Ranged, irang, memlayout, arithmetics::allow_division};
/// Convert an integer value to Ranged according to its own bounds.
///
/// Implemented for integer primitives.
///
/// ```
/// # #![feature(adt_const_params, generic_const_exprs)]
/// use ranged_integers::*;
///
/// let a = 42_u8;
/// let ra = a.as_ranged();
/// let check_ra: Ranged<0, 255> = ra;
/// ```
pub trait AsRanged: Copy {
    /// Conversion output
    type Res;

    /// Convert to Ranged type
    fn as_ranged(self) -> Self::Res;
}

macro_rules! int_ranged_converters {
    ($($t: ident)+) => {
        #[doc(hidden)]
        pub mod converter_checkers {
            use super::OperationPossibility;
            use super::irang;

            $(
                #[must_use] #[doc(hidden)] pub const fn $t (min: irang, max: irang)->OperationPossibility {
                    $crate::allow_if(min>=$t::MIN as irang && max<=$t::MAX as irang)
                }
            )+
        }

        impl<const MIN: irang, const MAX: irang> Ranged<MIN, MAX>
        where Assert<{allow_range(memlayout(MIN, MAX))}>: IsAllowed,
        {
            $(
                #[doc=concat!("Convert a Ranged into `", stringify!($t), "` value. Accessible if fits.")]
                #[must_use]
                pub const fn $t(self) -> $t
                where Assert<{converter_checkers::$t(MIN, MAX)}>: IsAllowed
                {
                    #![allow(clippy::cast_possible_truncation)]
                    #![allow(clippy::cast_sign_loss)]
                    self.get() as $t
                }
            )+
        }
        $(
            impl<const MIN: irang, const MAX: irang> From<Ranged<MIN, MAX>> for $t
            where
                Assert<{allow_range(memlayout(MIN, MAX))}>: IsAllowed,
                Assert<{converter_checkers::$t(MIN, MAX)}>: IsAllowed,
            {
                fn from(a: Ranged<MIN, MAX>) -> Self { a.$t() }
            }
        )+
    };
}
macro_rules! as_ranged_impl {
    ($($t: ident)+) => {
        $(
            impl AsRanged for $t {
                type Res = Ranged<{$t::MIN as irang},{$t::MAX as irang}>;
                fn as_ranged(self) -> Self::Res {
                    #![allow(clippy::cast_lossless)]
                    unsafe {Self::Res::unchecked_new(self as irang)}
                }
            }

        )+
    };
}

int_ranged_converters! {i8 u8 i16 u16 i32 u32 i64 u64 i128 isize usize}
as_ranged_impl! {i8 u8 i16 u16 i32 u32 i64 u64 isize usize}

macro_rules! signed_ranged_rem {
    ($($t: ident)+) => {
        $(
            impl<const VAL: irang> core::ops::Rem<Ranged<VAL, VAL>> for $t
            where
                Assert<{allow_range(memlayout(VAL, VAL))}>: IsAllowed,
                Assert<{allow_range(memlayout(1-VAL.abs(), VAL.abs()-1))}>: IsAllowed,
                Assert<{ allow_division(VAL, VAL) }>: IsAllowed,
                Assert<{ allow_if(true) }>: IsAllowed,
            {
                type Output = Ranged<{1-VAL.abs()}, {VAL.abs()-1}>;

                fn rem(self, _rhs: Ranged<VAL, VAL>) -> Self::Output {
                    #![allow(clippy::cast_lossless)]
                    unsafe { Ranged::unchecked_new(self as irang % VAL) }
                }
            }
        )+
    };
}
signed_ranged_rem! {i8 i16 i32 i64 i128 isize}

macro_rules! unsigned_ranged_rem {
    ($($t: ident)+) => {
        $(
            impl<const VAL: irang> core::ops::Rem<Ranged<VAL, VAL>> for $t
            where
                Assert<{allow_range(memlayout(VAL, VAL))}>: IsAllowed,
                Assert<{allow_range(memlayout(0, VAL.abs()-1))}>: IsAllowed,
            {
                type Output = Ranged<0, {VAL.abs()-1}>;

                fn rem(self, _rhs: Ranged<VAL, VAL>) -> Self::Output {
                    #![allow(clippy::cast_lossless)]
                    unsafe { Ranged::unchecked_new(self as irang % VAL) }
                }
            }
        )+
    };
}
unsigned_ranged_rem! {u8 u16 u32 u64 usize}

#[must_use]
#[doc(hidden)]
pub const fn expansion_possible(s_min: irang, s_max: irang, r_min: irang, r_max: irang) -> OperationPossibility {
    allow_if(r_min <= s_min && r_max >= s_max)
}

#[must_use]
#[doc(hidden)]
pub const fn lessthan(a: irang, b: irang) -> OperationPossibility {
    allow_if(a < b)
}
#[must_use]
#[doc(hidden)]
pub const fn lesseq(a: irang, b: irang) -> OperationPossibility {
    allow_if(a <= b)
}

impl<const MIN: irang, const MAX: irang> Ranged<MIN, MAX>
where Assert<{allow_range(memlayout(MIN, MAX))}>: IsAllowed,
{
    /// Convert to the Ranged with the wider bounds
    #[inline]
    #[must_use]
    pub const fn expand<const RMIN: irang, const RMAX: irang>(self) -> Ranged<RMIN, RMAX>
    where
        Assert<{allow_range(memlayout(RMIN, RMAX))}>: IsAllowed ,
        Assert<{ allow_if(true) }>: IsAllowed,
        Assert<{ expansion_possible(MIN, MAX, RMIN, RMAX) }>: IsAllowed,
    {
        unsafe { Ranged::unchecked_new(self.get()) }
    }

    /// Convert to the other `Ranged`, returning `None` if the value is out of range
    #[inline]
    #[must_use]
    pub const fn fit<const RMIN: irang, const RMAX: irang>(self) -> Option<Ranged<RMIN, RMAX>>
    where Assert<{allow_range(memlayout(RMIN, RMAX))}>: IsAllowed, {
        Ranged::<RMIN, RMAX>::new(self.get())
    }

    /// Change the `MIN` value of Ranged bounds, returning `None` if the value is out of range
    #[inline]
    #[must_use]
    pub const fn fit_max<const RMAX: irang>(self) -> Option<Ranged<MIN, RMAX>>
    where Assert<{allow_range(memlayout(MIN, RMAX))}>: IsAllowed, {
        Ranged::<MIN, RMAX>::new(self.get())
    }

    /// Change the `MAX` value of Ranged bounds, returning `None` if the value is out of range
    #[inline]
    #[must_use]
    pub const fn fit_min<const RMIN: irang>(self) -> Option<Ranged<RMIN, MAX>>
    where Assert<{allow_range(memlayout(RMIN, MAX))}>: IsAllowed, 
    {
        Ranged::<RMIN, MAX>::new(self.get())
    }

    /// Compares two `Ranged` values. If self is less than the other, it
    /// returns a Ranged with the same value and shrunk bounds.
    /// 
    /// Allowed only if the ranges interleave.
    #[inline]
    #[must_use]
    pub const fn fit_less_than<const RMIN: irang, const RMAX: irang>(self, other: Ranged<RMIN, RMAX>) -> Option<Ranged<MIN, {RMAX-1}>> 
    where
        Assert<{allow_range(memlayout(RMIN, RMAX))}>: IsAllowed,
        Assert<{allow_range(memlayout(MIN, RMAX-1))}>: IsAllowed,
        Assert<{lesseq(RMAX, MAX)}>: IsAllowed,
        Assert<{lessthan(MIN, RMAX)}>: IsAllowed,
    {
        if self.get() < other.get() {
            Some(unsafe{ Ranged::unchecked_new(self.get()) })
        } else {None}
    }

    /// Compares two `Ranged` values. If self is less than or equal to the other, it
    /// returns a Ranged with the same value and shrunk bounds.
    /// 
    /// Allowed only if the ranges interleave.
    #[inline]
    #[must_use]
    pub const fn fit_less_eq<const RMIN: irang, const RMAX: irang>(self, other: Ranged<RMIN, RMAX>) -> Option<Ranged<MIN, RMAX>> 
    where
        Assert<{allow_range(memlayout(RMIN, RMAX))}>: IsAllowed,
        Assert<{allow_range(memlayout(MIN, RMAX))}>: IsAllowed,
        Assert<{lessthan(RMAX, MAX)}>: IsAllowed,
        Assert<{lesseq(MIN, RMAX)}>: IsAllowed,
    {
        if self.get() <= other.get() {
            Some(unsafe{ Ranged::unchecked_new(self.get()) })
        } else {None}
    }

    /// Compares two `Ranged` values. If self is greater than the other, it
    /// returns a Ranged with the same value and shrunk bounds.
    /// 
    /// Allowed only if the ranges interleave.
    #[inline]
    #[must_use]
    pub const fn fit_greater_than<const RMIN: irang, const RMAX: irang>(self, other: Ranged<RMIN, RMAX>) -> Option<Ranged<{RMIN+1}, MAX>> 
    where
        Assert<{allow_range(memlayout(RMIN, RMAX))}>: IsAllowed,
        Assert<{allow_range(memlayout(RMIN+1, MAX))}>: IsAllowed,
        Assert<{lesseq(MIN, RMIN)}>: IsAllowed,
        Assert<{lessthan(RMIN, MAX)}>: IsAllowed,
    {
        if self.get() > other.get() {
            Some(unsafe{ Ranged::unchecked_new(self.get()) })
        } else {None}
    }

    /// Compares two `Ranged` values. If self is greater than or equal to the other, it
    /// returns a Ranged with the same value and shrunk bounds.
    /// 
    /// Allowed only if the ranges interleave.
    #[inline]
    #[must_use]
    pub const fn fit_greater_eq<const RMIN: irang, const RMAX: irang>(self, other: Ranged<RMIN, RMAX>) -> Option<Ranged<RMIN, MAX>> 
    where
        Assert<{allow_range(memlayout(RMIN, RMAX))}>: IsAllowed,
        Assert<{allow_range(memlayout(RMIN, MAX))}>: IsAllowed,
        Assert<{lessthan(MIN, RMIN)}>: IsAllowed,
        Assert<{lesseq(RMIN, MAX)}>: IsAllowed,
    {
        if self.get() >= other.get() {
            Some(unsafe{ Ranged::unchecked_new(self.get()) })
        } else {None}
    }



    /// Simple case analysis for Ranged
    #[must_use]
    pub const fn split<const MID: irang>(self) -> Split<MIN, MID, MAX>
    where 
        Assert<{allow_range(memlayout(MIN, MID-1))}>: IsAllowed,
        Assert<{allow_range(memlayout(MID, MAX))}>: IsAllowed
    {
        if let Some(higher) = self.fit_min() {
            Split::Higher(higher)
        } else {
            Split::Lower(unsafe {Ranged::unchecked_new(self.get())})
        }
    }

    /// Narrow ranges guiding by the subtraction of two values
    /// 
    /// Allowed only if the ranges overlap.
    #[must_use]
    pub const fn split_subtract<const BMIN: irang, const BMAX: irang>(self, other: Ranged<BMIN, BMAX>) -> SplitByDifference<MIN, MAX, BMIN, BMAX>
    where
        Assert<{allow_range(memlayout(BMIN, BMAX))}>: IsAllowed,
        Assert<{allow_range(memlayout(1, MAX-BMIN))}>: IsAllowed,
        Assert<{allow_range(memlayout(MIN-BMAX, -1))}>: IsAllowed,
        Assert<{allow_range(memlayout(max_irang(MIN, BMIN+1), MAX))}>: IsAllowed,
        Assert<{allow_range(memlayout(max_irang(BMIN, MIN+1), BMAX))}>: IsAllowed,
        Assert<{allow_range(memlayout(BMIN, min_irang(BMAX, MAX-1)))}>: IsAllowed,
        Assert<{allow_range(memlayout(MIN, min_irang(MAX, BMAX-1)))}>: IsAllowed,
        Assert<{allow_range(memlayout(max_irang(MIN, BMIN), min_irang(MAX, BMAX)))}>: IsAllowed,
    {
        unsafe {
            let minuend = self.get();
            let subtrahend = other.get();
            let difference = minuend - subtrahend;
            match difference {
                1.. => SplitByDifference::Greater {
                    minuend: Ranged::unchecked_new(minuend),
                    subtrahend: Ranged::unchecked_new(subtrahend),
                    difference: Ranged::unchecked_new(difference),
                },
                0 => SplitByDifference::Equal {
                    minuend: Ranged::unchecked_new(minuend),
                    subtrahend: Ranged::unchecked_new(subtrahend),
                    difference: Ranged::unchecked_new(difference),
                },
                ..=-1 => SplitByDifference::Less {
                    minuend: Ranged::unchecked_new(minuend),
                    subtrahend: Ranged::unchecked_new(subtrahend),
                    difference: Ranged::unchecked_new(difference),
                },
            }
        }
    }
}

/// Case-analyzed ranged value
pub enum Split<const MIN: irang, const MID: irang, const MAX: irang>
where 
    Assert<{allow_range(memlayout(MIN, MID-1))}>: IsAllowed,
    Assert<{allow_range(memlayout(MID, MAX))}>: IsAllowed
{
    /// Value is below the specified splitting point
    Lower(Ranged<MIN, {MID-1}>),
    /// Value equals or is above the specified splitting point
    Higher(Ranged<MID, MAX>)
}

/// Case-analyzed difference (subtraction result) of two values
/// 
/// Is created by [`Ranged::split_subtract`] method call.
pub enum SplitByDifference<const AMIN: irang, const AMAX: irang, const BMIN: irang, const BMAX: irang>
where
    Assert<{allow_range(memlayout(1, AMAX-BMIN))}>: IsAllowed,
    Assert<{allow_range(memlayout(AMIN-BMAX, -1))}>: IsAllowed,
    Assert<{allow_range(memlayout(max_irang(AMIN, BMIN+1), AMAX))}>: IsAllowed,
    Assert<{allow_range(memlayout(max_irang(BMIN, AMIN+1), BMAX))}>: IsAllowed,
    Assert<{allow_range(memlayout(BMIN, min_irang(BMAX, AMAX-1)))}>: IsAllowed,
    Assert<{allow_range(memlayout(AMIN, min_irang(AMAX, BMAX-1)))}>: IsAllowed,
    Assert<{allow_range(memlayout(max_irang(AMIN, BMIN), min_irang(AMAX, BMAX)))}>: IsAllowed,
{
    /// Minuend is greater than subtrahend
    Greater {
        /// Minuend (first parameter) with narrower bounds
        minuend: Ranged<{max_irang(AMIN, BMIN+1)}, AMAX>,
        /// Subtrahend (second parameter) with narrower bounds
        subtrahend: Ranged<BMIN, {min_irang(BMAX, AMAX-1)}>,
        /// Subtraction result
        difference: Ranged<1, {AMAX-BMIN}>},
    /// Minuend and subtrahend are equal
    Equal {
        /// Minuend (first parameter) with narrower bounds
        minuend: Ranged<{max_irang(AMIN, BMIN)},{min_irang(AMAX, BMAX)}>,
        /// Subtrahend (second parameter) with narrower bounds
        subtrahend: Ranged<{max_irang(AMIN, BMIN)},{min_irang(AMAX, BMAX)}>,
        /// Subtraction result
        difference: Ranged<0,0>,
    },
    /// Minuend is less than subtrahend
    Less {
        /// Minuend (first parameter) with narrower bounds
        minuend: Ranged<AMIN, {min_irang(AMAX, BMAX-1)}>,
        /// Subtrahend (second parameter) with narrower bounds
        subtrahend: Ranged<{max_irang(BMIN, AMIN+1)}, BMAX>,
        /// Subtraction result
        difference: Ranged<{AMIN-BMAX}, -1>,
    }
}

#[derive(Debug)]
pub struct ParseRangedError;

impl<const MIN: irang, const MAX: irang> FromStr for Ranged<MIN,MAX>
where Assert<{allow_range(memlayout(MIN, MAX))}>: IsAllowed,
{
    type Err = ParseRangedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let a : irang = s.parse().ok().ok_or(ParseRangedError)?;
        Self::new(a).ok_or(ParseRangedError)
    }
}
