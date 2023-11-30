/// This QuickSort implementation is generic as long as the datatype implements certain traits
/// We assume the pivot is the right most element
///
pub fn quick_sort<T>(mut input: Vec<T>) -> Vec<T>
where
    T: Eq + std::cmp::PartialOrd + Copy,
{
    let input_len = input.len();
    quick_sort_recursive(&mut input);
    input
}

pub fn quick_sort_recursive<T: Eq + std::cmp::PartialOrd + Copy>(input: &mut [T])
where
    T: Eq + std::cmp::PartialOrd + Copy,
{
    let slice_len = input.len();

    if slice_len > 0 {
        let partition_index = partition(input);
        quick_sort_recursive(&mut input[0..partition_index]);
        quick_sort_recursive(&mut input[partition_index + 1..slice_len]);
    }
}

pub fn partition<T: Eq + std::cmp::PartialOrd + Copy>(input: &mut [T]) -> usize
where
    T: Eq + std::cmp::PartialOrd + Copy,
{
    // We already checked slice is not empty
    let pivot_value = input.last().copied().unwrap();
    let mut temp_pivot_index = 0;
    let slice_len = input.len();

    for index in 0..slice_len {
        if input[index] < pivot_value {
            input.swap(temp_pivot_index, index);
            temp_pivot_index += 1;
        }
    }

    // Swap the pivot into right position
    if input[temp_pivot_index] > pivot_value {
        input.swap(temp_pivot_index, slice_len - 1);
    }

    temp_pivot_index
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    #[test]
    fn quick_sort_u32() {
        assert_eq!(quick_sort(vec![100, 21, 213, 4]), vec![4, 21, 100, 213]);
    }

    #[test]
    fn quick_sort_negatives() {
        assert_eq!(quick_sort(vec![-100, 21, -213, 4]), vec![-213, -100, 4, 21]);
    }

    #[test]
    fn quick_sort_already_sorted() {
        assert_eq!(quick_sort(vec![1, 2, 3, 4]), vec![1, 2, 3, 4]);
    }

    #[test]
    fn quick_sort_large() {
        let result: Vec<i32> = (0..10000).collect();
        let mut rng = thread_rng();
        let mut input = result.clone();
        input.shuffle(&mut rng);

        assert_eq!(quick_sort(input), result);
    }
}
