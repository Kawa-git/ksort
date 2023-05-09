pub fn bubble_sort<T: Ord + Copy>(mut vec: Vec<T>) -> Option<Vec<T>> {
    // complexity O(n^2), stable

    if vec.len() < 1 {
        return None;
    }

    for i in 0..vec.len() {
        for j in 0..vec.len() {
            if vec[i] <= vec[j] {
                vec.swap(i, j);
            }
        }
    }
    return Some(vec);
}

pub fn selection_sort<T: Ord + Copy>(mut vec: Vec<T>) -> Option<Vec<T>> {
    // O(n^2), non stable

    if vec.len() < 1 {
        return None;
    }

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

pub fn insertion_sort<T: Ord + Copy>(mut vec: Vec<T>) -> Option<Vec<T>> {
    // O(n^2), stable

    if vec.len() < 1 {
        return None;
    }

    for i in 0..vec.len() {
        let mut j = i;
        while j > 0 && vec[j] < vec[j - 1] {
            vec.swap(j, j - 1);
            j = j - 1;
        }
    }
    return Some(vec);
}

/// please dont use this
pub fn bogo_sort<T: Ord + Copy>(mut vec: Vec<T>) -> (Option<Vec<T>>, usize) {
    // Ω(n)
    if vec.len() < 1 {
        return (None, 0);
    }

    fn randomize<T>(mut vec: Vec<T>) -> Vec<T> {
        use rand::Rng;
        let mut rnd = rand::thread_rng();
        for i in 0..vec.len() {
            let random_numer = rnd.gen_range(0..(vec.len() as i32)) as usize;
            if random_numer == i {
                continue;
            }
            vec.swap(i, random_numer);
        }
        return vec;
    }

    let mut is_ok = true;
    let mut steps = 0;
    loop {
        steps += 1;
        for i in 0..vec.len() - 1 {
            if vec[i] > vec[i + 1] {
                is_ok = false;
                break;
            }
        }
        if !is_ok {
            vec = randomize(vec);
            is_ok = true;
        } else {
            return (Some(vec), steps);
        }
    }
}

pub fn merge_sort<T: Ord + Copy>(mut vec: Vec<T>) -> Option<Vec<T>> {
    pub fn sort<T: Ord + Copy>(mut vec: Vec<T>, left: usize, right: usize) -> Option<Vec<T>> {
        // θ(nlog(n)), stable
        if vec.len() < 1 {
            return None;
        }

        if right - 1 > left {
            let mid = left + (right - left) / 2;
            vec = sort(vec, left, mid).unwrap();
            vec = sort(vec, mid, right).unwrap();
            vec = merge(vec, left, mid, right);
            return Some(vec);
        } else {
            Some(vec)
        }
    }

    fn merge<T: Ord + Copy>(mut vec: Vec<T>, left: usize, mid: usize, right: usize) -> Vec<T> {
        let n1 = mid - left;
        let n2 = right - mid;
        let l1 = vec.clone();
        let r1 = vec.clone();
        let l = &l1[left..mid];
        let r = &r1[mid..right];

        let mut i = 0;
        let mut j = 0;
        let mut k = left;
        while i < n1 && j < n2 {
            if l[i] < r[j] {
                vec[k] = l[i];
                i = i + 1;
            } else {
                vec[k] = r[j];
                j = j + 1;
            }
            k = k + 1;
        }
        while i < n1 {
            vec[k] = l[i];
            i = i + 1;
            k = k + 1;
        }
        while j < n2 {
            vec[k] = r[j];
            j = j + 1;
            k = k + 1;
        }
        return vec;
    }
    let n = vec.len();
    return sort(vec, 0, n);
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

    #[test]
    #[ignore]
    fn bogo_sort_test() {
        let (result, steps) = bogo_sort(vec![3, 4, 1, 5, 2, 6, 8, 7, 9, 10]);
        assert_eq!(steps, 1);
        assert_eq!(result, Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
    }

    #[test]
    #[ignore]
    fn bogo_sort_empty_vec() {
        let vec: Vec<i32> = Vec::new();
        let (result, steps) = bogo_sort(vec);
        assert_eq!(result, None);
        assert_eq!(steps, 0);
    }

    #[test]
    fn merge_sort_test() {
        let vec = vec![3, 4, 1, 5, 2, 6, 8, 7, 9, 10];
        let result = merge_sort(vec.clone(), 0, vec.len());
        assert_eq!(result, Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
    }

    #[test]
    fn merge_sort_empty_vec() {
        let vec: Vec<i32> = Vec::new();
        let result = merge_sort(vec.clone(), 0, vec.len());
        assert_eq!(result, None);
    }
}
