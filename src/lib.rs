pub mod data;
pub mod graph;
pub mod numbers;
pub mod sorting;

#[cfg(test)]
mod tests {
    use super::sorting::{heapsort::*, mergesort::*, quicksort::*, simplesorts::*, utils::*};
    use itertools::Itertools;
    use rayon::prelude::*;

    const NUM_ELEMENTS: usize = 10;

    #[test]
    fn test_sorts() {
        assert!(is_sorted(&[1, 2, 3, 5, 7, 7, 10]));
        assert!(is_sorted(&[-82, -28, -23, -3, 10, 17, 31, 57, 87]));

        let mut list = [0; NUM_ELEMENTS];
        list.par_iter_mut()
            .for_each(|x| *x = rand::random::<i64>() % 100);
        selection_sort(&mut list);
        assert!(is_sorted(&list), "[{}]", list.iter().join(","));

        scramble(&mut list);
        insertion_sort(&mut list);
        assert!(is_sorted(&list), "[{}]", list.iter().join(","));

        scramble(&mut list);
        mergesort(&mut list);
        assert!(is_sorted(&list), "[{}]", list.iter().join(","));

        scramble(&mut list);
        quicksort(&mut list);
        assert!(is_sorted(&list), "[{}]", list.iter().join(","));

        scramble(&mut list);
        efficient_heapsort(&mut list);
        assert!(is_sorted(&list), "[{}]", list.iter().join(","));
    }

    // #[test]
    // fn test_data() {}
}
