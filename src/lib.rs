#![allow(clippy::needless_lifetimes)]
pub mod accessors;
pub mod fast_accessor;
mod impls;
mod iter;
pub mod safe_accessor;

use std::mem;

use accessors::*;
use fast_accessor::*;
use safe_accessor::*;

/// Trait to be implemented for all types that are vector-like. For example,
/// arrays, tuples, vecs, etc.
pub trait Vector<T: ?Sized, const D: usize> {
    fn get<'a>(&'a self, i: usize) -> Option<&'a T>;
    fn get_mut<'a>(&'a mut self, i: usize) -> Option<&'a mut T>;
}

/// Trait to be implemented for all types that are arrays of some sort and
/// contain only vectors.
pub trait VectorArray<T: ?Sized, const D: usize, V: Vector<T, D>, I>: Sized {
    fn get<'a>(&'a self, index: I) -> Option<&'a V>;
    fn get_mut<'a>(&'a mut self, index: I) -> Option<&'a mut V>;

    fn vec_split_safe<'a>(&'a self) -> [SafeAccessor<'a, T, D, V, I, Self>; D] {
        let mut array = Vec::new();
        for i in 0..D {
            array.push(SafeAccessor::new(self, i));
        }
        let mut array = array.into_iter();
        [0; D].map(|_| array.next().unwrap())
    }

    fn vec_split_safe_mut<'a>(&'a mut self) -> [SafeAccessorMut<'a, T, D, V, I, Self>; D] {
        let mut array = Vec::new();
        // SAFETY: SafeAccessorMut will ONLY access one dimension, and all other parts of the
        // array will be left alone. This means one part of RAM will only be accessible once.
        unsafe {
            for i in 0..D {
                array.push(SafeAccessorMut::new(&mut *(self as *mut _), i));
            }
        }
        let mut array = array.into_iter();
        [0; D].map(|_| array.next().unwrap())
    }
}

/// Trait to be implemented for all types that are arrays of some sort, contain
/// only vectors, **and have a way to access their internal pointers**. Used to
/// allow the use of [`FastAccessor`]s.
pub trait SizedVectorArray<T: Sized, const D: usize, V: RawVector<T, D>, I>:
    VectorArray<T, D, V, I>
{
    fn ptr(&self) -> *const V;
    fn ptr_mut(&mut self) -> *mut V;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Please make this inline for speed
    fn convert_index(&self, index: I) -> usize;

    fn vec_split_fast<'a>(&'a self) -> [FastAccessor<'a, T, D, V, I, Self>; D] {
        let mut array = Vec::new();
        for i in 0..D {
            array.push(FastAccessor::new(self, i));
        }
        let mut array = array.into_iter();
        [0; D].map(|_| array.next().unwrap())
    }

    fn vec_split_fast_mut<'a>(&'a mut self) -> [FastAccessorMut<'a, T, D, V, I, Self>; D] {
        let mut array = Vec::new();
        // SAFETY: SafeAccessorMut will ONLY access one dimension, and all other parts of the
        // array will be left alone. This means one part of RAM will only be accessible once.
        unsafe {
            for i in 0..D {
                array.push(FastAccessorMut::new(&mut *(self as *mut _), i));
            }
        }
        let mut array = array.into_iter();
        [0; D].map(|_| array.next().unwrap())
    }
}

/// Trait used to show that a vector-like type is guaranteed to have a
/// consistent and usable-for-pointers memory layout.
///
/// # Safety
///
/// MUST have no extra items before first dimension in memory, MUST
/// not have padding between items!! This means Vec, for example, is **NOT**
/// fit for this trait. [T; D], for example, works.
pub unsafe trait RawVector<T: Sized, const D: usize>: Vector<T, D> {}

#[cfg(test)]
mod test {
    use crate::{Accessor, SizedVectorArray, VectorArray};

    #[test]
    fn safe_accessor_vec() {
        let vec = vec![(1, 2), (3, 4), (5, 6)];
        let vec = vec.vec_split_safe();
        assert_eq!(vec[0].get(0).unwrap(), &1);
        assert_eq!(vec[1].get(0).unwrap(), &2);
        assert_eq!(vec[0].get(1).unwrap(), &3);
        assert_eq!(vec[1].get(1).unwrap(), &4);
        assert_eq!(vec[0].get(2).unwrap(), &5);
        assert_eq!(vec[1].get(2).unwrap(), &6);
        for dimension in vec {
            for i in 0..2 {
                println!("{}", dimension.get(i).unwrap());
            }
            println!();
        }
    }

    #[test]
    fn fast_accessor_vec() {
        let vec = vec![(1, 2), (3, 4), (5, 6)];
        let vec = vec.vec_split_fast();
        assert_eq!(vec[0].get(0).unwrap(), &1);
        assert_eq!(vec[1].get(0).unwrap(), &2);
        assert_eq!(vec[0].get(1).unwrap(), &3);
        assert_eq!(vec[1].get(1).unwrap(), &4);
        assert_eq!(vec[0].get(2).unwrap(), &5);
        assert_eq!(vec[1].get(2).unwrap(), &6);
        for dimension in vec {
            for i in 0..2 {
                println!("{}", dimension.get(i).unwrap());
            }
            println!();
        }
    }
}
