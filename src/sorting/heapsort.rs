
//Inserting into and removing from a heap is O(log n) complexity
// we can exploit this fact to create an easy sort that's O(n log n)
pub fn heapsort<T: Ord + Copy>(list: &mut [T]) {
    let mut heap = Vec::with_capacity(list.len());
    for i in 0..list.len() {
        insert(&mut heap, list[i]);
    }
    for i in 0..heap.len() {
        list[i] = extract(&mut heap);
    }
}

//Insert into heap, making sure that x < y | y is a child of x
fn insert<T: Ord + Copy>(heap: &mut Vec<T>, val: T) {
    let mut i = heap.len();
    heap.push(val);
    while i > 0 {
        if heap[i] < heap[i >> 1] {
            swp(heap, i, i >> 1);
            i = i >> 1;
        } else {
            break;
        }
    }
}

//Extract from the heap, then making sure it stays balanced
fn extract<T: Ord + Copy>(heap: &mut Vec<T>) -> T {
    let mut i = 0;
    let &out = heap.first().unwrap();
    heap[0] = heap.pop().unwrap();
    loop {
        if (i << 1) < heap.len() && heap[i] < heap[i << 1] {
            swp(heap, i, i >> 1);
            i = i << 1;
        } else if (i << 1) + 1 < heap.len() && heap[i] < heap[(i << 1) + 1] {
            swp(heap, i, (i << 1) + 1);
            i = (i << 1) + 1;
        } else {
            break;
        }
    }
    out
}

fn swp<T: Ord + Copy>(list: &mut Vec<T>, a: usize, b: usize) {
    let t = list[a];
    list[a] = list[b];
    list[b] = t;
}
