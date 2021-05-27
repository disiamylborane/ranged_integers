# Ranged integers [nightly only]

**Note: WIP, the API may change.**

The crate provides an integer type restricted to a compile time defined range.
Auto size and compile time checked arithmetics are included.

`Ranged<const MIN: i128, const MAX: i128>` is bounded to [MIN, MAX] interval **inclusively**.

# Usage and examples

## Prerequisites

The library's macro `r!` requires the following Rust features enabled:

```rust
// Without this the Ranged usage now fails with some unclear 
// "trait bound is not satisfied" errors:
#![feature(const_generics)] 
#![feature(const_evaluatable_checked)]

// This is needed for r! macro usage:
#![feature(const_panic)]
```

## Ranged semantics

Use `Ranged<MIN, MAX>` type to be sure of the parameter range:

```rust
fn move_player(dice_roll: Ranged<1, 6>) {
    let x : i32 = dice_roll.into(); // Convert to int
}
```

## Compile time Ranged creation

The macro `r!([MIN MAX] VALUE)` creates the const Ranged:

```rust
move_player(r!([1 6] 4));
```

It fails if the bounds are corrupted:

```rust
move_player(r!([1 6] 7)); // Error: Can't store 7 in [1 6] inverval
move_player(r!([1 7] 7)); // Error: type mismatch, move_player() requires Ranged<1, 6>
```

A special case with the single possible value:

```rust
let x = r![4]; // Means Ranged<4, 4> with the value 4
let y: Ranged<4,4> = x;
```

## Casting to Ranged at runtime

### Way 1: ensure the bounds with `new(int)->Option` method

```rust
let some_i32 = 4;
let some_wrong_i32 = 8;
assert!(Ranged::<0, 6>::new(some_i32).unwrap() == r![4]);
assert!(Ranged::<0, 6>::new(some_wrong_i32) == None);
//!
move_player(Ranged::new(4).unwrap());
```

### Way 2: use the remainder operation with the "const" divisor

```rust
let x = 15 % r![10];
let y: Ranged<0, 9> = x;
assert!(y == r![5]); // 15 % 10 == 5
```

```rust
let x = 15 % r![10];
let y: Ranged<0, 20> = x;  // Error: x is Ranged<0, 9>
```

### Way 3: Convert the primitive types to `Ranged` with their native bounds

```rust
let x = 15_u8.as_ranged(); // Ranged<0, 255>
                           // Trait AsRanged must be in scope
assert!(x == r![15]);
```

## Bounds expansion

Expand the bounds freely if needed:

```rust
let x = r!([0 100] 20);
let y : Ranged<-5, 200> = x.expand(); // From [0 100] to [-5 200]
let z = x.expand::<-5, 200>(); // Also [0 100] -> [-5 200]
```

Shrinking is not allowed:

```rust
let x = r!([0 100] 20);
let y : Ranged<1, 200> = x.expand(); // Error: x can be 0
```

## Cast Ranged to primitives

Casting to integer types is allowed when the value is proven to
fit into the result type:

```rust
let x = r!([0 200] 20);
assert_eq!(20_u8, x.into());
```

```rust
let x = r!([0 200] 20);
assert_eq!(20_i8, x.into()); // Error: can't fit the range 128..=200 in i8
```

There is also a set of const functions for Ranged to primitive casting:

```rust
let x = r!([0 200] 20);
let y = x.u8(); // y is u8
let z = x.i16(); // z is i16
```

## Comparison

Comparison between different Ranged types is allowed:

```rust
assert!(r!([1 6] 4) == r!([1 10] 4));
assert!(r!([1 6] 4) != r!([1 6] 5));
```

## Arithmetics

Currently addition, subtraction, multiplication and division operations are implemented.
The bounds of values are automatically recalculated:

```rust
let x = r!([1 6] 4);
let y = r!([1 6] 5);

let a = x + y;
let check_add: Ranged<2, 12> = a;

let s = x - y;
let check_sub: Ranged<-5, 5> = s;

let m = x * y;
let check_mul: Ranged<1, 36> = m;

let d = x / y;
let check_div: Ranged<0, 6> = d;
```

The division is allowed only if it's impossible to store "0" in the divisor:

```rust
let x = r!([1 6] 4);
let y = r!([0 6] 5);
let z = r!([-1 6] 5);

let d = x / y; // Error: division is not possible
let e = x / z; // Error: division is not possible
```

## Integer size

When MIN and MAX are provided, the
`Ranged` automatically chooses the signedness
and the size. It supports i8, u8, i16, u16, i32, u32, i64, u64 and i128 layouts (u128 is omitted).

```rust
use core::mem::size_of;
assert_eq!(size_of::<Ranged::<-100, 127>>(), 1); // The range fits i8
assert_eq!(size_of::<Ranged::<0, 200>>(), 1); // The range fits u8
assert_eq!(size_of::<Ranged::<-100, 200>>(), 2); // The range fits i16
assert_eq!(size_of::<Ranged::<0, 90000>>(), 4); // The range fits i32
```

The implementation heavily relies on the optimizer.
