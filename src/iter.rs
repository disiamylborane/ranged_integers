use crate::{allow_range, value_check::allow_if};
use super::{Assert, IsAllowed, OperationPossibility, Ranged, irang, memlayout};

use core::convert::TryFrom;

/// An iterator through given range
pub struct Iter<const MIN: irang, const MAX: irang> 
where Assert<{allow_range(memlayout(MIN, MAX))}>: IsAllowed,
{
    pub(crate) current: Option<Ranged<MIN, MAX>>
}

/// Const range for iterators with `Ranged` output and array indexing
///
/// Do not use directly, use [`r!`](macro.r.html) macro instead
///
/// # Example
///
/// ```
/// # #![feature(adt_const_params, generic_const_exprs)] use ranged_integers::*; 
/// let mut fibonacci = [0; 10];
/// fibonacci[0] = 1;
/// fibonacci[1] = 1;
/// for i in r!(2..10) {
///     fibonacci[i.expand()] = fibonacci[(i-r!(1)).expand()] + fibonacci[(i-r!(2)).expand()];
/// }
///
/// let fib234: [_; 3] = fibonacci[r!(2..5)];
/// assert_eq!(fib234, [2,3,5]);
///
/// ```
#[derive(Clone, Copy)]
pub struct ConstInclusiveRange<const MIN: irang, const MAX: irang>
where Assert<{allow_range(memlayout(MIN, MAX))}>: IsAllowed, ;

impl<const MIN: irang, const MAX: irang> IntoIterator for ConstInclusiveRange<MIN, MAX>
where Assert<{allow_range(memlayout(MIN, MAX))}>: IsAllowed,
{
    type Item = Ranged<MIN, MAX>;
    type IntoIter = Iter<MIN, MAX>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter{current: Some(unsafe{Ranged::unchecked_new(MIN)})}
    }
}

impl<const MIN: irang, const MAX: irang> Iterator for Iter<MIN, MAX>
where Assert<{allow_range(memlayout(MIN, MAX))}>: IsAllowed,
{
    type Item = Ranged<MIN, MAX>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe{
            let curr = self.current?;
            let cval = curr.get();
            self.current = if cval == MAX {None} else {Ranged::unchecked_new(cval+1).into()};
            Some(curr)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let range = MAX-MIN;
        usize::try_from(range).map_or((usize::MAX, None), |rangeus| (rangeus, Some(rangeus)))
    }

    #[inline]
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
        Some(unsafe{Ranged::unchecked_new(MAX)})
    }

    fn min(mut self) -> Option<Self::Item> {
        self.next()
    }

    fn max(self) -> Option<Self::Item> {
        self.last()
    }
}

#[doc(hidden)]
pub const fn range_fits_usize(min: irang, max: irang) -> OperationPossibility {
    allow_if((max-min) < (usize::MAX as i128))
}

impl<const MIN: irang, const MAX: irang> ExactSizeIterator for Iter<MIN, MAX>
where
    Assert<{allow_range(memlayout(MIN, MAX))}>: IsAllowed,
    Assert<{range_fits_usize(MAX, MIN)}>: IsAllowed,
{}

