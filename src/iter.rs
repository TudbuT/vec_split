use std::marker::PhantomData;

use crate::*;

pub struct AccessorIter<'a: 'b, 'b, T: ?Sized, A: Accessor<T, usize> + 'a> {
    pub(crate) phantom_t: PhantomData<&'b T>,
    pub(crate) safe_accessor: &'a A,
    pub(crate) idx: usize,
}

impl<'a: 'b, 'b, T: ?Sized, A: Accessor<T, usize> + 'a> Iterator for AccessorIter<'a, 'b, T, A> {
    type Item = &'b T;

    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: &'c T -> &'a T
        // Guaranteed to be valid: 'a outlives T and the accessor.
        unsafe { mem::transmute(self.safe_accessor.get((self.idx, self.idx += 1).0)) }
    }
}

pub struct AccessorIterMut<'a: 'b, 'b, T: ?Sized, A: AccessorMut<T, usize> + 'a> {
    pub(crate) phantom_t: PhantomData<&'b T>,
    pub(crate) safe_accessor: &'a A,
    pub(crate) idx: usize,
}

impl<'a: 'b, 'b, T: ?Sized, A: AccessorMut<T, usize> + 'a> Iterator
    for AccessorIterMut<'a, 'b, T, A>
{
    type Item = &'b mut T;

    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: &'c T -> &'a T
        // Guaranteed to be valid: 'a outlives T and the accessor.
        unsafe { mem::transmute(self.safe_accessor.get((self.idx, self.idx += 1).0)) }
    }
}
