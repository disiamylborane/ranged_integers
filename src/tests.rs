

use super::*;
use alloc::format;

#[test]
fn sizes() {
    assert_eq!(core::mem::size_of::<Ranged::<42,42>>(), 1);
    assert_eq!(core::mem::size_of::<Ranged::<42,342>>(), 2);
    assert_eq!(core::mem::size_of::<Ranged::<42,77742>>(), 4);

    assert_eq!(core::mem::size_of::<Ranged::<1, 10>>(), 1);
    assert_eq!(core::mem::size_of::<Ranged::<-10, -1>>(), 1);
    assert_eq!(core::mem::size_of::<Ranged::<-10, 10>>(), 1);

    assert_eq!(core::mem::size_of::<Ranged::<1, 127>>(), 1);
    assert_eq!(core::mem::size_of::<Ranged::<1, 128>>(), 2);
    assert_eq!(core::mem::size_of::<Ranged::<-128, 127>>(), 1);
    assert_eq!(core::mem::size_of::<Ranged::<-129, 127>>(), 2);
    assert_eq!(core::mem::size_of::<Ranged::<-129, 128>>(), 2);

    assert_eq!(core::mem::size_of::<Ranged::<1, 32767>>(), 2);
    assert_eq!(core::mem::size_of::<Ranged::<1, 32768>>(), 4);
    assert_eq!(core::mem::size_of::<Ranged::<-32768, 32767>>(), 2);
    assert_eq!(core::mem::size_of::<Ranged::<-32768, 32768>>(), 4);
    assert_eq!(core::mem::size_of::<Ranged::<-32769, -10>>(), 4);
    assert_eq!(core::mem::size_of::<Ranged::<10, 32768>>(), 4);

    assert_eq!(core::mem::align_of::<Ranged::<42,42>>(), 1);
    assert_eq!(core::mem::align_of::<Ranged::<42,342>>(), 2);
    assert_eq!(core::mem::align_of::<Ranged::<42,77742>>(), 4);

    assert_eq!(core::mem::align_of::<Ranged::<1, 10>>(), 1);
    assert_eq!(core::mem::align_of::<Ranged::<-10, -1>>(), 1);
    assert_eq!(core::mem::align_of::<Ranged::<-10, 10>>(), 1);

    assert_eq!(core::mem::align_of::<Ranged::<1, 127>>(), 1);
    assert_eq!(core::mem::align_of::<Ranged::<1, 128>>(), 2);
    assert_eq!(core::mem::align_of::<Ranged::<-128, 127>>(), 1);
    assert_eq!(core::mem::align_of::<Ranged::<-129, 127>>(), 2);
    assert_eq!(core::mem::align_of::<Ranged::<-129, 128>>(), 2);

    assert_eq!(core::mem::align_of::<Ranged::<1, 32767>>(), 2);
    assert_eq!(core::mem::align_of::<Ranged::<1, 32768>>(), 4);
    assert_eq!(core::mem::align_of::<Ranged::<-32768, 32767>>(), 2);
    assert_eq!(core::mem::align_of::<Ranged::<-32768, 32768>>(), 4);
    assert_eq!(core::mem::align_of::<Ranged::<-32769, -10>>(), 4);
    assert_eq!(core::mem::align_of::<Ranged::<10, 32768>>(), 4);
}

#[test]
fn print_val() {
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
