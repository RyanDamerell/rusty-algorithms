use super::utils::{is_sorted, scramble};

//a collection of impractical sorts

//Bogosort is not used in any situation. It's purely an example of the dumbest possible sort.
//It simply randomizes the list, and if it's not sorted, loop.
pub fn bogosort<T: Ord + Copy>(list: &mut [T]) {
    while !is_sorted(list) {
        scramble(list);
    }
}

//Bubblesort is often considered the brute force or "naive" solution to the sorting problem.
//Though it can theoretically be effective in almost-sorted lists, it's poor scaling and bad
//empirical performance leads it to almost never be used.
pub fn bubblesort<T: Ord + Copy>(list: &mut [T]) {
    let mut max = list.len() - 1; //we don't need to loop through the whole list each time, it reduces each loop
    while max > 1 {
        let mut flag = true;
        for i in 0..max { //for each pair of elements
            if list[i] < list[i + 1] {
                //swap the elements if they aren't in order
                list.swap(i, i + 1);
                flag = false;
            }
        }
        if flag {
            break;
        }
        max -= 1;
    }
}