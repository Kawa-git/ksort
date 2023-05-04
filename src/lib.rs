pub fn bubble_sort<T: Ord + Copy>(vec: Vec<T>) -> Option<Vec<T>> {
    // complexity O(n^2)

    if vec.len() < 1 {
        return None;
    }

    let mut vec = vec;
    for i in 0..vec.len() {
        for j in 0..vec.len() {
            if vec[i] <= vec[j] {
                vec.swap(i, j);
            }
        }
    }
    return Some(vec);
}

pub fn selection_sort<T: Ord + Copy>(vec: Vec<T>) -> Option<Vec<T>> {
    // non adaptive, O(n^2)

    if vec.len() < 1 {
        return None;
    }

    let mut vec = vec;
    let mut result: Vec<T> = Vec::new();

    // functional programming may make this more readable
    for _ in 0..vec.len() {
        let mut min = *vec.get(0).unwrap();
        let mut index = 0;

        // getting the minimum
        for i in 0..vec.len() {
            if vec[i] < min {
                min = vec[i];
                index = i;
            }
        }
        result.push(min);
        vec.remove(index);
    }
    return Some(result);
}

pub fn insertion_sort<T: Ord + Copy>(vec: Vec<T>) -> Option<Vec<T>> {
    // O(n^2)

    if vec.len() < 1 {
        return None;
    }

    let mut vec = vec;
    for i in 0..vec.len() {
        let mut j = i;
        while j > 0 && vec[j] < vec[j - 1] {
            vec.swap(j, j - 1);
            j = j - 1;
        }
    }
    return Some(vec);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_sort_test() {
        let result = bubble_sort(vec![3, 4, 1, 5, 2]);
        assert_eq!(result, Some(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn bubble_sort_empty_vec() {
        let vec: Vec<i32> = Vec::new();
        let result = bubble_sort(vec);
        assert_eq!(result, None);
    }

    #[test]
    fn selection_sort_test() {
        let result = selection_sort(vec![3, 4, 1, 5, 2]);
        assert_eq!(result, Some(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn selection_sort_empty_vec() {
        let vec: Vec<i32> = Vec::new();
        let result = selection_sort(vec);
        assert_eq!(result, None);
    }

    #[test]
    fn insertion_sort_test() {
        let result = insertion_sort(vec![3, 4, 1, 5, 2]);
        assert_eq!(result, Some(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn insertion_sort_empty_vec() {
        let vec: Vec<i32> = Vec::new();
        let result = insertion_sort(vec);
        assert_eq!(result, None);
    }
}
