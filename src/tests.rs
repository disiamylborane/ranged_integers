

use super::*;
use alloc::format;

#[test]
fn sizes() {
    use core::mem::{size_of, align_of};

    macro_rules! sz_align {
        ($sz:literal $t:ty) => {
            assert_eq!(size_of::<$t>(), $sz);
            assert_eq!(align_of::<$t>(), $sz);
        }
    };

    sz_align!(1 Ranged::<0,0>);
    sz_align!(1 Ranged::<10,10>);
    sz_align!(1 Ranged::<255,255>);
    sz_align!(1 Ranged::<127,127>);
    sz_align!(1 Ranged::<-128,-128>);
    sz_align!(1 Ranged::<0,10>);
    sz_align!(1 Ranged::<0,127>);
    sz_align!(1 Ranged::<0,255>);
    sz_align!(1 Ranged::<127,255>);
    sz_align!(1 Ranged::<-128,127>);

    sz_align!(2 Ranged::<-128,128>);

    sz_align!(2 Ranged::<-32768, 32767>);
    sz_align!(2 Ranged::<0, 32768>);
    sz_align!(2 Ranged::<0, 65535>);
    sz_align!(2 Ranged::<-32768, -32768>);
    sz_align!(2 Ranged::<32767, 32767>);
    sz_align!(2 Ranged::<65535, 65535>);

    sz_align!(4 Ranged::<-32768, 32768>);
    sz_align!(4 Ranged::<0, 65536>);
    sz_align!(4 Ranged::<65536, 65536>);
}

#[test]
fn print_val() {
    macro_rules! assert_val {
        ([$min:literal $max:literal] $x:literal) => {
            assert_eq!(ranged![[$min $max] $x].get(), $x);
        }
    };

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

    let x = const_val_i32::<42>();
    assert_eq!(format!("{}", x), "42");
    assert_eq!(format!("{:?}", x), "Ranged<42, 42> { _val: 42 }");

    let x = const_val_i32::<400000>();
    assert_eq!(format!("{}", x), "400000");

    let x = const_val_i32::<4000>();
    assert_eq!(format!("{}", x), "4000");

    let x = const_val_i32::<40>();
    assert_eq!(format!("{}", x), "40");

    let x = const_val_i32::<0>();
    assert_eq!(format!("{}", x), "0");

    let x = const_val_i32::<{-400000}>();
    assert_eq!(format!("{}", x), "-400000");

    let x = const_val_i32::<{-4000}>();
    assert_eq!(format!("{}", x), "-4000");

    let x = const_val_i32::<{-40}>();
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
}

#[test]
fn addsub() {
    let a = ranged![20] + ranged![22];
    assert_eq!(format!("{}", a), "42");
    assert_eq!(format!("{:?}", a), "Ranged<42, 42> { _val: 42 }");

    let a = 15 % ranged![20];
    let b = a + a;
    assert_eq!(format!("{}", b), "30");
    assert_eq!(format!("{:?}", b), "Ranged<0, 38> { _val: 30 }");

    let c = ranged![22] - ranged![20];
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
