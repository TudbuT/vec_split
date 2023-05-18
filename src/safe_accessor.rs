use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use crate::*;

pub struct SafeAccessor<
    'a,
    T: ?Sized,
    const D: usize,
    V: Vector<T, D>,
    I,
    VA: VectorArray<T, D, V, I>,
> {
    phantom_t: PhantomData<T>,
    phantom_v: PhantomData<V>,
    phantom_i: PhantomData<I>,
    array_ref: &'a VA,
    dim: usize,
}

impl<'a, T: ?Sized, const D: usize, V: Vector<T, D>, I, VA: VectorArray<T, D, V, I>>
    SafeAccessor<'a, T, D, V, I, VA>
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
impl<'a, T: ?Sized, const D: usize, V: Vector<T, D>, I, VA: VectorArray<T, D, V, I>> Accessor<T, I>
    for SafeAccessor<'a, T, D, V, I, VA>
{
    fn get<'b>(&'b self, index: I) -> Option<&'b T> {
        self.array_ref.get(index).map(|x| x.get(self.dim).unwrap())
    }
}

impl<'a, T: ?Sized, const D: usize, V: Vector<T, D>, I: Debug, VA: VectorArray<T, D, V, I>> Index<I>
    for SafeAccessor<'a, T, D, V, I, VA>
{
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        self.get(index)
            .expect("index {index:?} is too large for array.")
    }
}

pub struct SafeAccessorMut<
    'a,
    T: ?Sized,
    const D: usize,
    V: Vector<T, D>,
    I,
    VA: VectorArray<T, D, V, I>,
> {
    phantom_t: PhantomData<T>,
    phantom_v: PhantomData<V>,
    phantom_i: PhantomData<I>,
    array_ref: &'a mut VA,
    dim: usize,
}

impl<'a, T: ?Sized, const D: usize, V: Vector<T, D>, I, VA: VectorArray<T, D, V, I>>
    SafeAccessorMut<'a, T, D, V, I, VA>
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

impl<'a, T: ?Sized, const D: usize, V: Vector<T, D>, I, VA: VectorArray<T, D, V, I>> Accessor<T, I>
    for SafeAccessorMut<'a, T, D, V, I, VA>
{
    fn get<'b>(&'b self, index: I) -> Option<&'b T> {
        self.array_ref.get(index).map(|x| x.get(self.dim).unwrap())
    }
}
impl<'a, T: ?Sized, const D: usize, V: Vector<T, D>, I, VA: VectorArray<T, D, V, I>>
    AccessorMut<T, I> for SafeAccessorMut<'a, T, D, V, I, VA>
{
    fn get_mut<'b>(&'b mut self, index: I) -> Option<&'b mut T> {
        self.array_ref
            .get_mut(index)
            .map(|x| x.get_mut(self.dim).unwrap())
    }
}

impl<'a, T: ?Sized, const D: usize, V: Vector<T, D>, I: Debug, VA: VectorArray<T, D, V, I>> Index<I>
    for SafeAccessorMut<'a, T, D, V, I, VA>
{
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        self.get(index)
            .expect("index {index:?} is too large for array.")
    }
}

impl<'a, T: ?Sized, const D: usize, V: Vector<T, D>, I: Debug, VA: VectorArray<T, D, V, I>>
    IndexMut<I> for SafeAccessorMut<'a, T, D, V, I, VA>
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.get_mut(index)
            .expect("index {index:?} is too large for array.")
    }
}
