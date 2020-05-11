

use super::*;
use alloc::format;


#[test]
fn sizes() {
    use core::mem::{size_of, align_of};

    macro_rules! sz_align {
        ($sz:ty, $t:ty) => {
            assert_eq!(size_of::<$t>(), size_of::<$sz>());
            assert_eq!(align_of::<$t>(), align_of::<$sz>());
        }
    };

    sz_align!(i8, Ranged::<0,0>);
    sz_align!(i8, Ranged::<10,10>);
    sz_align!(i8, Ranged::<255,255>);
    sz_align!(i8, Ranged::<127,127>);
    sz_align!(i8, Ranged::<-128,-128>);
    sz_align!(i8, Ranged::<0,10>);
    sz_align!(i8, Ranged::<0,127>);
    sz_align!(i8, Ranged::<0,255>);
    sz_align!(i8, Ranged::<127,255>);
    sz_align!(i8, Ranged::<-128,127>);

    sz_align!(i16, Ranged::<-128,128>);

    sz_align!(i16, Ranged::<-32768, 32767>);
    sz_align!(i16, Ranged::<0, 32768>);
    sz_align!(i16, Ranged::<0, 65535>);
    sz_align!(i16, Ranged::<-32768, -32768>);
    sz_align!(i16, Ranged::<32767, 32767>);
    sz_align!(i16, Ranged::<65535, 65535>);

    sz_align!(i32, Ranged::<-32768, 32768>);
    sz_align!(i32, Ranged::<0, 65536>);
    sz_align!(i32, Ranged::<65536, 65536>);

    sz_align!(i32, Ranged::<0, 4294967295>);
    sz_align!(i32, Ranged::<-2147483648, 2147483647>);
    sz_align!(i32, Ranged::<100, 10000000>);
    sz_align!(i32, Ranged::<-100, 10000000>);
    sz_align!(i32, Ranged::<100, 2147483647>);
    sz_align!(i32, Ranged::<-100, 2147483647>);

    sz_align!(i64, Ranged::<-1, 4294967295>);
    sz_align!(i64, Ranged::<0, 4294967296>);
    sz_align!(i64, Ranged::<-2147483649, 2147483647>);
    sz_align!(i64, Ranged::<-2147483648, 2147483648>);

    sz_align!(i64, Ranged::<0, 18446744073709551615>);
    sz_align!(i64, Ranged::<-9223372036854775808, 9223372036854775807>);

    sz_align!(i128, Ranged::<-1, 18446744073709551615>);

    sz_align!(i128, Ranged::<0, 18446744073709551616>);
    sz_align!(i128, Ranged::<-9223372036854775809, 9223372036854775807>);
    sz_align!(i128, Ranged::<-9223372036854775808, 9223372036854775808>);
}

#[test]
fn print_val() {
    macro_rules! assert_val {
        ([$min:literal $max:literal] $x:literal) => {
            assert_eq!(ranged![[$min $max] $x].get(), $x);
        }
    };

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

    let x = ranged![42];
    assert_eq!(format!("{}", x), "42");
    assert_eq!(format!("{:?}", x), "Ranged<42, 42> { _val: 42 }");

    let x = ranged![400000];
    assert_eq!(format!("{}", x), "400000");

    let x = ranged![4000];
    assert_eq!(format!("{}", x), "4000");

    let x = ranged![40];
    assert_eq!(format!("{}", x), "40");

    let x = ranged![0];
    assert_eq!(format!("{}", x), "0");

    let x = ranged![-400000];
    assert_eq!(format!("{}", x), "-400000");

    let x = ranged![-4000];
    assert_eq!(format!("{}", x), "-4000");

    let x = ranged![-40];
    assert_eq!(format!("{}", x), "-40");
}

#[test]
fn ranged_macro() {
    let x = ranged!{[0 4] 2};
    assert_eq!(format!("{}", x), "2");
    assert_eq!(format!("{:?}", x), "Ranged<0, 4> { _val: 2 }");

    let x = ranged!{10};
    assert_eq!(format!("{}", x), "10");
    assert_eq!(format!("{:?}", x), "Ranged<10, 10> { _val: 10 }");
}

#[test]
fn remdr() {
    let y = 64 % ranged![42];
    assert_eq!(format!("{}", y), "22");
    assert_eq!(format!("{:?}", y), "Ranged<0, 41> { _val: 22 }");

    let y = 64i8 % ranged![42];
    assert_eq!(format!("{}", y), "22");
    assert_eq!(format!("{:?}", y), "Ranged<0, 41> { _val: 22 }");

    let y = 64u8 % ranged![42];
    assert_eq!(format!("{}", y), "22");
    assert_eq!(format!("{:?}", y), "Ranged<0, 41> { _val: 22 }");

    let y = 64i16 % ranged![42];
    assert_eq!(format!("{}", y), "22");
    assert_eq!(format!("{:?}", y), "Ranged<0, 41> { _val: 22 }");

    let y = 64u16 % ranged![42];
    assert_eq!(format!("{}", y), "22");
    assert_eq!(format!("{:?}", y), "Ranged<0, 41> { _val: 22 }");

    let y = 64i32 % ranged![42];
    assert_eq!(format!("{}", y), "22");
    assert_eq!(format!("{:?}", y), "Ranged<0, 41> { _val: 22 }");

    let y = 64u32 % ranged![42];
    assert_eq!(format!("{}", y), "22");
    assert_eq!(format!("{:?}", y), "Ranged<0, 41> { _val: 22 }");

    let y = 64i64 % ranged![42];
    assert_eq!(format!("{}", y), "22");
    assert_eq!(format!("{:?}", y), "Ranged<0, 41> { _val: 22 }");

    let y = 64u64 % ranged![42];
    assert_eq!(format!("{}", y), "22");
    assert_eq!(format!("{:?}", y), "Ranged<0, 41> { _val: 22 }");

    let y = 64i128 % ranged![42];
    assert_eq!(format!("{}", y), "22");
    assert_eq!(format!("{:?}", y), "Ranged<0, 41> { _val: 22 }");
}

#[test]
fn addsub() {
    let a = ranged![20] + ranged![22];
    assert_eq!(format!("{}", a), "42");
    assert_eq!(format!("{:?}", a), "Ranged<42, 42> { _val: 42 }");

    let a = ranged![20] .add (ranged![22]);
    assert_eq!(format!("{}", a), "42");
    assert_eq!(format!("{:?}", a), "Ranged<42, 42> { _val: 42 }");

    let a = 15 % ranged![20];
    let b = a + a;
    assert_eq!(format!("{}", b), "30");
    assert_eq!(format!("{:?}", b), "Ranged<0, 38> { _val: 30 }");

    let c = ranged![22] - ranged![20];
    assert_eq!(format!("{}", c), "2");
    assert_eq!(format!("{:?}", c), "Ranged<2, 2> { _val: 2 }");

    let c = ranged![22] .sub (ranged![20]);
    assert_eq!(format!("{}", c), "2");
    assert_eq!(format!("{:?}", c), "Ranged<2, 2> { _val: 2 }");
}


#[test]
fn mul() {
    let a = ranged![20] * ranged![20];
    assert_eq!(format!("{}", a), "400");
    assert_eq!(format!("{:?}", a), "Ranged<400, 400> { _val: 400 }");

    let b = ranged!{[-3 3] 1} * ranged!{[-3 3] 2};
    assert_eq!(format!("{}", b), "2");
    assert_eq!(format!("{:?}", b), "Ranged<-9, 9> { _val: 2 }");

    let c = Ranged::<{-3}, 0>::new(-1).unwrap() * Ranged::<0, 3>::new(2).unwrap();
    assert_eq!(format!("{}", c), "-2");
    assert_eq!(format!("{:?}", c), "Ranged<-9, 0> { _val: -2 }");

    let b = ranged!{[-30000 30000] 1} * ranged!{[-3 3] 2};
    assert_eq!(format!("{}", b), "2");
    assert_eq!(format!("{:?}", b), "Ranged<-90000, 90000> { _val: 2 }");

    let a = ranged![20] .mul (ranged![20]);
    assert_eq!(format!("{}", a), "400");
    assert_eq!(format!("{:?}", a), "Ranged<400, 400> { _val: 400 }");

    let b = ranged!{[-3 3] 1} .mul (ranged!{[-3 3] 2});
    assert_eq!(format!("{}", b), "2");
    assert_eq!(format!("{:?}", b), "Ranged<-9, 9> { _val: 2 }");

    let c = Ranged::<{-3}, 0>::new(-1).unwrap() .mul (Ranged::<0, 3>::new(2).unwrap());
    assert_eq!(format!("{}", c), "-2");
    assert_eq!(format!("{:?}", c), "Ranged<-9, 0> { _val: -2 }");

    let b = ranged!{[-30000 30000] 1} .mul (ranged!{[-3 3] 2});
    assert_eq!(format!("{}", b), "2");
    assert_eq!(format!("{:?}", b), "Ranged<-90000, 90000> { _val: 2 }");
}

#[test]
fn div() {
    let a = RDiv(ranged![20], ranged![20]).div();
    assert_eq!(format!("{}", a), "1");
    assert_eq!(format!("{:?}", a), "Ranged<1, 1> { _val: 1 }");

    let a = RDiv(ranged![[0 100] 20], ranged![[1 10] 5]).div();
    assert_eq!(format!("{}", a), "4");
    assert_eq!(format!("{:?}", a), "Ranged<0, 100> { _val: 4 }");

    let a = RDiv(ranged![[0 100] 20], ranged![[-10 -1] -5]).div();
    assert_eq!(format!("{}", a), "-4");
    assert_eq!(format!("{:?}", a), "Ranged<-100, 0> { _val: -4 }");

    let a = RDiv(ranged![[-100 0] -20], ranged![[-10 -1] -5]).div();
    assert_eq!(format!("{}", a), "4");
    assert_eq!(format!("{:?}", a), "Ranged<0, 100> { _val: 4 }");

    let a = RDiv(ranged![[-100 0] -20], ranged![[1 10] 5]).div();
    assert_eq!(format!("{}", a), "-4");
    assert_eq!(format!("{:?}", a), "Ranged<-100, 0> { _val: -4 }");

    let a = RDiv(ranged![[100 1000] 500], ranged![[1 6] 5]).div();
    assert_eq!(format!("{}", a), "100");
    assert_eq!(format!("{:?}", a), "Ranged<16, 1000> { _val: 100 }");

    let a = RDiv(ranged![[100 1000] 500], ranged![[-6 -1] -5]).div();
    assert_eq!(format!("{}", a), "-100");
    assert_eq!(format!("{:?}", a), "Ranged<-1000, -16> { _val: -100 }");
}

#[test]
fn eq() {
    let a = ranged![20];
    let b = ranged![40];
    assert_eq!(a, a);
    assert_eq!(15%a, 15%b);
    assert!(10%a + 15%b == 15%a + 10%b);
    assert!(11%a + 15%b != 15%a + 10%b);
    assert!(a != b);
}

#[test]
fn convert() {
    assert_eq!(20, i8::from(ranged!([0 100] 20)));
    assert_eq!(20, u8::from(ranged!([0 100] 20)));
    assert_eq!(20, i16::from(ranged!([0 100] 20)));
    assert_eq!(20, u16::from(ranged!([0 100] 20)));
    assert_eq!(20, i32::from(ranged!([0 100] 20)));
    assert_eq!(20, u32::from(ranged!([0 100] 20)));
    assert_eq!(20, i64::from(ranged!([0 100] 20)));
    assert_eq!(20, u64::from(ranged!([0 100] 20)));
    assert_eq!(20, i128::from(ranged!([0 100] 20)));

    let x: Ranged<{-128},127> = 10_i8.as_ranged();
    assert_eq!(ranged![10], x);
    let x: Ranged<0,255> = 10_u8.as_ranged();
    assert_eq!(ranged![10], x);
    let x: Ranged<{-32768},32767> = 10_i16.as_ranged();
    assert_eq!(ranged![10], x);
    let x: Ranged<0,65535> = 10_u16.as_ranged();
    assert_eq!(ranged![10], x);
    let x: Ranged<{-2147483648},2147483647> = 10_i32.as_ranged();
    assert_eq!(ranged![10], x);
    let x: Ranged<0,4294967295> = 10_u32.as_ranged();
    assert_eq!(ranged![10], x);
    let x: Ranged<{-9223372036854775808}, 9223372036854775807> = 10_i64.as_ranged();
    assert_eq!(ranged![10], x);
    let x: Ranged<0,18446744073709551615> = 10_u64.as_ranged();
    assert_eq!(ranged![10], x);
    let x: Ranged<{-170141183460469231731687303715884105728}, 170141183460469231731687303715884105727> = 10_i128.as_ranged();
    assert_eq!(ranged![10], x);

    /*
    let x: Ranged<{-128},127> = 10_i8.into();
    assert_eq!(ranged![10], x);
    let x: Ranged<0,255> = 10_u8.into();
    assert_eq!(ranged![10], x);
    let x: Ranged<{-32768},32767> = 10_i16.into();
    assert_eq!(ranged![10], x);
    let x: Ranged<0,65535> = 10_u16.into();
    assert_eq!(ranged![10], x);
    let x: Ranged<{-2147483648},2147483647> = 10_i32.into();
    assert_eq!(ranged![10], x);
    let x: Ranged<0,4294967295> = 10_u32.into();
    assert_eq!(ranged![10], x);
    let x: Ranged<{-9223372036854775808}, 9223372036854775807> = 10_i64.into();
    assert_eq!(ranged![10], x);
    let x: Ranged<0,18446744073709551615> = 10_u64.into();
    assert_eq!(ranged![10], x);
    let x: Ranged<{-170141183460469231731687303715884105728}, 170141183460469231731687303715884105727> = 10_i128.into();
    assert_eq!(ranged![10], x);

    assert_eq!(format!("{:?}", Ranged::from(10_i8)), "Ranged<-128, 127> { _val: 10 }");
    assert_eq!(format!("{:?}", Ranged::from(10_i16)), "Ranged<-32768, 32767> { _val: 10 }");
    assert_eq!(format!("{:?}", Ranged::from(10_i32)), "Ranged<-2147483648, 2147483647> { _val: 10 }");
    assert_eq!(format!("{:?}", Ranged::from(10_i64)), "Ranged<-9223372036854775808, 9223372036854775807> { _val: 10 }");
    assert_eq!(format!("{:?}", Ranged::from(10_i128)), "Ranged<-170141183460469231731687303715884105728, 170141183460469231731687303715884105727> { _val: 10 }");
    assert_eq!(format!("{:?}", Ranged::from(10_u8)), "Ranged<0, 255> { _val: 10 }");
    assert_eq!(format!("{:?}", Ranged::from(10_u16)), "Ranged<0, 65535> { _val: 10 }");
    assert_eq!(format!("{:?}", Ranged::from(10_u32)), "Ranged<0, 4294967295> { _val: 10 }");
    assert_eq!(format!("{:?}", Ranged::from(10_u64)), "Ranged<0, 18446744073709551615> { _val: 10 }");
    */
}


#[test]
fn expand() {
    let x = ranged!([0 100] 20);
    assert!(x == ranged!(20));

    let y = Expand::<0,100,{-5},200>(x).expand();
    assert!(y == ranged!(20));

    let z: Ranged<{-5}, 200> = y;
    assert!(z == ranged!(20));
}

