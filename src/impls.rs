use crate::*;

impl<T, const D: usize, V: Vector<T, D>> VectorArray<T, D, V, usize> for Vec<V> {
    fn get<'a>(&'a self, index: usize) -> Option<&'a V> {
        <[V]>::get(self, index)
    }

    fn get_mut<'a>(&'a mut self, index: usize) -> Option<&'a mut V> {
        <[V]>::get_mut(self, index)
    }
}

impl<T, const D: usize, V: RawVector<T, D>> SizedVectorArray<T, D, V, usize> for Vec<V> {
    fn ptr(&self) -> *const V {
        <[V]>::as_ptr(self)
    }

    fn ptr_mut(&mut self) -> *mut V {
        <[V]>::as_mut_ptr(self)
    }

    fn len(&self) -> usize {
        <[V]>::len(self)
    }

    #[inline]
    fn convert_index(&self, index: usize) -> usize {
        index
    }
}

impl<T: ?Sized, const D: usize, V: Vector<T, D>> VectorArray<T, D, V, usize> for &mut [V] {
    fn get<'a>(&'a self, index: usize) -> Option<&'a V> {
        <[V]>::get(self, index)
    }

    fn get_mut<'a>(&'a mut self, index: usize) -> Option<&'a mut V> {
        <[V]>::get_mut(self, index)
    }
}

impl<T, const D: usize, V: RawVector<T, D>> SizedVectorArray<T, D, V, usize> for &mut [V] {
    fn ptr(&self) -> *const V {
        <[V]>::as_ptr(self)
    }

    fn ptr_mut(&mut self) -> *mut V {
        <[V]>::as_mut_ptr(self)
    }

    fn len(&self) -> usize {
        <[V]>::len(self)
    }

    #[inline]
    fn convert_index(&self, index: usize) -> usize {
        index
    }
}

impl<T: ?Sized, const D: usize, V: Vector<T, D>, const A: usize> VectorArray<T, D, V, usize>
    for [V; A]
{
    fn get<'a>(&'a self, index: usize) -> Option<&'a V> {
        <[V]>::get(self, index)
    }

    fn get_mut<'a>(&'a mut self, index: usize) -> Option<&'a mut V> {
        <[V]>::get_mut(self, index)
    }
}

impl<T, const D: usize, V: RawVector<T, D>, const A: usize> SizedVectorArray<T, D, V, usize>
    for [V; A]
{
    fn ptr(&self) -> *const V {
        <[V]>::as_ptr(self)
    }

    fn ptr_mut(&mut self) -> *mut V {
        <[V]>::as_mut_ptr(self)
    }

    fn len(&self) -> usize {
        A
    }

    #[inline]
    fn convert_index(&self, index: usize) -> usize {
        index
    }
}

macro_rules! impl_tuple {
    ($type:tt, $amount:expr; $($item:tt ,)*) => {
        impl<T: Sized> Vector<T, $amount> for $type {
            fn get<'a>(&'a self, i: usize) -> Option<&'a T> {
                match i {
                    $(
                        $item => Some(&self.$item),
                    )*
                    _ => None,
                }
            }

            fn get_mut<'a>(&'a mut self, i: usize) -> Option<&'a mut T> {
                match i {
                    $(
                        $item => Some(&mut self.$item),
                    )*
                    _ => None,
                }
            }
        }
        unsafe impl<T: Sized> RawVector<T, $amount> for $type {}
    };
}

impl_tuple!((T,                             ),  1; 0,);
impl_tuple!((T, T                           ),  2; 0, 1,);
impl_tuple!((T, T, T                        ),  3; 0, 1, 2,);
impl_tuple!((T, T, T, T                     ),  4; 0, 1, 2, 3,);
impl_tuple!((T, T, T, T, T                  ),  5; 0, 1, 2, 3, 4,);
impl_tuple!((T, T, T, T, T, T               ),  6; 0, 1, 2, 3, 4, 5,);
impl_tuple!((T, T, T, T, T, T, T            ),  7; 0, 1, 2, 3, 4, 5, 6,);
impl_tuple!((T, T, T, T, T, T, T, T         ),  8; 0, 1, 2, 3, 4, 5, 6, 7,);
impl_tuple!((T, T, T, T, T, T, T, T, T      ),  9; 0, 1, 2, 3, 4, 5, 6, 7, 8,);
impl_tuple!((T, T, T, T, T, T, T, T, T, T   ), 10; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,);
impl_tuple!((T, T, T, T, T, T, T, T, T, T, T), 11; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,);

impl<T, const D: usize> Vector<T, D> for [T; D] {
    fn get<'a>(&'a self, index: usize) -> Option<&'a T> {
        <[T]>::get(self, index)
    }

    fn get_mut<'a>(&'a mut self, index: usize) -> Option<&'a mut T> {
        <[T]>::get_mut(self, index)
    }
}

unsafe impl<T, const D: usize> RawVector<T, D> for [T; D] {}

// no SizedVector here, as Vec is not aligned properly for that.
impl<T, const D: usize> Vector<T, D> for Vec<T> {
    fn get<'a>(&'a self, index: usize) -> Option<&'a T> {
        <[T]>::get(self, index)
    }

    fn get_mut<'a>(&'a mut self, index: usize) -> Option<&'a mut T> {
        <[T]>::get_mut(self, index)
    }
}
