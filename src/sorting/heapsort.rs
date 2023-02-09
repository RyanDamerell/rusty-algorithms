use std::collections::BinaryHeap;

//Inserting into and removing from a heap is O(log n) complexity
//we can exploit this fact to create an easy sort that's O(n log n)
pub fn heapsort<T: Ord + Copy>(list: &mut [T]) {
    let mut heap = BinaryHeap::with_capacity(list.len());
    for e in list.iter() {
        heap.push(*e);
    }
    for i in (0..list.len()).rev() {
        list[i] = heap.pop().unwrap();
    }
}

//And here's what that code looks like without std hiding things
pub fn custom_heapsort<T: Ord + Copy>(list: &mut [T]) {
    for i in (0..list.len() >> 1).rev() {
        heapify(list, i);
    }
    for i in (0..list.len()).rev() {
        list.swap(0, i);
        heapify(list, 0);
    }
}

fn heapify<T: Ord + Copy>(dat: &mut [T], i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < dat.len() && dat[left] > dat[largest] {
        largest = left;
    }
    if right < dat.len() && dat[right] > dat[largest] {
        largest = right;
    }

    if largest != i {
        dat.swap(i, largest);
        heapify(dat, largest);
    }
}
