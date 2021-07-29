
use super::{Assert, IsAllowed, OperationPossibility, Ranged, irang, memlayout};

use core::convert::TryFrom;

/// An iterator through given range
pub struct Iter<const MIN: irang, const MAX: irang> 
where [u8; memlayout(MIN, MAX).bytes()]:,
{
    pub(crate) current: Option<Ranged<MIN, MAX>>
}


pub const fn rstart(r: core::ops::Range<irang>) -> irang{r.start}
pub const fn rlast(r: core::ops::Range<irang>) -> irang{r.end-1}


/// Create a range iterator with `Ranged` output
///
/// # Example
///
/// ```
/// # #![feature(const_generics, const_evaluatable_checked)] use ranged_integers::*; 
/// let mut arr = [0; 10];
/// arr[0] = 1;
/// arr[1] = 1;
/// for i in range::<{2..9}>() {
///     arr[i.expand()] = arr[(i-r!(1)).expand()] + arr[(i-r!(2)).expand()];
/// }
/// ```
#[must_use]
pub fn range<const RANGE: core::ops::Range<irang>>() -> Iter<{rstart(RANGE)}, {rlast(RANGE)}>
where [(); memlayout(rstart(RANGE), rlast(RANGE)).bytes()]:
{
    Iter::<{rstart(RANGE)}, {rlast(RANGE)}>{current: unsafe{Ranged::__unsafe_new(RANGE.start).into()}}
}


impl<const MIN: irang, const MAX: irang> Iterator for Iter<MIN, MAX>
where [u8; memlayout(MIN, MAX).bytes()]:,
{
    type Item = Ranged<MIN, MAX>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe{
            let curr = self.current?;
            let cval = curr.get();
            self.current = if cval == MAX {None} else {Ranged::__unsafe_new(cval+1).into()};
            Some(curr)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let range = MAX-MIN;
        if let Ok(rangeus) = usize::try_from(range) {
            (rangeus, Some(rangeus))
        }
        else {(usize::MAX, None)}
    }

    #[inline]
    #[allow(clippy::option_if_let_else)]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let cval = self.current?;
        if let Some(out) = Ranged::new(cval.get() + n as i128) {
            self.current = Some(out);
            self.next()
        } else {
            self.current = None;
            None
        }
    }

    fn last(self) -> Option<Self::Item> {
        self.current?;
        Some(unsafe{Ranged::__unsafe_new(MAX)})
    }

    fn min(mut self) -> Option<Self::Item> {
        self.next()
    }

    fn max(self) -> Option<Self::Item> {
        self.last()
    }
}


impl<const MIN: irang, const MAX: irang> ExactSizeIterator for Iter<MIN, MAX>
where
    [u8; memlayout(MIN, MAX).bytes()]:,
    Assert<{OperationPossibility::allow_if(MAX-MIN < usize::MAX as i128)}>: IsAllowed,
{}

