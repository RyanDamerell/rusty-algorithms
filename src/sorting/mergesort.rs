//Mergesort is an efficient algorithm that relies heavily on recursion and can be
// very effectively parallelized with slight adjustments.
pub fn mergesort<T: Ord + Copy>(list: &mut [T]) {
    sort(list, &mut Vec::with_capacity(list.len()));
}

fn sort<T: Ord + Copy>(list: &mut [T], buffer: &mut Vec<T>) {
    if list.len() >= 2 {
        //if the list has more than 2 elements
        {
            let (lo, hi) = list.split_at_mut(list.len() / 2); //divide the list in half
            mergesort(lo); //sort the lower half
            mergesort(hi); //sort the upper half
        }
        merge(list, buffer);
    }
}

fn merge<T: Ord + Copy>(list: &mut [T], buffer: &mut Vec<T>) {
    let mid = list.len() / 2;
    let mut a = 0;
    let mut b = mid;
    for _ in 0..list.len() {
        if list[a] <= list[b] {
            buffer.push(list[a]);
            a += 1;
        } else {
            buffer.push(list[b]);
            b += 1;
        }

        if a >= mid {
            buffer.extend_from_slice(&list[b..]);
            break;
        } else if b >= list.len() {
            buffer.extend_from_slice(&list[a..mid]);
            break;
        }
    }
    list.copy_from_slice(buffer.as_slice());
    buffer.resize(0, buffer[0]);
}
