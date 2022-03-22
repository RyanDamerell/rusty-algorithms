//Mergesort is an efficient algorithm that relies heavily on recursion and can be
// very effectively parallelized with slight adjustments. This implementation is in-place.
pub fn mergesort<T: Ord + Copy>(list: &mut [T]) {
    //call recursive function with initial values
    sort(list, 0, list.len());
}

fn sort<T: Ord + Copy>(list: &mut [T], lo: usize, hi: usize) {
    if hi - lo >= 2 { //if the list has more than 2 elements
        let x = (hi - lo) / 2 + lo;   //divide the list in half
        sort(list, lo, x);              //sort the lower half
        sort(list, x + 1, hi);          //sort sort the upper half
        merge(list, lo, x, x + 1, hi); //merge the two lists
    }
}

fn merge<T: Ord + Copy>(list: &mut [T], s1: usize, e1: usize, s2: usize, e2: usize) {
    if s1 >= e1 || s2 >= e2 || list[e1] <= list[s2] {
        return; // if the segments are already sorted, do nothing
    } else if list[s2] > list[s1] {
        list.swap(s1, s2);
        for i in s1..e1 {
            list.swap(i, i + 1);
        }
    }
    merge(list, s1 + 1, e1, s2, e2);
}
