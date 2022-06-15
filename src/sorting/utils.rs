use itertools::Itertools;
use rand::Rng;

pub fn is_sorted<T: Ord + Copy>(list: &[T]) -> bool {
    list.iter().tuple_windows().all(|(a, b)| a <= b)
}

pub fn scramble<T: Ord + Copy>(list: &mut [T]) {
    let mut rng = rand::thread_rng();
    for i in 0..list.len() {
        list.swap(i, rng.gen_range(i..list.len()));
    }
}
