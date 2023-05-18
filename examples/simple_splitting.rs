use vec_split::{
    accessors::{IterateAccessor, IterateAccessorMut},
    SizedVectorArray,
};

fn main() {
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
}
