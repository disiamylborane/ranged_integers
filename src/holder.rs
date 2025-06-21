//! The internals for `Ranged` type

#![doc(hidden)]
#![allow(clippy::inline_always)]

pub trait Aligner
{
    // This marker traits guarantee the Ranged has the same ones
    type A: Copy + Send + Sync + Unpin + core::panic::UnwindSafe + core::panic::RefUnwindSafe + core::marker::Freeze + core::hash::Hash;
}

// The structure converting the const into a type, which has the desired alignment
struct AlignWrap<const N: usize>;

impl Aligner for AlignWrap<0> { type A = (); }
impl Aligner for AlignWrap<1> { type A = i8; }
impl Aligner for AlignWrap<2> { type A = i16; }
impl Aligner for AlignWrap<4> { type A = i32; }
impl Aligner for AlignWrap<8> { type A = i64; }
impl<const N: usize> Aligner for AlignWrap<N> {
    default type A = i64;  // This will never be the case, but it prevents the
                           // appearance of a new constraint AlignWrap:Aligner in Ranged type.
}

// The internal representation of `Ranged` is struct `RangedRepr`. It contains
// a needed-size byte array and a zero-size type to align it. So, the optimizer
// will pay attention to this, and the from_irang/to_irang methods will be reduced
// to no-operation (or a simple copy).
#[derive(Clone, Copy, Hash)]
pub struct RangedRepr<const N: usize>
{
    _align: [<AlignWrap<N> as Aligner>::A; 0],
    bytes: [u8; N],
}


impl<const N: usize> RangedRepr<N>
{
    const fn new() -> Self { Self{_align: [], bytes: [0; N]} }

    #[allow(clippy::inline_always)]
    #[inline(always)]
    pub(crate) const fn from_irang(v: i128) -> Self {
        let mut x = Self::new();
        let bytes = v.to_ne_bytes();

        match N {
            // Loops are problematic inside const fns, so we just manually go through the options.
            // The optimizer handles it.
            1 => {
                x.bytes[0] = bytes[0];
            }
            2 => {
                x.bytes[0] = bytes[0];
                x.bytes[1] = bytes[1];
            }
            4 => {
                x.bytes[0] = bytes[0];
                x.bytes[1] = bytes[1];
                x.bytes[2] = bytes[2];
                x.bytes[3] = bytes[3];
            }
            8 => {
                x.bytes[0] = bytes[0];
                x.bytes[1] = bytes[1];
                x.bytes[2] = bytes[2];
                x.bytes[3] = bytes[3];
                x.bytes[4] = bytes[4];
                x.bytes[5] = bytes[5];
                x.bytes[6] = bytes[6];
                x.bytes[7] = bytes[7];
            }
            _ => {} // The impossible case. Just suppress it.
        }

        x
    }

    #[allow(clippy::inline_always)]
    #[inline(always)]
    pub(crate) const fn to_irang(self, unsigned: bool) -> i128 {
        let b = self.bytes;
        // We do not introduce `unsigned` as const generic, since we do not want to introduce
        // an extra constraint in the inner code. However, the `unsigned` is always known at
        // compile time, so the method will be optimized to a simple copy.
        match N {
            1 => {
                if unsigned {
                    u8::from_ne_bytes([b[0]]) as i128
                } else {
                    i8::from_ne_bytes([b[0]]) as i128
                }
            }
            2 => {
                if unsigned {
                    u16::from_ne_bytes([b[0], b[1]]) as i128
                } else {
                    i16::from_ne_bytes([b[0], b[1]]) as i128
                }
            }
            4 => {
                if unsigned {
                    u32::from_ne_bytes([b[0], b[1], b[2], b[3]]) as i128
                } else {
                    i32::from_ne_bytes([b[0], b[1], b[2], b[3]]) as i128
                }
            }
            8 => {
                if unsigned {
                    u64::from_ne_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]) as i128
                } else {
                    i64::from_ne_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]) as i128
                }
            }
            _ => {0}  // The impossible case. Just suppress it.
        }
    }
}
