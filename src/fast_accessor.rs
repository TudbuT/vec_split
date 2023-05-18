//! Despite the name, FastAccessor is not much less safe than SafeAccessor.
//! However, it does not require T: Sized, or I: usize.

use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use crate::*;

pub struct FastAccessor<
    'a,
    T: Sized,
    const D: usize,
    V: RawVector<T, D>,
    I,
    VA: SizedVectorArray<T, D, V, I>,
> {
    phantom_t: PhantomData<T>,
    phantom_v: PhantomData<V>,
    phantom_i: PhantomData<I>,
    array_ref: &'a VA,
    dim: usize,
}

impl<'a, T: Sized, const D: usize, V: RawVector<T, D>, I, VA: SizedVectorArray<T, D, V, I>>
    FastAccessor<'a, T, D, V, I, VA>
{
    pub(crate) fn new(array_ref: &'a VA, dim: usize) -> Self {
        Self {
            phantom_t: PhantomData,
            phantom_v: PhantomData,
            phantom_i: PhantomData,
            array_ref,
            dim,
        }
    }
}

impl<'a, T: Sized, const D: usize, V: RawVector<T, D>, I, VA: SizedVectorArray<T, D, V, I>>
    Accessor<T, I> for FastAccessor<'a, T, D, V, I, VA>
{
    fn get<'b>(&'b self, index: I) -> Option<&'b T> {
        let idx = self.array_ref.convert_index(index);
        if idx > self.array_ref.len() {
            return None;
        }
        // SAFETY: implementing SizedVector requires memory layout to be sound for this operation:
        // the first dimension MUST be at the memory offset of the object, and it MUST not have padding
        // in-between dimensions
        unsafe {
            (self.array_ref.ptr().add(idx) as *const T).add(self.dim)
                .as_ref()
        }
    }
}

impl<'a, T: Sized, const D: usize, V: RawVector<T, D>, I, VA: SizedVectorArray<T, D, V, I>> Index<I>
    for FastAccessor<'a, T, D, V, I, VA>
{
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        self.get(index).expect("index is too large for array.")
    }
}

pub struct FastAccessorMut<
    'a,
    T: Sized,
    const D: usize,
    V: RawVector<T, D>,
    I,
    VA: SizedVectorArray<T, D, V, I>,
> {
    phantom_t: PhantomData<T>,
    phantom_v: PhantomData<V>,
    phantom_i: PhantomData<I>,
    array_ref: &'a mut VA,
    dim: usize,
}

impl<'a, T: Sized, const D: usize, V: RawVector<T, D>, I, VA: SizedVectorArray<T, D, V, I>>
    FastAccessorMut<'a, T, D, V, I, VA>
{
    pub(crate) fn new(array_ref: &'a mut VA, dim: usize) -> Self {
        Self {
            phantom_t: PhantomData,
            phantom_v: PhantomData,
            phantom_i: PhantomData,
            array_ref,
            dim,
        }
    }
}

impl<'a, T: Sized, const D: usize, V: RawVector<T, D>, I, VA: SizedVectorArray<T, D, V, I>>
    Accessor<T, I> for FastAccessorMut<'a, T, D, V, I, VA>
{
    fn get<'b>(&'b self, index: I) -> Option<&'b T> {
        let idx = self.array_ref.convert_index(index);
        if idx > self.array_ref.len() {
            return None;
        }
        // SAFETY: implementing SizedVector requires memory layout to be sound for this operation:
        // the first dimension MUST be at the memory offset of the object, and it MUST not have padding
        // in-between dimensions
        unsafe {
            (self.array_ref.ptr().add(idx) as *const T).add(self.dim)
                .as_ref()
        }
    }
}
impl<'a, T: Sized, const D: usize, V: RawVector<T, D>, I, VA: SizedVectorArray<T, D, V, I>>
    AccessorMut<T, I> for FastAccessorMut<'a, T, D, V, I, VA>
{
    fn get_mut<'b>(&'b mut self, index: I) -> Option<&'b mut T> {
        let idx = self.array_ref.convert_index(index);
        if idx > self.array_ref.len() {
            return None;
        }
        // SAFETY: implementing SizedVector requires memory layout to be sound for this operation:
        // the first dimension MUST be at the memory offset of the object, and it MUST not have padding
        // in-between dimensions
        unsafe {
            (self.array_ref.ptr().add(idx) as *mut T).add(self.dim)
                .as_mut()
        }
    }
}

impl<'a, T: Sized, const D: usize, V: RawVector<T, D>, I, VA: SizedVectorArray<T, D, V, I>> Index<I>
    for FastAccessorMut<'a, T, D, V, I, VA>
{
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        self.get(index).expect("index is too large for array.")
    }
}

impl<'a, T: Sized, const D: usize, V: RawVector<T, D>, I, VA: SizedVectorArray<T, D, V, I>>
    IndexMut<I> for FastAccessorMut<'a, T, D, V, I, VA>
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.get_mut(index).expect("index is too large for array.")
    }
}
