# Ranged integers [nightly only]

Ranged integers for Rust based on const generics.

**Note: WIP, the API may change.**

The crate provides an integer-like type restricted to a compile-time defined range.

[`Ranged<MIN, MAX>`] is bounded to [MIN, MAX] interval **inclusively**.
Parametrized by `i128`.

## Integer size

When MIN and MAX are provided, the
[`Ranged`] automatically chooses the signedness
and the size. It supports i8, u8, i16, u16, i32, u32, i64, u64 and i128 layouts.
The value may be casted to these types.
Due to the fact the `Ranged` is parametrized by `i128`, `u128` layout is not supported.

```rust
use core::mem::size_of;
assert_eq!(size_of::<Ranged::<-100, 127>>(), 1); // The range fits i8
assert_eq!(size_of::<Ranged::<0, 200>>(), 1); // The range fits u8
assert_eq!(size_of::<Ranged::<-100, 200>>(), 2); // The range fits i16
assert_eq!(size_of::<Ranged::<0, 90000>>(), 4); // i32 is needed
```

The implementation heavily relies on the optimizer.

# Usage and examples

## Prerequisites

The library's macro [`ranged!`] requires the following Rust features enabled:

```rust
#![feature(const_if_match)]
#![feature(const_panic)]
```

## Ranged semantics

Use `Ranged<MIN, MAX>` as a function argument to ensure the parameter range:

```rust
fn move_player(dice_roll: Ranged<1, 6>) {
    let x : i32 = dice_roll.into(); // Convert to int
}
```

## Create Ranged at compile time

The macro `ranged!([MIN MAX] VALUE)` creates the const Ranged:

```rust
move_player(ranged!([1 6] 4));
```

It fails if the bounds are corrupted:

```rust
move_player(ranged!([1 6] 7)); // Error: Can't store 7 in [1 6] inverval
move_player(ranged!([1 7] 7)); // Error: type mismatch, move_player() requires Ranged<1, 6>
```

A special case with the single possible value:

```rust
let x = ranged![4]; // Means Ranged<4, 4> with the value 4
let y: Ranged<4,4> = x;
```

## Cast to Ranged at runtime

### Way 1: Ensure the bounds with `new` method:

```rust
let some_i32 = 4;
let some_wrong_i32 = 8;
assert!(Ranged::<0, 6>::new(some_i32).unwrap() == ranged![4]);
assert!(Ranged::<0, 6>::new(some_wrong_i32) == None);
```

The user must always specify the bounds with "turbofish" operator when
uses `new()` method (doesn't compile otherwise). This is related to a
compile-time check for MIN<MAX.
This is to be fixed when possible.

```rust
let a : Ranged::<0, 6> = Ranged::<0, 6>::new(1).unwrap();  // Ok
```

```rust
let a : Ranged::<0, 6> = Ranged::new(1).unwrap();  // Currently fails
```

### Way 2: use remainder operation with the const divisor as a way to create Ranged:

```rust
let x = 15 % ranged![10];
let y: Ranged<0, 9> = x;
assert!(y == ranged![5]); // 15 % 10 == 5
```

```rust
let x = 15 % ranged![10];
let y: Ranged<0, 20> = x;  // Error: x is Ranged<0, 9>
```

### Way 3: Convert the primitive types to `Ranged` with their native bounds:

```rust
let x = 15_u8.as_ranged(); // Ranged<0, 255>
                           // Trait AsRanged must be in scope
assert!(x == ranged![15]);
```

## Expand Ranged bounds

Use `Expand` helper to widen the Ranged bounds:

```rust
let x = ranged!([0 100] 20);
let y = Expand::<0,100,-5,200>(x).expand(); // From [0 100] to [-5 200]
let check: Ranged<-5, 200> = y;
```

Shrinking is not allowed:

```rust
let x = ranged!([0 100] 20);
let y = Expand::<0,100,5,200>(x).expand(); // Error: must contain [0 4]
```

## Cast from Ranged

Casting to integer types is allowed when the value is proven to
fit in the result type:

```rust
let x = ranged!([0 200] 20);
assert_eq!(20_u8, x.into()); // Impossible in const fns
assert_eq!(20_u8, x.u8());   // Possible in const fns
```

```rust
let x = ranged!([0 200] 20);
assert_eq!(20_i8, x.into()); // Error: can't fit the range 128..=200 in i8
```

## MIN and MAX checks

It's unluckily possible to create something like `Ranged<1, 0>` type where
min and max are jumbled. However, it's impossible to use the documented creation
and int-conversion features with such types.

```rust
type T = Ranged<1, 0>; // Works
```

```rust
let x = Ranged::<1, 0>::new(1); // Compile error: MAX<MIN is weird
```

```rust
let x = ranged!([1 0] 1); // Compile error
```

## Comparison

Comparison between different types is allowed:

```rust
assert!(ranged!([1 6] 4) == ranged!([1 10] 4));
assert!(ranged!([1 6] 4) != ranged!([1 6] 5));
```

## Arithmetics

Currently addition, subtraction, multiplication and division are possible.
The bounds of values are automatically recalculated:

```rust
let x = ranged!([0 6] 4);
let y: Ranged<0, 10> = x*x; // Error: wrong type, must be Ranged<0, 36>
```

### Addition

```rust
let a = ranged!([1 6] 4);
let b = ranged!([1 6] 5);
let c = a + b;     // Impossible in const fns
let d = a.add(b);  // Possible in const fns
let check: Ranged<2, 12> = c;  // 2 = 1+1, 12 = 6+6
```

### Subtraction

```rust
let a = ranged!([1 6] 4);
let b = ranged!([1 6] 5);
let c = a - b;     // Impossible in const fns
let d = a.sub(b);  // Possible in const fns
let check: Ranged<-5, 5> = c;  // -5 = 1-6, 5 = 6-1
```

### Multiplication

```rust
let a = ranged!([1 6] 4);
let b = ranged!([1 6] 5);
let c = a * b;     // Impossible in const fns
let d = a.mul(b);  // Possible in const fns
let check: Ranged<1, 36> = c;  // 1 = 1*1, 36 = 6*6
```

### Division

Allowed if the range of second operand doesn't include 0.
The syntax is a bit non-trivial.

```rust
let a = ranged!([1 6] 4);
let b = ranged!([1 6] 5);
let c = RDiv(a, b).div(); // Possible in const fns
let check: Ranged<0, 6> = c;  // 0 = 1/6, 6 = 6/1
```

```rust
let a = ranged!([1 6] 4);
let b = ranged!([-1 6] 5);
let x = RDiv(a, b).div(); // Disallowed, the second operand may be 0
```
