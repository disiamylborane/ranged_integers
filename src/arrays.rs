use crate::{allow_range, conversions, irang, memlayout, Assert, ConstInclusiveRange, IsAllowed, Ranged};

#[allow(clippy::cast_sign_loss)]
impl<T, const N: usize> core::ops::Index<Ranged<0, {N as i128 - 1}>> for [T; N]
where
    // Constraint required by Ranged
    Assert<{allow_range(memlayout(0, N as i128 - 1))}>: IsAllowed,
    // Check if the range fits the array length
    Assert<{conversions::converter_checkers::usize(0, N as i128 - 1)}>: IsAllowed
{
    type Output = T;
    fn index(&self, index: Ranged<0, {N as i128 - 1}>) -> &Self::Output {
        unsafe{self.get_unchecked(index.usize())}
    }
}

#[allow(clippy::cast_sign_loss)]
impl<T, const N: usize> core::ops::IndexMut<Ranged<0, {N as i128 - 1}>> for [T; N]
where
    // Constraint required by Ranged
    Assert<{allow_range(memlayout(0, N as i128 - 1))}>: IsAllowed,
    // Check if the range fits the array length
    Assert<{conversions::converter_checkers::usize(0, N as i128 - 1)}>: IsAllowed
{
    fn index_mut(&mut self, index: Ranged<0, {N as i128 - 1}>) -> &mut Self::Output {
        unsafe{self.get_unchecked_mut(index.usize())}
    }
}

#[allow(clippy::cast_sign_loss)]
impl<T, const N: usize, const MIN: irang, const MAX: irang>
core::ops::Index<ConstInclusiveRange<MIN, MAX>> for [T; N] 
where
    // Constraint required by ConstInclusiveRange
    Assert<{allow_range(memlayout(MIN, MAX))}>: IsAllowed,
    // Constraint required by the output array
    [T; (MAX-MIN+1) as usize]:,
    // Check if the range fits the array length (min)
    Assert<{conversions::converter_checkers::usize(MIN, MAX)}>: IsAllowed,
    // Check if the range fits the array length (max)
    Assert<{conversions::lessthan(MAX, N as i128)}>: IsAllowed,
{
    type Output = [T; (MAX-MIN+1) as usize];
    fn index(&self, _index: ConstInclusiveRange<MIN, MAX>) -> &Self::Output {
        unsafe{
            &*self.get_unchecked((MIN as usize)..=(MAX as usize)).as_ptr().cast()
        }
    }
}

#[allow(clippy::cast_sign_loss)]
impl<T, const N: usize, const MIN: irang, const MAX: irang>
core::ops::IndexMut<ConstInclusiveRange<MIN, MAX>> for [T; N] 
where
    // Constraint required by ConstInclusiveRange
    Assert<{allow_range(memlayout(MIN, MAX))}>: IsAllowed,
    // Constraint required by the output array
    [T; (MAX-MIN+1) as usize]:,
    // Check if the range fits the array length (min)
    Assert<{conversions::converter_checkers::usize(MIN, MAX)}>: IsAllowed,
    // Check if the range fits the array length (max)
    Assert<{conversions::lessthan(MAX, N as i128)}>: IsAllowed,
{
    fn index_mut(&mut self, _index: ConstInclusiveRange<MIN, MAX>) -> &mut Self::Output {
        unsafe{
            &mut *self.get_unchecked_mut((MIN as usize)..=(MAX as usize)).as_mut_ptr().cast()
        }
    }
}
