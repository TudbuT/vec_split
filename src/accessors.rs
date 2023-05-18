use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use crate::iter::*;

pub trait Accessor<T: ?Sized, I>: Index<I, Output = T> {
    fn get<'b>(&'b self, index: I) -> Option<&'b T>;
}
pub trait AccessorMut<T: ?Sized, I>: Accessor<T, I> + IndexMut<I, Output = T> {
    fn get_mut<'b>(&'b mut self, index: I) -> Option<&'b mut T>;
}

pub trait IterateAccessor<'a, T: ?Sized>: Accessor<T, usize> + 'a + Sized {
    fn iter<'b, 'c>(&'b self) -> AccessorIter<'b, 'c, T, Self> {
        AccessorIter {
            phantom_t: PhantomData,
            safe_accessor: self,
            idx: 0,
        }
    }
}
pub trait IterateAccessorMut<'a, T: ?Sized>: AccessorMut<T, usize> + 'a + Sized {
    fn iter_mut<'b, 'c>(&'b mut self) -> AccessorIterMut<'b, 'c, T, Self> {
        AccessorIterMut {
            phantom_t: PhantomData,
            safe_accessor: self,
            idx: 0,
        }
    }
}

impl<'a, T, A: Accessor<T, usize> + 'a> IterateAccessor<'a, T> for A {}
impl<'a, T, A: AccessorMut<T, usize> + 'a> IterateAccessorMut<'a, T> for A {}
