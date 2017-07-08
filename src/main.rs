fn main() {
    println!("Hello, world!");

    let mut a = vec![8, 6, 4, 1];
    sort(&mut a);

    println!("{:?}", a);
}

pub fn sort(array: &mut [i64]) {
    for i in 0..array.len() {
        let mut min_found = i;

        for j in i..array.len() {
            if array[j] < array[min_found] {
                min_found = j;
            }
        }

        if array[min_found] < array[i] {
            let curr_value = array[i];
            array[i] = array[min_found];
            array[min_found] = curr_value;
        }
    }
}

