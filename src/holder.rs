// The internals for `Ranged` type

// The internal representation of `Ranged` is struct `RangedRepr`. It contains
// a needed-size byte array and a zero-size type to align it.

//! The internals for `Ranged` type

#![doc(hidden)]

use super::irang;

// A helper trait specifying the alignment
pub trait Aligner
{
    type A: Copy;
}

// A helper enum specifying the amount of bytes needed for a Ranged
#[derive(PartialEq, Eq, Copy, Clone, core::marker::ConstParamTy)]
#[allow(non_camel_case_types)]
pub enum IntLayout {
    i8, u8, i16, u16, i32, u32, i64, u64, i128, Trivial
}
impl IntLayout {
    #[doc(hidden)]
    pub const fn bytes(self) -> usize {
        match self {
            Self::i8 => 1,
            Self::u8 => 1,
            Self::i16 => 2,
            Self::u16 => 2,
            Self::i32 => 4,
            Self::u32 => 4,
            Self::i64 => 8,
            Self::u64 => 8,
            Self::i128 => 16,
            Self::Trivial => 0,
        }
    }
}


// Convert the IntLayout into the corresponding type
#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct AlignWrap<const N: IntLayout>;
// the internal type must have the alignment of the corresponding integer
impl Aligner for AlignWrap<{IntLayout::i8}> { type A = i8; }
impl Aligner for AlignWrap<{IntLayout::u8}> { type A = u8; }
impl Aligner for AlignWrap<{IntLayout::i16}> { type A = i16; }
impl Aligner for AlignWrap<{IntLayout::u16}> { type A = u16; }
impl Aligner for AlignWrap<{IntLayout::i32}> { type A = i32; }
impl Aligner for AlignWrap<{IntLayout::u32}> { type A = u32; }
impl Aligner for AlignWrap<{IntLayout::i64}> { type A = i64; }
impl Aligner for AlignWrap<{IntLayout::u64}> { type A = u64; }
impl Aligner for AlignWrap<{IntLayout::i128}> { type A = i128; }
impl Aligner for AlignWrap<{IntLayout::Trivial}> { type A = u8; }

// The internal representation of Ranged: an array of bytes with the length and alignmemt ensured
#[derive(Clone, Copy)]
pub(crate) struct RangedRepr<const LAYOUT: IntLayout>
where
    AlignWrap<LAYOUT>: Aligner,
    [u8; LAYOUT.bytes()]:,
{
    // Ensure the alignment
    _align: [<AlignWrap<LAYOUT> as Aligner>::A; 0],
    // Bytewise access
    bytes: [u8; LAYOUT.bytes()],
}

// Convert NumberBytes to and from integers.
// This code heavily relies on optimization
impl<const LAYOUT: IntLayout> RangedRepr<LAYOUT>
where
    AlignWrap<LAYOUT> : Aligner,
    [(); LAYOUT.bytes()]:
{
    const fn new() -> Self { Self{_align: [], bytes: [0; LAYOUT.bytes()]} }

    #[inline(always)]
    pub(crate) const fn from_irang(v: irang) -> Self {
        let mut x = Self::new();
        let bytes = v.to_ne_bytes();
        match LAYOUT {
            IntLayout::Trivial => {
            }
            IntLayout::i8 => {
                x.bytes[0] = bytes[0];
            }
            IntLayout::u8 => {
                x.bytes[0] = bytes[0];
            }
            IntLayout::i16 => {
                x.bytes[0] = bytes[0];
                x.bytes[1] = bytes[1];
            }
            IntLayout::u16 => {
                x.bytes[0] = bytes[0];
                x.bytes[1] = bytes[1];
            }
            IntLayout::i32 => {
                x.bytes[0] = bytes[0];
                x.bytes[1] = bytes[1];
                x.bytes[2] = bytes[2];
                x.bytes[3] = bytes[3];
            }
            IntLayout::u32 => {
                x.bytes[0] = bytes[0];
                x.bytes[1] = bytes[1];
                x.bytes[2] = bytes[2];
                x.bytes[3] = bytes[3];
            }
            IntLayout::i64 => {
                x.bytes[0] = bytes[0];
                x.bytes[1] = bytes[1];
                x.bytes[2] = bytes[2];
                x.bytes[3] = bytes[3];
                x.bytes[4] = bytes[4];
                x.bytes[5] = bytes[5];
                x.bytes[6] = bytes[6];
                x.bytes[7] = bytes[7];
            }
            IntLayout::u64 => {
                x.bytes[0] = bytes[0];
                x.bytes[1] = bytes[1];
                x.bytes[2] = bytes[2];
                x.bytes[3] = bytes[3];
                x.bytes[4] = bytes[4];
                x.bytes[5] = bytes[5];
                x.bytes[6] = bytes[6];
                x.bytes[7] = bytes[7];
            }
            IntLayout::i128 => {
                x.bytes[0] = bytes[0];
                x.bytes[1] = bytes[1];
                x.bytes[2] = bytes[2];
                x.bytes[3] = bytes[3];
                x.bytes[4] = bytes[4];
                x.bytes[5] = bytes[5];
                x.bytes[6] = bytes[6];
                x.bytes[7] = bytes[7];
                x.bytes[8] = bytes[8];
                x.bytes[9] = bytes[9];
                x.bytes[10] = bytes[10];
                x.bytes[11] = bytes[11];
                x.bytes[12] = bytes[12];
                x.bytes[13] = bytes[13];
                x.bytes[14] = bytes[14];
                x.bytes[15] = bytes[15];
            }
        }

        x
    }


    #[inline(always)]
    pub(crate) const fn to_irang(self) -> irang {
        let b = self.bytes;
        match LAYOUT {
            IntLayout::Trivial => {
                0
            }
            IntLayout::i8 => {
                i8::from_ne_bytes([b[0]]) as irang
            }
            IntLayout::u8 => {
                u8::from_ne_bytes([b[0]]) as irang
            }
            IntLayout::i16 => {
                i16::from_ne_bytes([b[0], b[1]]) as irang
            }
            IntLayout::u16 => {
                u16::from_ne_bytes([b[0], b[1]]) as irang
            }
            IntLayout::i32 => {
                i32::from_ne_bytes([b[0], b[1], b[2], b[3]]) as irang
            }
            IntLayout::u32 => {
                u32::from_ne_bytes([b[0], b[1], b[2], b[3]]) as irang
            }
            IntLayout::i64 => {
                i64::from_ne_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]) as irang
            }
            IntLayout::u64 => {
                u64::from_ne_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]) as irang
            }
            IntLayout::i128 => {
                i128::from_ne_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7], b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]]) as irang
            }
        }
    }
}

