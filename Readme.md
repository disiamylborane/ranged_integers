# Ranged integers [nightly only]

[Documentation at docs.rs](https://docs.rs/ranged_integers)

Provides the single generic type `Ranged<MIN, MAX>` representing an integer
within a specified range. It automatically chooses the data size guided by
the range specified (so `Ranged<-50, 50>` is of 1 byte while
`Ranged<-20_000, 100_000>` is of 4 bytes) and supports the arithmetic operations
with automatic bound recalculation.

The conversion and arithmetic functions catch the errors such as possible overflow
and zero division at compile time.

## Example

```rust

#![allow(incomplete_features)]
#![feature(const_generics)]

extern crate ranged_integers;
use ranged_integers::*;

// Consider a simple race game. The player rolls a
// die and then moves forward, backward or forward
// with the double speed according to some rules.

enum MoveType {MoveForward, DoubleSpeed, MoveBackward}

// Get a die roll using a standard random number generator
fn roll_die(rng: &mut dyn rand::RngCore) -> Ranged<1, 6> {
    let random: u8 = rng.gen();
    random % r!(6) + r!(1)  // The consistency is proved at compile time:
                            // r!(6) means Ranged<6,6> with the value 6
                            // r!(1) means Ranged<1,1> with the value 1
                            // u8 % Ranged<6, 6> = Ranged<0, 5>
                            // Ranged<0, 5> + Ranged<1, 1> = Ranged<1, 6>
}

// Calculate where the player must move. The result fits the range -6..=12
fn move_player(move_type: MoveType, dice_points: Ranged<1, 6>) -> Ranged<-6, 12>  {
    match move_type {
        MoveType::MoveForward => {
            dice_points.expand()  // Expands 1..=6 to -6..=12
        }
        MoveType::DoubleSpeed => {
            let mv = dice_points*r!(2);  // mv is Ranged<2, 12>
            mv.expand()  // Expands to -6..=12
        }
        MoveType::MoveBackward => {
            let mv = -dice_points;  // mv is Ranged<-6, -1>
            mv.expand()  // Expands to -6..=12
        }
    }
}
```
