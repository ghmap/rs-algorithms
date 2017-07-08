
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_work_with_sorted_collection () {
        let mut a = vec![1, 5, 6, 8];
        sort(&mut a);
        assert_eq!(a, vec![1, 5, 6, 8]);
    }

    #[test]
    fn should_work_with_unsorted_collection() {
        let mut a = vec![8, 6, 4, 1];
        sort(&mut a);
        assert_eq!(a, vec![1, 4, 6, 8]);
    }

    #[test]
    fn should_work_with_repeated_values() {
        let mut a = vec![5, 5, 4, 1];
        sort(&mut a);
        assert_eq!(a, vec![1, 4, 5, 5]);
    }

    #[test]
    fn should_work_with_negative_values() {
        let mut a = vec![7, 5, -44, 1];
        sort(&mut a);
        assert_eq!(a, vec![-44, 1, 5, 7]);
    }
}