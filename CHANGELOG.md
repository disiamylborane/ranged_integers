# Changelog

## 0.10.1

Works on nightly-2025-06-24-x86_64
- **Fixed** soundness hole in array slicing.

## 0.10.0

Works on nightly-2025-06-19.
! Contains undefined behavior in public interface

- **Added** const arithmetics via dedicated methods.
- **Changed** shrunk the bounds or Ranged to hold not wider than i64 or u64, `i128` layout
  has been removed. This was taken to prevent the non-helping error message when the
  bounds are violated, which does not show the caller place.
- **Changed** the internals to stop causing ICEs be supported by the current version of Rust.
- **Changed** switched to 2024 edition of Rust.
- **Removed** the deprecated try_expand method.

## 0.9.0

Works on nightly-2024-10-12.
- **Removed** arithmetic operators in const context, which are not supported by the current
  version of Rust (nightly-2024-10-12).

## 0.8.0

Works on nightly-2024-02-04
- **Changed** updated to the new version of Rust (nightly-2024-02-04).
- (**Changed** to prevent ICEs, the internals were reverted to the old byte array-based version;
the specialization feature was disabled and an additional trait bound was added to Ranged type out of necessity).

## 0.7.1

Works on nightly-2022-11-26
- **Changed** const-ify array indexing traits.
- **Added** type constraining comparisons `fit_less_than`, `fit_less_eq`, `fit_greater_than`, `fit_greater_eq`.
- (**Changed** the naming of internals).

## 0.7.0

Works on nightly-2022-11-26
- **Changed** updated to the new version of Nightly rust.
- **Changed** globally revamped the internal structure of Ranged.
- **Added** `rmatch` macro and pattern matching over Ranged.

## 0.6.0

- **Removed** `range` function.
- **Changed** `try_expand` into `fit` function (deprecated `try_expand`).
- **Added** `fit_min` and `fit_max` functions.
- **Added** comparisons.
- **Added** `ConstInclusiveRange` zero-size structure with `IntoIterator` instead of `fn range()`.
- **Added** slicing arrays by `ConstInclusiveRange` with fixed-size array reference output.
- **Fixed** `r!` revamp and support of ranges (MIN..END and MIN..=MAX)

## 0.5.1

- **Added** `r!` support of `range`.
- **Added** `FromStr` trait for ranged.
- **Added** Comparisons `i128` vs `Ranged`.
- **Fixed** Auto Trait Implementations

## 0.5.0

- **Changed** updated to the new version of Nightly rust.
- **Added** `abs`, `div_euclid`, `rem_euclid` functions.

## 0.4.2

- **Added** iterator over ranges, `iter_from` method and `range` function, indexing arrays by `Ranged`.
- **Added** [sudoku](examples/sudoku.rs) example.

## 0.4.1

- **Added** to and from `isize` and `usize` conversions

## 0.4.0

- **Changed**: the "constant" `Ranged<N, N>` types made zero-sized

## 0.3.1

- **Fixed** `r!` macro  failure with `const_evaluatable_checked` enabled
- **Added** `min` and `max` functions

## 0.3.0

First release

## 0.1 - 0.2

Not published, built for the old versions of nightly Rust
