//This file is full of sorts that are usually considered trivial to implement.
//They're easy to accomplish and get decent performance

//Insertion sort goes through and inserts each element where it belongs in the initial sublist
pub fn insertion_sort<T: Ord + Copy>(list: &mut [T]) {
    for i in 1..list.len() {
        let mut t = &list[i];
        let mut j;
        match list[0..i].binary_search(t) {
            //find the position t should be inserted into
            Ok(v) => {
                j = v - 1;
            }
            Err(v) => {
                j = v;
            }
        }
        while j <= i {
            list[j] = *t;
            j += 1;
            t = &list[j];
        }
    }
}

//Selection sort is sort of the opposite, finding the minimum of the remaining elements
//and placing it in its appropriate position to the left of all other remaining elements
pub fn selection_sort<T: Ord + Copy>(list: &mut [T]) {
    for i in 0..list.len() {
        //get min of the remaining elements and return the index, idiomatically
        if let Some((t, _)) = list[i..]
            .iter()
            .enumerate()
            .min_by(|(_, v), (_, c)| v.cmp(c))
        {
            list.swap(t, i); //replace the i-th element with the smallest element
        }
    }
}
