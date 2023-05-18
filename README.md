# vec_split

vec_split is a library to work with arrays of vectors as if they were instead 
one array for each dimension of the vector.

## Example

```rs
let mut array = [[0.0, 0.0]; 10];
let [mut x_array, mut y_array] = array.vec_split_fast_mut();
for (i, item) in x_array.iter_mut().enumerate() {
    *item = i as f64 / 10.0;
}
for (i, item) in y_array.iter_mut().enumerate() {
    *item = i as f64 * 2.0;
}
for item in x_array.iter() {
    println!("X {item}");
}
for item in y_array.iter() {
    println!("Y {item}");
}
```

## Safety and Accessor types

As is visible in the example, vec_split allows multiple mutable references to
what is technically all the same array. This is SAFE because each part of the
split array cannot write where the others do, meaning they are effectively
separate.

`FastAccessor` also uses unsafe code to do some pointer manipulation. Because
we want it to still be safe, the SizedVector trait has some special 
requirements. If these are always ensured, FastAccessor IS SAFE.

> SAFETY: MUST have no extra items before first dimension in memory, MUST
> not have padding between items!! This means Vec, for example, is **NOT**
> fit for this trait. `[T; D]`, for example, works.

This is essentially just saying that the vector MUST be an array, just like
`[T; D]`. `Vec<T>` however does not work for this, as it has a lot of extra
room around the items. If this is the case `SafeAccessor` must be used, which
may be a small bit slower.

SafeAccessor uses no memory manipulation, so it may be slower, but it also
doesn't have any way to cause UB or some other weirdness if some trait is
implemented wrongly.
