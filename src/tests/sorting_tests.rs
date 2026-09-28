#[cfg(test)]
mod tests {
    use crate::sorting::sort_algos::selection_sort;
    use crate::sorting::utils::find_min;

    #[test]
    fn test_find_min_whole_array() {
        let v = vec![5, 3, 8, 1, 2];
        assert_eq!(find_min(&v, 0, v.len() - 1), 3);
    }

    #[test]
    fn test_find_min_partial_array() {
        let v = vec![5, 3, 8, 4, 2];
        assert_eq!(find_min(&v, 0, 2), 1);
    }

    #[test]
    fn test_find_min_with_duplicates() {
        let v = vec![4, 2, 2, 3];
        assert_eq!(find_min(&v, 0, 3), 1);
    }

    #[test]
    fn test_selection_sort_empty() {
        let mut v: Vec<i32> = vec![];
        selection_sort(&mut v);
        assert_eq!(v, vec![]);
    }

    #[test]
    fn test_selection_sort_single_element() {
        let mut v = vec![42];
        selection_sort(&mut v);
        assert_eq!(v, vec![42]);
    }

    #[test]
    fn test_selection_sort_already_sorted() {
        let mut v = vec![1, 2, 3, 4, 5];
        selection_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_selection_sort_reversed() {
        let mut v = vec![5, 4, 3, 2, 1];
        selection_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_selection_sort_random_order() {
        let mut v = vec![7, 2, 9, 1, 5, 4];
        selection_sort(&mut v);
        assert_eq!(v, vec![1, 2, 4, 5, 7, 9]);
    }

    #[test]
    fn test_selection_sort_with_duplicates() {
        let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
        selection_sort(&mut v);
        assert_eq!(v, vec![1, 1, 2, 3, 3, 4, 5, 5, 6, 9]);
    }

    #[test]
    fn test_selection_sort_all_same_elements() {
        let mut v = vec![7, 7, 7, 7];
        selection_sort(&mut v);
        assert_eq!(v, vec![7, 7, 7, 7]);
    }

    #[test]
    fn test_selection_sort_strings() {
        let mut v = vec!["zucchina", "mela", "banana", "arancia"];
        selection_sort(&mut v);
        assert_eq!(v, vec!["arancia", "banana", "mela", "zucchina"]);
    }
}
