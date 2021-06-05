//! The internals for `Ranged` type

#![doc(hidden)]

use super::irang;

// A helper trait specifying the alignment
pub trait Aligner
{
    type A: Copy;
}

// A helper enum specifying the amount of bytes needed for a Ranged
#[derive(PartialEq, Eq, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum IntLayout {
    i8, u8, i16, u16, i32, u32, i64, u64, i128
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
        }
    }
}

impl<const N: IntLayout> Aligner for AlignWrap<N> { default type A = i128; }

// Convert the IntLayout into the corresponding type
#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct AlignWrap<const N: IntLayout>;
impl Aligner for AlignWrap<{IntLayout::i8}> { type A = i8; }
impl Aligner for AlignWrap<{IntLayout::u8}> { type A = u8; }
impl Aligner for AlignWrap<{IntLayout::i16}> { type A = i16; }
impl Aligner for AlignWrap<{IntLayout::u16}> { type A = u16; }
impl Aligner for AlignWrap<{IntLayout::i32}> { type A = i32; }
impl Aligner for AlignWrap<{IntLayout::u32}> { type A = u32; }
impl Aligner for AlignWrap<{IntLayout::i64}> { type A = i64; }
impl Aligner for AlignWrap<{IntLayout::u64}> { type A = u64; }
impl Aligner for AlignWrap<{IntLayout::i128}> { type A = i128; }

// The internal representation of Ranged: an array of bytes with the length and alignmemt ensured
#[derive(Clone, Copy)]
pub(crate) struct NumberBytes<const LAYOUT: IntLayout>
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
    [(); LAYOUT.bytes()]:
{
    const fn new() -> Self { Self{_align: [], bytes: [0; LAYOUT.bytes()]} }

    #[inline(always)]
    pub(crate) const fn from_irang(v: irang) -> Self {
        let mut x = Self::new();
        let bytes = v.to_ne_bytes();

        let mut i=0;
        while i < x.bytes.len() {
            x.bytes[i] = bytes[i];
            i += 1;
        }

        x
    }


    #[inline(always)]
    pub(crate) const fn to_irang(self) -> irang {
        let b = self.bytes;
        match LAYOUT {
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
