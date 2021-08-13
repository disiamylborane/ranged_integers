//! The internals for `Ranged` type

#![doc(hidden)]

use super::irang;

// A helper trait specifying the alignment
pub trait Aligner {
    type A: Copy;
}

#[derive(PartialEq, Eq, Copy, Clone)]
#[doc(hidden)]
pub struct Trivial;

// A helper enum specifying the amount of bytes needed for a Ranged
#[derive(PartialEq, Eq, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum IntLayout {Trivial, i8, u8, i16, u16, i32, u32, i64, u64, i128}

impl IntLayout {
    #[must_use]
    #[doc(hidden)]
    pub const fn bytes(self) -> usize {
        macro_rules! get_typesize {
            (  $($tt:ident)* ) => {
                match self {
                    $(
                        Self::$tt => {core::mem::size_of::<$tt>()}
                    )+
                }
            };
        }

        get_typesize! {Trivial i8 u8 i16 u16 i32 u32 i64 u64 i128 }
    }
}

// The usage of specialization isn't necessary,
// but it excludes the need of AlignWrap<...>: Aligner
// trait bound specification for every generic Ranged
impl<const N: IntLayout> Aligner for AlignWrap<N> {
    default type A = i128;
}

macro_rules! alignwrap {
    (  $($tt:ident)* ) => {
        $(
            impl const Aligner for AlignWrap<{ IntLayout::$tt }> {
                type A = $tt;
            }
        )+
    };
}

// Convert the IntLayout into the corresponding type
#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct AlignWrap<const N: IntLayout>;
alignwrap!{Trivial i8 u8 i16 u16 i32 u32 i64 u64 i128 }

// The internal representation of Ranged: an array of bytes with the length and alignmemt ensured
#[derive(Clone, Copy)]
pub struct NumberBytes<const LAYOUT: IntLayout>
where
    [(); LAYOUT.bytes()]:,
{
    // Ensure the alignment
    _align: [<AlignWrap<LAYOUT> as Aligner>::A; 0],
    // Bytewise access
    bytes: [u8; LAYOUT.bytes()],
}

// Convert NumberBytes to and from integers.
// This code heavily relies on optimization
impl<const LAYOUT: IntLayout> NumberBytes<LAYOUT>
where
    [(); LAYOUT.bytes()]: ,
{
    #[inline(always)]
    pub(crate) const fn from_irang(v: irang) -> Self {
        macro_rules! conv_from_irang {
            (  $($tt:ident)* ) => {
                match const{LAYOUT} {
                    IntLayout::Trivial => {Self {_align:[], bytes: [0; LAYOUT.bytes()]}}
                    $(
                        IntLayout::$tt => {
                            Self {_align:[], bytes: unsafe{ *((v as $tt).to_ne_bytes().as_ptr() as *const _) }}
                        }
                    )+
                }
            };
        }

        conv_from_irang! {i8 u8 i16 u16 i32 u32 i64 u64 i128}
    }

    #[inline(always)]
    pub(crate) const fn to_irang(self) -> irang {
        macro_rules! conv_to_irang {
            (  $($tt:ident)* ) => {
                match const{LAYOUT} {
                    IntLayout::Trivial => {unreachable!()}
                    $(
                        IntLayout::$tt => {
                            $tt::from_ne_bytes(unsafe{*(self.bytes.as_ptr() as *const _)}) as irang
                        }
                    )+
                }
            };
        }

        conv_to_irang! { i8 u8 i16 u16 i32 u32 i64 u64 i128 }
    }
}
