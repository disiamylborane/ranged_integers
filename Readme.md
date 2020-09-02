# Ranged

The `Ranged` is a current nightly Rust compatible
library, providing a wrapper around u8 to make sure
of the value bounds.

```rust
fn consume_ranged(a: Ranged<2,31>) {
    // {a} is between 2 and 31 inclusively
    let x : u8 = a.val(); // Convert back to int
}

// Compile time checking:
consume_ranged( ranged!(<2 31> 2) );  // Success
consume_ranged( ranged!(<2 31> 1) );  // Compile error: invalid bounds
consume_ranged( ranged!(<1 31> 1) );  // Compile error: wrong type

// Runtime checking:
let x_input : u8 = user_input_u8();
let x = Ranged::try_new(x_input);
if let Some(xval) = x {
    consume_ranged(xval);
}
```

The original library (implementing ranged integers
arithmetics and autosize) is saved on 'full_version' branch
(compiles on nightly-2020-05-19 and earlier)
