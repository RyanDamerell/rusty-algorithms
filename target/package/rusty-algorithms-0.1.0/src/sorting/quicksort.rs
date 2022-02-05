//Quicksort as an algorithm is named for requiring remarkably few comparisons
pub fn quicksort<T: Ord + Copy>(list: &mut [T]) {
    qs(list, 0, list.len());
}

fn qs<T: Ord + Copy>(list: &mut [T], lo: usize, hi: usize) {
    if lo < hi {
        let p = partition(list, lo, hi);
        qs(list, lo, p - 1); //before partition
        qs(list, p + 1, hi); //after partition
    }
}

//partition the list, putting all values larger than the pivot to the right of it
fn partition<T: Ord + Copy>(list: &mut [T], lo: usize, hi: usize) -> usize {
    //Rightmost element selected as pivot. Not always optimal, but fast and easy.
    let pivot = list[hi];
    let mut i = lo;
    for j in lo..hi {
        if list[j] < pivot {
            list.swap(i, j);
            i += 1;
        }
    }
    list.swap(i, hi);
    i
}