// Ranged<(-?[0-9]+), (-?[0-9]+)> \{ _val: (-?[0-9]+) \}
// r![[$1 $2] $3]

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

    sz_align!(i8, Ranged<0,0>);
    sz_align!(i8, Ranged<10,10>);
    sz_align!(i8, Ranged<255,255>);
    sz_align!(i8, Ranged<127,127>);
    sz_align!(i8, Ranged<-128, -128>);
    sz_align!(i8, Ranged<0,10>);
    sz_align!(i8, Ranged<0,127>);
    sz_align!(i8, Ranged<0,255>);
    sz_align!(i8, Ranged<127,255>);
    sz_align!(i8, Ranged<-128, 127>);

    sz_align!(i16, Ranged<-128, 128>);

    sz_align!(i16, Ranged<-32768, 32767>);
    sz_align!(i16, Ranged<0, 32768>);
    sz_align!(i16, Ranged<0, 65535>);
    sz_align!(i16, Ranged<-32768, -32768>);
    sz_align!(i16, Ranged<32767, 32767>);
    sz_align!(i16, Ranged<65535, 65535>);

    sz_align!(i32, Ranged<-32768, 32768>);
    sz_align!(i32, Ranged<0, 65536>);
    sz_align!(i32, Ranged<65536, 65536>);

    sz_align!(i32, Ranged<0, 4294967295>);
    sz_align!(i32, Ranged<-2147483648, 2147483647>);
    sz_align!(i32, Ranged<100, 10000000>);
    sz_align!(i32, Ranged<-100, 10000000>);
    sz_align!(i32, Ranged<100, 2147483647>);
    sz_align!(i32, Ranged<-100, 2147483647>);

    sz_align!(i64, Ranged<-1, 4294967295>);
    sz_align!(i64, Ranged<0, 4294967296>);
    sz_align!(i64, Ranged<-2147483649, 2147483647>);
    sz_align!(i64, Ranged<-2147483648, 2147483648>);

    sz_align!(i64, Ranged<0, 18446744073709551615>);
    sz_align!(i64, Ranged<-9223372036854775808, 9223372036854775807>);

    sz_align!(i128, Ranged<-1, 18446744073709551615>);

    sz_align!(i128, Ranged<0, 18446744073709551616>);
    sz_align!(i128, Ranged<-9223372036854775809, 9223372036854775807>);
    sz_align!(i128, Ranged<-9223372036854775808, 9223372036854775808>);
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

    let x = r!(400000);
    assert_eq!(format!("{}", x), "400000");

    let x = r!(4000);
    assert_eq!(format!("{}", x), "4000");

    let x = r!(40);
    assert_eq!(format!("{}", x), "40");

    let x = r!(0);
    assert_eq!(format!("{}", x), "0");

    let x = r!(-400000);
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

    let a = r!([0 100] 20) / r![[-10 - 1] - 5];
    assert_eq!(format!("{}", a), "-4");
    assert_eq!(format!("{:?}", a), "r!([-100 0] -4)");

    let a = r!([-100 0] -20) / r![[-10 - 1] - 5];
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
}

#[test]
fn rem() {
    let a = r!(25) % r!(20);
    assert_eq!(a, r!(5));

    let a: Ranged<0, 19> = r!(25) % r!([1 20] 13);
    assert_eq!(a, r!(12));

    let a: Ranged<-19, 0> = r!(-25) % r!([1 20] 13);
    assert_eq!(a, r!(-12));
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
}

#[test]
fn eqz() {
    let some_i32 = 4;
    let some_wrong_i32 = 8;
    assert!(Ranged::<0, 6>::new(some_i32).unwrap() == r!(4));
    assert!(Ranged::<0, 6>::new(some_wrong_i32) == None);
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

    let x: Ranged<-128, 127> = 10_i8.as_ranged();
    assert_eq!(r!(10), x);
    let x: Ranged<0, 255> = 10_u8.as_ranged();
    assert_eq!(r!(10), x);
    let x: Ranged<-32768, 32767> = 10_i16.as_ranged();
    assert_eq!(r!(10), x);
    let x: Ranged<0, 65535> = 10_u16.as_ranged();
    assert_eq!(r!(10), x);
    let x: Ranged<-2147483648, 2147483647> = 10_i32.as_ranged();
    assert_eq!(r!(10), x);
    let x: Ranged<0, 4294967295> = 10_u32.as_ranged();
    assert_eq!(r!(10), x);
    let x: Ranged<-9223372036854775808, 9223372036854775807> = 10_i64.as_ranged();
    assert_eq!(r!(10), x);
    let x: Ranged<0, 18446744073709551615> = 10_u64.as_ranged();
    assert_eq!(r!(10), x);
    let x: Ranged<
        -170141183460469231731687303715884105728,
        170141183460469231731687303715884105727,
    > = 10_i128.as_ranged();
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
