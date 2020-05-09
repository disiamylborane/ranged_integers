# Ranged integers

Ranged integers for Rust based on const generics.

`Ranged<MIN, MAX>` is an integer-like type that ranges from MIN to MAX **inclusively**.

## Integer size

`Ranged` stores a signed value. The type size is automatically adjusted
according to the bounds (maximum 32 bits):

```rust
use core::mem::size_of;

assert_eq!(size_of::<Ranged::<100, 127>>(), 1); // Only i8 is needed to store the value
assert_eq!(align_of::<Ranged::<100, 127>>(), 1);
assert_eq!(size_of::<Ranged::<100, 128>>(), 2); // Need 16 bits to store +128
assert_eq!(align_of::<Ranged::<100, 128>>(), 2);
assert_eq!(size_of::<Ranged::<0, 90000>>(), 4); // 4 bytes needed
assert_eq!(align_of::<Ranged::<0, 90000>>(), 4);
```

## Examples

The library's macro `ranged!` requires the following features:

```rust
#![feature(const_if_match)]
#![feature(const_panic)]
```

Use `Ranged<MIN, MAX>` as an argument to make the parameter's value compile-time checked:

```rust
fn move_player(dice_roll: Ranged<1, 6>) {
    let x : i32 = dice_roll.get(); // Convert back to int
}
```

Create the value at compile-time:

```rust
move_player(ranged!([1 6] 4));
```

It fails if the bounds are corrupted:

```rust
move_player(ranged!([1 6] 7)); // Can't store 7 in [1 6] inverval
move_player(ranged!([1 7] 7)); // Mismatched types, move_player() requires Ranged<1, 6>
```

A special case with single possible value:

```rust
let x = ranged![4]; // Means Ranged<4, 4> with the value 4
let y: Ranged<4,4> = x;
```

Comparison between different types is allowed:

```rust
assert!(ranged!([1 6] 4) == ranged!([1 10] 4));
assert!(ranged!([1 6] 4) != ranged!([1 6] 5));
```

Ensure the bounds at runtime:

```rust
let some_i32 = 4;
let some_wrong_i32 = 8;
assert!(Ranged::<0, 6>::new(some_i32).unwrap() == ranged![4]);
assert!(Ranged::<0, 6>::new(some_wrong_i32) == None);
```

Use remainder operation with the const divisor as a way to create Ranged:

```rust
let x = 15 % ranged![10];
let y: Ranged<0, 9> = x;
assert!(y == ranged![5]); // 15 % 10 == 5
```

This will fail:

```rust
let x = 15 % ranged![10];
let y: Ranged<0, 20> = x;  // Error: x is Ranged<0, 9>
```

Arithmetics: the bounds are automatically recalculated

```rust
let x = ranged![4];
assert!(x+x == ranged![8]);
assert!(x-x == ranged![0]);
assert!(x*ranged![2] == ranged![8]);
```
