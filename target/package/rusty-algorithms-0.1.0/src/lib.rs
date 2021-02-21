pub mod data;
pub mod sorting;
pub mod graph;

#[cfg(test)]
mod tests {
    use super::sorting::{mergesort::*, utils::*, quicksort::*, heapsort::*, simplesorts::*};
    use rayon::prelude::*;

    const NUM_ELEMENTS: usize = 10_000;

    #[test]
    fn test_sorts() {

        //sorts
        let mut list = [0; NUM_ELEMENTS];
        list.par_iter_mut().for_each(|x| *x = rand::random());
        mergesort(&mut list);
        assert!(is_sorted(&list));

        scramble(&mut list);
        quicksort(&mut list);
        assert!(is_sorted(&list));

        scramble(&mut list);
        heapsort(&mut list);
        assert!(is_sorted(&list));

        scramble(&mut list);
        selection_sort(&mut list);
        assert!(is_sorted(&list));

        scramble(&mut list);
        insertion_sort(&mut list);
        assert!(is_sorted(&list));

        //graphs

        //data
        
    }
}
