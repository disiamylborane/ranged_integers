//! The internals for `Ranged` type

#![doc(hidden)]
#![allow(clippy::inline_always)]

use super::irang;

// A trait implemented for any struct that may represent
// a ranged integer. It may be converted to and from irang.
#[const_trait]
pub trait Shrinkable: Sized + Copy + ~const core::marker::Destruct {
    fn shrinkfrom(_: irang) -> Self;
    fn shrinkinto(self) -> irang;
}

// A type-resolving trait specifying the integer size
#[const_trait]
pub trait IntSize {
    type Primitive: ~const Shrinkable;
}

macro_rules! wrap_primitive {
    ($($name:ident $repr:ident,)*) => {
        $(
            #[repr(transparent)]
            #[derive(Clone, Copy)]
            pub struct $name {inner: $repr}

            impl const Shrinkable for $name {
                #[inline(always)]
                fn shrinkfrom(v: irang) -> Self { Self{inner: v as $repr} }
                #[inline(always)]
                fn shrinkinto(self) -> irang { self.inner as irang }
            }

            impl const IntSize for IntSizeWrap<{ IntLayout::$repr }> {
                type Primitive = $name;
            }
        )*
    };
}


wrap_primitive!{
    U8 u8,
    U16 u16,
    U32 u32,
    U64 u64,
    I8 i8,
    I16 i16,
    I32 i32,
    I64 i64,
    I128 i128,
}

#[derive(PartialEq, Eq, Clone, Copy)]
#[doc(hidden)]
pub struct Trivial;
impl const Shrinkable for Trivial {
    #[inline(always)]
    fn shrinkfrom(_: irang) -> Self { Self }
    #[inline(always)]
    fn shrinkinto(self) -> irang { unreachable!() }
}

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


// Convert the IntLayout into the corresponding type
#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct IntSizeWrap<const N: IntLayout>;

impl<const N: IntLayout> const IntSize for IntSizeWrap<N> {
    default type Primitive = I128;
}

impl const IntSize for IntSizeWrap<{ IntLayout::Trivial }> {
    type Primitive = Trivial;
}


// The internal representation of Ranged
#[derive(Clone, Copy)]
pub struct NumberBytes<const LAYOUT: IntLayout>
{
    bytes: <IntSizeWrap<LAYOUT> as IntSize>::Primitive,
}

// Convert NumberBytes to and from integers.
impl<const LAYOUT: IntLayout> NumberBytes<LAYOUT>
{
    #[inline(always)]
    pub(crate) const fn from_irang(v: irang) -> Self {
        Self {bytes: <IntSizeWrap<LAYOUT> as IntSize>::Primitive::shrinkfrom(v)}
    }

    #[inline(always)]
    pub(crate) const fn to_irang(self) -> irang {
        self.bytes.shrinkinto()
    }
}

