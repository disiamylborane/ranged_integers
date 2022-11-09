// The internals for `Ranged` type

// The internal representation of `Ranged` is struct `RangedRepr`. It contains
// a `Shrinkable` (capable to be casted to/from i128) type with the required
// size.



#![doc(hidden)]
#![allow(clippy::inline_always)]

use super::irang;

// Shrinkable type can be casted to/from irang
// It must be implemented for any type inside Ranged
#[const_trait]
trait Shrinkable: Sized + Copy + ~const core::marker::Destruct {
    fn shrinkfrom(_: irang) -> Self;
    fn shrinkinto(self) -> irang;
}

// A type-resolving trait, which specifies the integer size
#[const_trait]
trait RangedPrimitiveSelector {
    type Primitive: ~const Shrinkable;
}

macro_rules! describe_primitives {
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

            impl const RangedPrimitiveSelector for RangedTypeGenerator<{ IntLayout::$repr }> {
                type Primitive = $name;
            }
        )*
    };
}


describe_primitives!{
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
    #[inline(always)] fn shrinkfrom(_: irang) -> Self { Self }
    #[inline(always)] fn shrinkinto(self) -> irang { unreachable!() }
}


// A helper enum specifying the amount of bytes needed for a Ranged
// and a data type inside Ranged
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


// Convert the `IntLayout` into the corresponding type
//
// The trait RangedPrimitiveSelector is implemented for this type with
// the specialization for any of the variants of IntLayout. The
// RangedPrimitiveSelector::Primitive type is a type which is needed to
// hold the specific IntLayout
#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct RangedTypeGenerator<const N: IntLayout>;

impl<const N: IntLayout> const RangedPrimitiveSelector for RangedTypeGenerator<N> {
    default type Primitive = I128;
}

impl const RangedPrimitiveSelector for RangedTypeGenerator<{ IntLayout::Trivial }> {
    type Primitive = Trivial;
}


// The internal representation of Ranged
//
// Having the IntLayout parameter, it chooses a primitive data type to be
// hold inside with the help of <RangedTypeGenerator as RangedPrimitiveSelector>
// to select a needed primitive type, one of Trivial (zero-sized) or an integer
// primitive
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct RangedRepr<const LAYOUT: IntLayout>
{
    bytes: <RangedTypeGenerator<LAYOUT> as RangedPrimitiveSelector>::Primitive,
}

// Convert NumberBytes to and from integers.
impl<const LAYOUT: IntLayout> RangedRepr<LAYOUT>
{
    #[inline(always)]
    pub(crate) const fn from_irang(v: irang) -> Self {
        Self {bytes: <RangedTypeGenerator<LAYOUT> as RangedPrimitiveSelector>::Primitive::shrinkfrom(v)}
    }

    #[inline(always)]
    pub(crate) const fn to_irang(self) -> irang {
        self.bytes.shrinkinto()
    }
}

