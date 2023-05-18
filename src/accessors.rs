use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use crate::iter::*;

#[macro_export]
/// Macro to easily implement the Accessor types for your own array types.
macro_rules! impl_accessors {
    ($type:ty, $getter:ty, unknown $index_constraints:tt) => {
        impl<'a, T, I: $index_constraints> Accessor<T, I> for $type {
            fn get<'b>(&'b self, index: I) -> Option<&'b T> {
                <$getter>::get(self, index)
            }
        }
        impl<'a, T, I: $index_constraints> AccessorMut<T, I> for $type {
            fn get_mut<'b>(&'b mut self, index: I) -> Option<&'b mut T> {
                <$getter>::get_mut(self, index)
            }
        }
    };
    ($type:ty, $getter:ty, known $index:ty) => {
        impl<'a, T> Accessor<T, $index> for $type {
            fn get<'b>(&'b self, index: $index) -> Option<&'b T> {
                <$getter>::get(self, index)
            }
        }
        impl<'a, T> AccessorMut<T, $index> for $type {
            fn get_mut<'b>(&'b mut self, index: $index) -> Option<&'b mut T> {
                <$getter>::get_mut(self, index)
            }
        }
    };
}

/// Trait used to unify the different accessors (Fast, Safe)
pub trait Accessor<T: ?Sized, I>: Index<I, Output = T> {
    /// Gets an item out of the array. This maybe be a split array, in which
    /// case the operation will take an item out of the vector.
    fn get<'b>(&'b self, index: I) -> Option<&'b T>;
}
/// Trait used to unify the different accessors's mutable variants (Fast, Safe)
pub trait AccessorMut<T: ?Sized, I>: Accessor<T, I> + IndexMut<I, Output = T> {
    /// Gets an item out of the array mutably. This maybe be a split array, in
    /// which case the operation will take an item out of the vector.
    fn get_mut<'b>(&'b mut self, index: I) -> Option<&'b mut T>;
}

/// Trait used to add the `iter` method to structs implementing [`Accessor`]
pub trait IterateAccessor<'a, T: ?Sized>: Accessor<T, usize> + 'a + Sized {
    fn iter<'b, 'c>(&'b self) -> AccessorIter<'b, 'c, T, Self> {
        AccessorIter {
            phantom_t: PhantomData,
            safe_accessor: self,
            idx: 0,
        }
    }
}
/// Trait used to add the `iter_mut` method to structs implementing
/// [`AccessorMut`]
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

impl_accessors!(Vec<T>, [T], known usize);
impl_accessors!([T], [T], known usize);
