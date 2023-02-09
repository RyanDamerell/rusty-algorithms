use itertools::Itertools;

//Quicksort as an algorithm is named for requiring remarkably few comparisons
pub fn quicksort<T: Ord + Copy>(list: &mut [T]) {
    if list.len() > 3 {
        let p = partition(list);
        let (lo, hi) = list.split_at_mut(p);
        quicksort(lo);
        quicksort(&mut hi[1..])
    } else if list.len() > 1 {
        let (i, min) = list.iter().enumerate().min_by_key(|(_, &v)| v).unwrap();
        if list[0] != *min {
            list.swap(0, i);
        }
        quicksort(&mut list[1..])
    }
}

//partition the list, putting all values larger than the pivot to the right of it
fn partition<T: Ord + Copy>(list: &mut [T]) -> usize {
    //Rightmost element selected as pivot. Not always optimal, but fast and easy.
    //It's also possible to take the middle of 3 different values, preventing the worst case scenario
    let pivot = *list.last().unwrap();
    let mut left = 0;
    let mut right = list.len() - 2;
    loop {
        while list[left] < pivot && left <= right {
            left += 1;
        }
        while list[right] > pivot && left <= right {
            right -= 1;
        }
        if left == right {
            break;
        } else {
            list.swap(left, right);
        }
    }
    if list[left] > pivot {
        list.swap(left, list.len() - 1);
    }
    left
}
