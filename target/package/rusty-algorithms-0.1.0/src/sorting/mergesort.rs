//Mergesort is an efficient algorithm that relies heavily on recursion and can be
// very effectively parallelized with slight adjustments
pub fn mergesort<T: Ord + Copy>(list: &mut [T]) {
    sort(list, 0, list.len());
}

fn sort<T: Ord + Copy>(list: &mut [T], lo: usize, hi: usize) {
    if hi - lo >= 2 {
        let x = (hi - lo) / 2 + lo;
        sort(list, lo, x);
        sort(list, x + 1, hi);
        merge(list, lo, x, x + 1, hi);
    }
}

fn merge<T: Ord + Copy>(list: &mut [T], s1: usize, e1: usize, s2: usize, e2: usize) {
    if s1 >= e1 || s2 >= e2 || list[e1] <= list[s2] {
        return;
    } else if list[s2] > list[s1] {
        list.swap(s1, s2);
        for i in s1..e1 {
            list.swap(i, i + 1);
        }
    }
    merge(list, s1 + 1, e1, s2, e2);
}
