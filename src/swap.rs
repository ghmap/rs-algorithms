pub fn swap(array: &mut [i64], a: usize, b: usize) {
    let curr_value = array[a];
    array[a] = array[b];
    array[b] = curr_value;
}