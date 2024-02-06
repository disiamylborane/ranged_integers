# Ranged integers [nightly only]

**Note: the library causes ICEs on some Rust toolchains.**
**The current version (0.8.0) was tested on nightly-2024-02-04.**

[Documentation at docs.rs](https://docs.rs/ranged_integers)

[Sudoku example](https://github.com/disiamylborane/ranged_integers/blob/master/examples/sudoku.rs)

[Changelog](https://github.com/disiamylborane/ranged_integers/blob/master/CHANGELOG.md)

Provides a generic type `Ranged<MIN, MAX>` representing an integer
within a specified range. It automatically chooses the data size guided by
the range specified (so `Ranged<-50, 50>` is of 1 byte while
`Ranged<-20_000, 100_000>` is of 4 bytes) and supports the arithmetic operations
with automatic bound recalculation and range iteration / fixed-size array indexing.

The conversion and arithmetic functions catch the errors such as possible overflow
and zero division at compile time.

## Example

```rust
#![allow(incomplete_features)]
#![feature(adt_const_params, generic_const_exprs)]

extern crate ranged_integers;
use ranged_integers::*;

// Consider a simple race game. The player rolls a
// die and then moves forward, backward or forward
// with the double speed according to some rules.

enum MoveType {MoveForward, DoubleSpeed, MoveBackward}

// Get a die roll using a standard random number generator
fn roll_die(rng: &mut dyn rand::RngCore) -> Ranged<1, 6> {
    let random: u8 = rng.gen();
        // The consistency is proved at compile time:
        // r!(6) means Ranged<6,6> with the value 6
        // r!(1) means Ranged<1,1> with the value 1
        // u8 % Ranged<6, 6> = Ranged<0, 5>
        // Ranged<0, 5> + Ranged<1, 1> = Ranged<1, 6>
    random % r!(6) + r!(1)
}

// Calculate where the player must move
// The result fits the range -6..=12
fn move_player(
    mtype: MoveType, 
    dice_points: Ranged<1, 6>
) -> Ranged<-6, 12>
{
    match move_type {
        MoveType::MoveForward => {
            // Expand 1..=6 to -6..=12
            dice_points.expand()
        }
        MoveType::DoubleSpeed => {
            let mv = dice_points*r!(2); // Ranged<2, 12>
            mv.expand() // Expand to -6..=12
        }
        MoveType::MoveBackward => {
            let mv = -dice_points; // Ranged<-6, -1>
            mv.expand() // Expand to -6..=12
        }
    }
}
```
