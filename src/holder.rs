//! The internals for `Ranged` type

#![doc(hidden)]
#![allow(clippy::inline_always)]

use core::marker::{ConstParamTy_};


pub const trait ShrinkWrap
    : Copy + Send + Sync + Unpin + core::panic::UnwindSafe + core::panic::RefUnwindSafe + core::marker::Freeze + core::hash::Hash
{
    fn put(v: i128) -> Self;
    fn get(self) -> i128;
}

macro_rules! impl_shrinkwrap {
    ($($t:tt)*) => {     
        $(   
            const impl ShrinkWrap for $t {
                #[inline(always)]
                fn put(v: i128) -> Self {#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] v as Self}
                #[inline(always)]
                fn get(self) -> i128 {self as i128}
            }
        )*
    };
}

impl_shrinkwrap!{i8 u8 i16 u16 i32 u32 i64 u64 i128}
const impl ShrinkWrap for () {
    #[inline(always)] fn put(_v: i128) {}
    #[inline(always)] fn get(self) -> i128 {0} // never called
}

pub const trait Placer
{
    // This marker traits guarantee the Ranged has the same ones
    type Representation: const ShrinkWrap;
}


#[derive(Clone, Copy)]
pub enum Layout {Trivial, I8, U8, I16, U16, I32, U32, I64, U64, Forbidden}

const impl PartialEq for Layout {
    fn eq(&self, other: &Self) -> bool {*self as u8 == *other as u8}
}
const impl Eq for Layout {}
impl core::marker::StructuralPartialEq for Layout {}
impl ConstParamTy_ for Layout {}


// The structure converting the const into a type, which has the desired alignment
struct SizeWrap<const L: Layout>;

impl Placer for SizeWrap<{Layout::Trivial}> { type Representation = (); }
impl Placer for SizeWrap<{Layout::I8}> { type Representation = i8; }
impl Placer for SizeWrap<{Layout::U8}> { type Representation = u8; }
impl Placer for SizeWrap<{Layout::I16}> { type Representation = i16; }
impl Placer for SizeWrap<{Layout::U16}> { type Representation = u16; }
impl Placer for SizeWrap<{Layout::I32}> { type Representation = i32; }
impl Placer for SizeWrap<{Layout::U32}> { type Representation = u32; }
impl Placer for SizeWrap<{Layout::I64}> { type Representation = i64; }
impl Placer for SizeWrap<{Layout::U64}> { type Representation = u64; }
impl<const N: Layout> Placer for SizeWrap<N> {
    // This will never be the case, but it prevents the
    // appearance of a new constraint SizeWrap:Placer in Ranged type.
    default type Representation = i64;
}

// The internal representation of `Ranged` is struct `RangedRepr`. It contains
// a needed-size byte array and a zero-size type to align it. So, the optimizer
// will pay attention to this, and the from_irang/to_irang methods will be reduced
// to no-operation (or a simple copy).
#[derive(Clone, Copy, Hash)]
pub struct RangedRepr<const LAYOUT: Layout> {
    bytes: <SizeWrap<LAYOUT> as Placer>::Representation,
}


impl<const N: Layout> RangedRepr<N>
{
    #[inline(always)]
    pub(crate) const fn from_irang(v: i128) -> Self {
        Self{bytes: <SizeWrap<N> as Placer>::Representation::put(v)}
    }

    #[inline(always)]
    pub(crate) const fn to_irang(self) -> i128 {self.bytes.get()}
}
