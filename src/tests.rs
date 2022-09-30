#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::cognitive_complexity)]

use std::prelude::v1::*;

use super::*;
use core::ops::Add;
use core::ops::Div;
use core::ops::Mul;
use core::ops::Sub;

#[test]
fn sizes() {
    use core::mem::{align_of, size_of};

    macro_rules! sz_align {
        ($sz:ty, $t:ty) => {
            assert_eq!(size_of::<$t>(), size_of::<$sz>());
            assert_eq!(align_of::<$t>(), align_of::<$sz>());
        };
    }

    sz_align!((), Ranged<0,0>);
    sz_align!((), Ranged<1,1>);
    sz_align!((), Ranged<-1,-1>);
    sz_align!((), Ranged<100, 100>);
    sz_align!((), Ranged<-100, -100>);
    sz_align!((), Ranged<500, 500>);
    sz_align!((), Ranged<-500, -500>);
    sz_align!((), Ranged<100_000, 100_000>);
    sz_align!((), Ranged<-100_000, -100_000>);
    sz_align!((), Ranged<10_000_000_000, 10_000_000_000>);
    sz_align!((), Ranged<-10_000_000_000, -10_000_000_000>);
    sz_align!((), Ranged<18_446_744_073_709_551_616, 18_446_744_073_709_551_616>);
    sz_align!((), Ranged<-18_446_744_073_709_551_616, -18_446_744_073_709_551_616>);

    sz_align!((), Ranged<-32768, -32768>);
    sz_align!((), Ranged<32767, 32767>);
    sz_align!((), Ranged<65535, 65535>);
    sz_align!((), Ranged<65536, 65536>);

    sz_align!(i8, Ranged<10,11>);
    sz_align!(i8, Ranged<254,255>);
    sz_align!(i8, Ranged<126,127>);
    sz_align!(i8, Ranged<-128, -127>);
    sz_align!(i8, Ranged<0,10>);
    sz_align!(i8, Ranged<0,127>);
    sz_align!(i8, Ranged<0,255>);
    sz_align!(i8, Ranged<127,255>);
    sz_align!(i8, Ranged<-128, 127>);

    sz_align!(i16, Ranged<-128, 128>);

    sz_align!(i16, Ranged<-32768, 32767>);
    sz_align!(i16, Ranged<0, 32768>);
    sz_align!(i16, Ranged<0, 65535>);
    sz_align!(i16, Ranged<-32768, -32767>);
    sz_align!(i16, Ranged<32766, 32767>);
    sz_align!(i16, Ranged<65534, 65535>);

    sz_align!(i32, Ranged<-32768, 32768>);
    sz_align!(i32, Ranged<0, 65536>);

    sz_align!(i32, Ranged<0, 4_294_967_295>);
    sz_align!(i32, Ranged<-2_147_483_648, 2_147_483_647>);
    sz_align!(i32, Ranged<100, 10_000_000>);
    sz_align!(i32, Ranged<-100, 10_000_000>);
    sz_align!(i32, Ranged<100, 2_147_483_647>);
    sz_align!(i32, Ranged<-100, 2_147_483_647>);

    sz_align!(i64, Ranged<-1, 4_294_967_295>);
    sz_align!(i64, Ranged<0, 4_294_967_296>);
    sz_align!(i64, Ranged<-2_147_483_649, 2_147_483_647>);
    sz_align!(i64, Ranged<-2_147_483_648, 2_147_483_648>);

    sz_align!(i64, Ranged<0, 18_446_744_073_709_551_615>);
    sz_align!(i64, Ranged<-9_223_372_036_854_775_808, 9_223_372_036_854_775_807>);

    sz_align!(i128, Ranged<-1, 18_446_744_073_709_551_615>);

    sz_align!(i128, Ranged<0, 18_446_744_073_709_551_616>);
    sz_align!(i128, Ranged<-9_223_372_036_854_775_809, 9_223_372_036_854_775_807>);
    sz_align!(i128, Ranged<-9_223_372_036_854_775_808, 9_223_372_036_854_775_808>);
}

#[test]
fn print_val() {
    macro_rules! assert_val {
        ([$min:literal $max:literal] $x:literal) => {
            assert_eq!(r!([$min $max] $x).get(), $x);
        }
    }

    for i in 0..126 {
        let w = 2_i128.pow(i);
        assert_eq!(w.as_ranged().get(), w);
        assert_eq!((-w).as_ranged().get(), -w);
    }

    assert_val!([-128 127] -128);
    assert_val!([-128 127] -127);
    assert_val!([-128 127] -30);
    assert_val!([-128 127] -1);
    assert_val!([-128 127] 0);
    assert_val!([-128 127] 1);
    assert_val!([-128 127] 30);
    assert_val!([-128 127] 127);

    assert_val!([0 200] 0);
    assert_val!([0 200] 1);
    assert_val!([0 200] 30);
    assert_val!([0 200] 127);
    assert_val!([0 200] 128);
    assert_val!([0 200] 200);

    assert_val!([-1000 1000] -500);
    assert_val!([-1000 1000] -100);
    assert_val!([-1000 1000] 0);
    assert_val!([-1000 1000] 100);
    assert_val!([-1000 1000] 500);

    assert_val!([0 1000] 0);
    assert_val!([0 1000] 100);
    assert_val!([0 1000] 500);

    assert_val!([0 65535] 0);
    assert_val!([0 65535] 1);
    assert_val!([0 65535] 65534);
    assert_val!([0 65535] 65535);

    let x = r!(42);
    assert_eq!(format!("{}", x), "42");
    assert_eq!(format!("{:?}", x), "r!(42)");

    let x = r!(400_000);
    assert_eq!(format!("{}", x), "400000");

    let x = r!(4000);
    assert_eq!(format!("{}", x), "4000");

    let x = r!(40);
    assert_eq!(format!("{}", x), "40");

    let x = r!(0);
    assert_eq!(format!("{}", x), "0");

    let x = r!(-400_000);
    assert_eq!(format!("{}", x), "-400000");

    let x = r!(-4000);
    assert_eq!(format!("{}", x), "-4000");

    let x = r!(-40);
    assert_eq!(format!("{}", x), "-40");
}

#[test]
fn ranged_macro() {
    let x = r! {[0 4] 2};
    assert_eq!(format!("{}", x), "2");
    assert_eq!(format!("{:?}", x), "r!([0 4] 2)");

    let x: Ranged<2, 15> = r!([] 10);
    assert_eq!(format!("{}", x), "10");
    assert_eq!(format!("{:?}", x), "r!([2 15] 10)");

    let x = r!(10);
    assert_eq!(format!("{}", x), "10");
    assert_eq!(format!("{:?}", x), "r!(10)");
}

#[test]
fn addsub() {
    let a = r!(20) + r!(22);
    assert_eq!(format!("{}", a), "42");
    assert_eq!(format!("{:?}", a), "r!(42)");

    let a = r!(20).add(r!(22));
    assert_eq!(format!("{}", a), "42");
    assert_eq!(format!("{:?}", a), "r!(42)");

    let a = 15_u32 % r!(20);
    let b = a + a;
    assert_eq!(format!("{}", b), "30");
    assert_eq!(format!("{:?}", b), "r!([0 38] 30)");

    let c = r!(22) - r!(20);
    assert_eq!(format!("{}", c), "2");
    assert_eq!(format!("{:?}", c), "r!(2)");

    let c = r!(22).sub(r!(20));
    assert_eq!(format!("{}", c), "2");
    assert_eq!(format!("{:?}", c), "r!(2)");
}

#[test]
fn mul() {
    let a = r!(20) * r!(20);
    assert_eq!(format!("{}", a), "400");
    assert_eq!(format!("{:?}", a), "r!(400)");

    let b = r! {[-3 3] 1} * r! {[-3 3] 2};
    assert_eq!(format!("{}", b), "2");
    assert_eq!(format!("{:?}", b), "r!([-9 9] 2)");

    let c = Ranged::<{ -3 }, 0>::new(-1).unwrap() * Ranged::<0, 3>::new(2).unwrap();
    assert_eq!(format!("{}", c), "-2");
    assert_eq!(format!("{:?}", c), "r!([-9 0] -2)");

    let b = r! {[-30000 30000] 1} * r! {[-3 3] 2};
    assert_eq!(format!("{}", b), "2");
    assert_eq!(format!("{:?}", b), "r!([-90000 90000] 2)");

    let a = r!(20).mul(r!(20));
    assert_eq!(format!("{}", a), "400");
    assert_eq!(format!("{:?}", a), "r!(400)");

    let b = r! {[-3 3] 1}.mul(r! {[-3 3] 2});
    assert_eq!(format!("{}", b), "2");
    assert_eq!(format!("{:?}", b), "r!([-9 9] 2)");

    let c = Ranged::<{ -3 }, 0>::new(-1)
        .unwrap()
        .mul(Ranged::<0, 3>::new(2).unwrap());
    assert_eq!(format!("{}", c), "-2");
    assert_eq!(format!("{:?}", c), "r!([-9 0] -2)");

    let b = r! {[-30000 30000] 1}.mul(r! {[-3 3] 2});
    assert_eq!(format!("{}", b), "2");
    assert_eq!(format!("{:?}", b), "r!([-90000 90000] 2)");
}

#[test]
fn div() {
    let a = r!(20) / r!(20);
    assert_eq!(format!("{}", a), "1");
    assert_eq!(format!("{:?}", a), "r!(1)");

    let a = r!(20).div(r!(20));
    assert_eq!(format!("{}", a), "1");
    assert_eq!(format!("{:?}", a), "r!(1)");

    let a = r!([0 100] 20) / r!([1 10] 5);
    assert_eq!(format!("{}", a), "4");
    assert_eq!(format!("{:?}", a), "r!([0 100] 4)");

    let a = r!([0 100] 20) / r![[-10 -1] - 5];
    assert_eq!(format!("{}", a), "-4");
    assert_eq!(format!("{:?}", a), "r!([-100 0] -4)");

    let a = r!([-100 0] -20).div_euclid(r![[-10 - 1] - 5]);
    assert_eq!(format!("{}", a), "4");
    assert_eq!(format!("{:?}", a), "r!([0 100] 4)");

    let a = r!([-100 0] -20) / r!([1 10] 5);
    assert_eq!(format!("{}", a), "-4");
    assert_eq!(format!("{:?}", a), "r!([-100 0] -4)");

    let a = r!([100 1000] 500) / r!([1 6] 5);
    assert_eq!(format!("{}", a), "100");
    assert_eq!(format!("{:?}", a), "r!([16 1000] 100)");

    let a = r!([100 1000] 500) / r![[-6 - 1] - 5];
    assert_eq!(format!("{}", a), "-100");
    assert_eq!(format!("{:?}", a), "r!([-1000 -16] -100)");

    let _: Ranged<0, 50> = r!([0 101] 17) / r![[2 10] 5];
    let _: Ranged<0, 50> = r!([0 101] 17).div_euclid(r![[2 10] 5]);
    let _: Ranged<-50, 0> = r!([-101 0] -17) / r![[2 10] 5];
    let _: Ranged<-51, 0> = r!([-101 0] -17).div_euclid(r![[2 10] 5]);

    let _: Ranged<-50, 0> = r!([0 101] 17) / r![[-10 -2] -5];
    let _: Ranged<-50, 0> = r!([0 101] 17).div_euclid(r![[-10 -2] -5]);
    let _: Ranged<0, 50> = r!([-101 0] -17) / r![[-10 -2] -5];
    let _: Ranged<0, 51> = r!([-101 0] -17).div_euclid(r![[-10 -2] -5]);
}

#[test]
fn rem() {
    macro_rules! chrem{
        ($a:literal / $b:literal) => {{
            assert_eq!(r!($a)/r!($b)*r!($b) + r!($a)%r!($b), r!($a));
            assert_eq!(r!($a).div_euclid(r!($b))*r!($b) + r!($a).rem_euclid(r!($b)), r!($a));
        }}
    }

    chrem!(9 / 5);
    chrem!(10 / 5);
    chrem!(11 / 5);
    chrem!(9 / -5);
    chrem!(10 / -5);
    chrem!(11 / -5);
    chrem!(-9 / 5);
    chrem!(-10 / 5);
    chrem!(-11 / 5);
    chrem!(-9 / -5);
    chrem!(-10 / -5);
    chrem!(-11 / -5);

    // Rem operation for primitives
    let a: Ranged<0, 2> = 7_u8 % r!(3);
    assert_eq!(a, 1);
    let a: Ranged<0, 2> = 7_u8 % r!(-3);
    assert_eq!(a, 1);
    let a: Ranged<-2, 2> = 7_i8 % r!(3);
    assert_eq!(a, 1);
    let a: Ranged<-2, 2> = 7_i8 % r!(-3);
    assert_eq!(a, 1);
    let a: Ranged<-2, 2> = -7_i8 % r!(3);
    assert_eq!(a, -1);
    let a: Ranged<-2, 2> = -7_i8 % r!(-3);
    assert_eq!(a, -1);

    // Value checks
    assert_eq!(r!(25) % r!(20), 5);
    assert_eq!(r!(25) % r!(-20), 5);
    assert_eq!(r!(-25) % r!(20), -5);
    assert_eq!(r!(-25) % r!(-20), -5);
    assert_eq!(r!(25).rem_euclid(r!(20)), 5);
    assert_eq!(r!(25).rem_euclid(r!(-20)), 5);
    assert_eq!(r!(-25).rem_euclid(r!(20)), 15);
    assert_eq!(r!(-25).rem_euclid(r!(-20)), 15);

    // Range checks, Tier 1/5: constant values
    let _: Ranged<0, 0> = r!(25) % r!(25);
    let _: Ranged<5, 5> = r!(25) % r!(20);
    let _: Ranged<5, 5> = r!(25) % r!(-20);
    let _: Ranged<-5, -5> = r!(-25) % r!(20);
    let _: Ranged<-5, -5> = r!(-25) % r!(-20);

    let _: Ranged<0, 0> = r!(25).rem_euclid(r!(25));
    let _: Ranged<5, 5> = r!(25).rem_euclid(r!(20));
    let _: Ranged<5, 5> = r!(25).rem_euclid(r!(-20));
    let _: Ranged<15, 15> = r!(-25).rem_euclid(r!(20));
    let _: Ranged<15, 15> = r!(-25).rem_euclid(r!(-20));

    // Range checks, Tier 2/5: small range by constant value
    let _: Ranged<3, 6> = r!([23 26] 23) % r!(20);
    let _: Ranged<3, 6> = r!([23 26] 23) % r!(-20);
    let _: Ranged<-6, -3> = r!([-26 -23] -23) % r!(20);
    let _: Ranged<-6, -3> = r!([-26 -23] -23) % r!(-20);

    let _: Ranged<3, 6> = r!([23 26] 23).rem_euclid(r!(20));
    let _: Ranged<3, 6> = r!([23 26] 23).rem_euclid(r!(-20));
    let _: Ranged<14, 17> = r!([-26 -23] -23).rem_euclid(r!(20));
    let _: Ranged<14, 17> = r!([-26 -23] -23).rem_euclid(r!(-20));

    // Range checks, Tier 2a/5: small range must fail
    let _: Ranged<0, 19> = r!([19 21] 21) % r!(20);
    let _: Ranged<0, 19> = r!([19 21] 21) % r!(-20);
    let _: Ranged<-19, 0> = r!([-21 -19] -21) % r!(20);
    let _: Ranged<-19, 0> = r!([-21 -19] -21) % r!(-20);

    let _: Ranged<0, 19> = r!([19 21] 21).rem_euclid(r!(20));
    let _: Ranged<0, 19> = r!([19 21] 21).rem_euclid(r!(-20));
    let _: Ranged<0, 19> = r!([-21 -19] -21).rem_euclid(r!(20));
    let _: Ranged<0, 19> = r!([-21 -19] -21).rem_euclid(r!(-20));

    // Range checks, Tier 3/5: positive dividend
    // 3a: large dividend, small divisor
    let _: Ranged<0, 39> = r!([2 400] 21) % r!([1 40] 10);
    let _: Ranged<0, 39> = r!([2 400] 21) % r!([-40 -1] -10);
    let _: Ranged<0, 39> = r!([2 400] 21).rem_euclid(r!([1 40] 10));
    let _: Ranged<0, 39> = r!([2 400] 21).rem_euclid(r!([-40 -1] -10));
    // 3b: large divisor, small dividend
    let _: Ranged<0, 20> = r!([2 20] 17) % r!([1 400] 10);
    let _: Ranged<0, 20> = r!([2 20] 17) % r!([-400 -1] -10);
    let _: Ranged<0, 20> = r!([2 20] 17).rem_euclid(r!([1 400] 10));
    let _: Ranged<0, 20> = r!([2 20] 17).rem_euclid(r!([-400 -1] -10));

    // Range checks, Tier 4/5: negative dividend
    // 4a: large dividend, small divisor
    let _: Ranged<-39, 0> = r!([-400 -2] -21) % r!([1 40] 10);
    let _: Ranged<-39, 0> = r!([-400 -2] -21) % r!([-40 -1] -10);
    let _: Ranged<0, 39> = r!([-400 -2] -21).rem_euclid(r!([1 40] 10));
    let _: Ranged<0, 39> = r!([-400 -2] -21).rem_euclid(r!([-40 -1] -10));
    // 4b: large divisor, small dividend
    let _: Ranged<-20, 0> = r!([-20 -2] -17) % r!([1 400] 10);
    let _: Ranged<-20, 0> = r!([-20 -2] -17) % r!([-400 -1] -10);
    let _: Ranged<0, 399> = r!([-20 -2] -17).rem_euclid(r!([1 400] 10));  // FIXME: Something is possible to be done
    let _: Ranged<0, 399> = r!([-20 -2] -17).rem_euclid(r!([-400 -1] -10));

    // Range checks, Tier 5/5: wide dividend
    let _: Ranged<-39, 39> = r!([-400 400] 17) % r!([1 40] 10);
    let _: Ranged<-39, 39> = r!([-400 400] 17) % r!([-40 -1] -10);
    let _: Ranged<-20, 39> = r!([-20 400] 17) % r!([1 40] 10);
    let _: Ranged<-20, 39> = r!([-20 400] 17) % r!([-40 -1] -10);
    let _: Ranged<-39, 20> = r!([-400 20] 17) % r!([1 40] 10);
    let _: Ranged<-39, 20> = r!([-400 20] 17) % r!([-40 -1] -10);
    let _: Ranged<-20, 20> = r!([-20 20] 17) % r!([1 40] 10);
    let _: Ranged<-20, 20> = r!([-20 20] 17) % r!([-40 -1] -10);

    let _: Ranged<0, 39> = r!([-400 400] 17).rem_euclid(r!([1 40] 10));
    let _: Ranged<0, 39> = r!([-400 400] 17).rem_euclid(r!([-40 -1] -10));
    let _: Ranged<0, 39> = r!([-20 400] 17).rem_euclid(r!([1 40] 10));
    let _: Ranged<0, 39> = r!([-20 400] 17).rem_euclid(r!([-40 -1] -10));
    let _: Ranged<0, 39> = r!([-400 20] 17).rem_euclid(r!([1 40] 10));
    let _: Ranged<0, 39> = r!([-400 20] 17).rem_euclid(r!([-40 -1] -10));
    let _: Ranged<0, 39> = r!([-20 20] 17).rem_euclid(r!([1 40] 10));
    let _: Ranged<0, 39> = r!([-20 20] 17).rem_euclid(r!([-40 -1] -10));
}

#[test]
fn eq() {
    let a = r!(20);
    let b = r!(40);
    assert_eq!(a, a);
    assert_eq!(15 % a, 15 % b);
    assert!(10 % a + 15 % b == 15 % a + 10 % b);
    assert!(11 % a + 15 % b != 15 % a + 10 % b);
    assert!(a != b);

    assert!(r!(40) > r!(20));
    assert!(r!(40) >= r!(20));
    assert!(r!(10) < r!(20));
    assert!(r!(10) <= r!(20));

    assert!(r!(40) > 20);
    assert!(r!(40) >= 20);
    assert!(r!(10) < 20);
    assert!(r!(10) <= 20);

    assert!(40 > r!(20));
    assert!(40 >= r!(20));
    assert!(10 < r!(20));
    assert!(10 <= r!(20));
}

#[test]
fn eqz() {
    let some_i32 = 4;
    let some_wrong_i32 = 8;
    assert!(Ranged::<0, 6>::new(some_i32) == Some(r!([] 4)));
    assert!(Ranged::<0, 6>::new(some_wrong_i32).is_none());
}

#[test]
fn convert() {
    assert_eq!(20, i8::from(r!([0 100] 20)));
    assert_eq!(20, u8::from(r!([0 100] 20)));
    assert_eq!(20, i16::from(r!([0 100] 20)));
    assert_eq!(20, u16::from(r!([0 100] 20)));
    assert_eq!(20, i32::from(r!([0 100] 20)));
    assert_eq!(20, u32::from(r!([0 100] 20)));
    assert_eq!(20, i64::from(r!([0 100] 20)));
    assert_eq!(20, u64::from(r!([0 100] 20)));
    assert_eq!(20, i128::from(r!([0 100] 20)));
    assert_eq!(20, isize::from(r!([0 100] 20)));
    assert_eq!(20, usize::from(r!([0 100] 20)));

    let x: Ranged<-128, 127> = 10_i8.as_ranged();
    assert_eq!(r!(10), x);
    let x: Ranged<0, 255> = 10_u8.as_ranged();
    assert_eq!(r!(10), x);
    let x: Ranged<-32768, 32767> = 10_i16.as_ranged();
    assert_eq!(r!(10), x);
    let x: Ranged<0, 65535> = 10_u16.as_ranged();
    assert_eq!(r!(10), x);
    let x: Ranged<-2_147_483_648, 2_147_483_647> = 10_i32.as_ranged();
    assert_eq!(r!(10), x);
    let x: Ranged<0, 4_294_967_295> = 10_u32.as_ranged();
    assert_eq!(r!(10), x);
    let x: Ranged<-9_223_372_036_854_775_808, 9_223_372_036_854_775_807> = 10_i64.as_ranged();
    assert_eq!(r!(10), x);
    let x: Ranged<0, 18_446_744_073_709_551_615> = 10_u64.as_ranged();
    assert_eq!(r!(10), x);
    let x: Ranged<
        -170_141_183_460_469_231_731_687_303_715_884_105_728,
        170_141_183_460_469_231_731_687_303_715_884_105_727,
    > = 10_i128.as_ranged();
    assert_eq!(r!(10), x);
    let x: Ranged<{usize::MIN as i128}, {usize::MAX as i128}> = 10_usize.as_ranged();
    assert_eq!(r!(10), x);
    let x: Ranged<{isize::MIN as i128}, {isize::MAX as i128}> = 10_isize.as_ranged();
    assert_eq!(r!(10), x);
}

#[test]
fn expand() {
    let x = r!([0 100] 20);
    assert_eq!(x, r!(20));

    let y: Ranged<{ -5 }, 200> = x.expand();
    assert_eq!(y, r!(20));

    let z: Ranged<{ -5 }, 200> = y;
    assert_eq!(z, r!(20));

    let x_ex: Ranged<0, 600> = 10_u8.as_ranged().expand();
    assert_eq!(x_ex, r!(10));
}

#[test]
fn minmax() {
    let x = r!([-100 100] 15);
    let y = r!([-20 20] 3);

    let min = x.min(y);
    let max = x.max(y);

    let _: Ranged<-100,20> = min;
    let _: Ranged<-20,100> = max;

    assert_eq!(min, r!(3));
    assert_eq!(max, r!(15));
}

#[test]
fn abs() {
    let x: Ranged<0, 100> = r!([-100 100] 15).abs();
    assert_eq!(x, r!(15));
    let x: Ranged<0, 100> = r!([-100 100] -15).abs();
    assert_eq!(x, r!(15));
    let x: Ranged<50, 100> = r!([50 100] 75).abs();
    assert_eq!(x, r!(75));
    let x: Ranged<50, 100> = r!([-100 -50] -75).abs();
    assert_eq!(x, r!(75));
}


#[test]
fn iter() {
    use core::fmt::Write;

    let mut s = String::new();
    for r in crate::ConstInclusiveRange::<0, 9> {
        write!(&mut s, "{} ", r).unwrap();
    }
    assert_eq!(s, "0 1 2 3 4 5 6 7 8 9 ");

    assert_eq!(r!(0..10).into_iter().step_by(2).collect::<Vec<_>>(), vec![r!([0 8] 0), r!([] 2), r!([] 4), r!([] 6), r!([] 8)]);
    assert_eq!(r!(0..11).into_iter().step_by(2).collect::<Vec<_>>(), vec![r!([0 10] 0), r!([] 2), r!([] 4), r!([] 6), r!([] 8), r!([] 10)]);

    let mut fibonacci = [0; 10];
    fibonacci[0] = 1;
    fibonacci[1] = 1;
    for i in r!(2..10) {
        fibonacci[i.expand()] = fibonacci[(i-r!(1)).expand()] + fibonacci[(i-r!(2)).expand()];
    }
    assert_eq!(fibonacci, [1,1,2,3,5,8,13,21,34,55]);

    let mut fib234: [_; 3] = fibonacci[r!(2..5)];
    assert_eq!(fib234, [2,3,5]);

    fib234[r!(1..3)] = [10,15];
    assert_eq!(fib234, [2,10,15]);

    let rfib = &mut fib234;
    rfib[r!(0..2)] = [0,5];
    assert_eq!(fib234, [0,5,15]);
}


#[test]
fn fromstr() {
    let x = "42".parse::<Ranged<0, 100>>().unwrap();
    assert_eq!(x, 42);

    let x = "333".parse::<Ranged<0, 100>>().ok();
    assert_eq!(x, None);

    let x = "-42".parse::<Ranged<-100, 100>>().unwrap();
    assert_eq!(x, -42);

    let x = "-333".parse::<Ranged<-100, 100>>().ok();
    assert_eq!(x, None);

    let x = r!(16).to_string();
    assert_eq!(x, "16");
}


const fn rmatch_example(val: Ranged<1, 20>) -> &'static str {
    rmatch!{[1 20] val
        1..=5 | 16..=20 => {"Fail"}
        6..=15 => {"Success"}
    }
}

const fn rmatch_digit(val: Ranged<0, 9>) -> &'static str {
    rmatch!{[0 9] val
        0 => {"Zero"}
        1 => {"One"}
        2 => {"Two"}
        3 => {"Three"}
        4 => {"Four"}
        5 => {"Five"}
        6 => {"Six"}
        7 => {"Seven"}
        8 => {"Eight"}
        9 => {"Nine"}
    }
}

#[test]
fn test_rmatch() {
    assert_eq!(rmatch_example(r!([] 5)), "Fail");
    assert_eq!(rmatch_example(r!([] 10)), "Success");
    assert_eq!(rmatch_example(r!([] 15)), "Success");
    assert_eq!(rmatch_example(r!([] 20)), "Fail");

    let all_digits = r!(0..=9).into_iter().map(rmatch_digit).collect::<Vec<_>>().join(" ");
    assert_eq!(all_digits, "Zero One Two Three Four Five Six Seven Eight Nine");
}
