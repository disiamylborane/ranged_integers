use crate::{Assert, IsAllowed, OperationPossibility, Ranged, irang, memlayout, arithmetics::allow_division};

/// Convert an integer value to Ranged according to its own bounds.
///
/// Implemented for integer primitives.
///
/// ```
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
                    OperationPossibility::allow_if(min>=core::$t::MIN as irang && max<=core::$t::MAX as irang)
                }
            )+
        }

        impl<const MIN: irang, const MAX: irang> Ranged<MIN, MAX>
        where [u8; memlayout(MIN, MAX).bytes()]:,
        {
            $(
                #[doc=concat!("Convert a Ranged into `", stringify!($t), "` value. Accessible if fits.")]
                pub const fn $t(self) -> $t
                where Assert<{converter_checkers::$t(MIN, MAX)}>: IsAllowed
                {
                    self.get() as $t
                }
            )+
        }

        $(
            impl<const MIN: irang, const MAX: irang> const From<Ranged<MIN, MAX>> for $t
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

        )+
    };
}

int_ranged_converters! {i8 u8 i16 u16 i32 u32 i64 u64 i128 isize usize}

macro_rules! signed_ranged_rem {
    ($($t: ident)+) => {
        $(
            impl<const VAL: irang> const core::ops::Rem<Ranged<VAL, VAL>> for $t
            where
                [(); memlayout(VAL, VAL).bytes()]:,
                [(); memlayout(1-VAL.abs(), VAL.abs()-1).bytes()]:,
                Assert<{ allow_division(VAL, VAL) }>: IsAllowed,
            {
                type Output = Ranged<{1-VAL.abs()}, {VAL.abs()-1}>;

                fn rem(self, _rhs: Ranged<VAL, VAL>) -> Self::Output {
                    unsafe { Ranged::__unsafe_new(self as irang % VAL) }
                }
            }
        )+
    };
}
signed_ranged_rem! {i8 i16 i32 i64 i128 isize}

macro_rules! unsigned_ranged_rem {
    ($($t: ident)+) => {
        $(
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
unsigned_ranged_rem! {u8 u16 u32 u64 usize}

#[must_use]
#[doc(hidden)]
pub const fn expansion_possible(s_min: irang, s_max: irang, r_min: irang, r_max: irang) -> OperationPossibility {
    OperationPossibility::allow_if(r_min <= s_min && r_max >= s_max)
}

impl<const MIN: irang, const MAX: irang> Ranged<MIN, MAX>
where [u8; memlayout(MIN, MAX).bytes()]:
{
    /// Convert to the Ranged with the wider bounds
    pub const fn expand<const RMIN: irang, const RMAX: irang>(self) -> Ranged<RMIN, RMAX>
    where
        [u8; memlayout(RMIN, RMAX).bytes()]: ,
        Assert<{ expansion_possible(MIN, MAX, RMIN, RMAX) }>: IsAllowed,
    {
        unsafe { Ranged::__unsafe_new(self.get()) }
    }

    /// Convert to the other Ranged, returning None if the value is out of range
    pub const fn try_expand<const RMIN: irang, const RMAX: irang>(self) -> Option<Ranged<RMIN, RMAX>>
    where [u8; memlayout(RMIN, RMAX).bytes()]: {
        Ranged::<RMIN, RMAX>::new(self.get())
    }
}
