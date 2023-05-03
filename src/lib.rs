pub fn bubble_sort<T: Ord + Copy>(vec: Vec<T>) -> Vec<T> {
    // complexity O(n^2)

    if vec.len() < 2 {
        return Vec::new(); // this should return None
    }

    let mut vec = vec;
    for i in 0..vec.len() {
        for j in 0..vec.len() {
            if vec[i] <= vec[j] {
                let (el1, el2) = swap(vec[i], vec[j]);
                vec[i] = el1;
                vec[j] = el2;
            }
        }
    }
    return vec;
}

pub fn selection_sort<T: Ord + Copy>(vec: Vec<T>) -> Vec<T> {
    // non adaptive, O(n^2)

    if vec.len() < 2 {
        return Vec::new(); // this should return None
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
    return result;
}
#[inline]
fn swap<T>(el1: T, el2: T) -> (T, T) {
    return (el2, el1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_sort_test() {
        let result = bubble_sort(vec![3, 4, 1, 5, 2]);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn bubble_sort_empty_vec() {
        let vec = vec![0];
        let result: Vec<_> = bubble_sort(vec);
        assert_eq!(result, Vec::new());
    }
    #[test]
    fn selection_sort_test() {
        let result = selection_sort(vec![3, 4, 1, 5, 2]);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn selection_sort_empty_vec() {
        let vec = vec![0];
        let result: Vec<_> = selection_sort(vec);
        assert_eq!(result, Vec::new());
    }
}
