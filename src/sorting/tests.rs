#[cfg(test)]
mod tests {
    use crate::sorting::sort_algos::{insertion_sort, selection_sort};
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

    macro_rules! test_sorting_algo {
        ($nome_modulo:ident, $funzione_sort:path) => {
            mod $nome_modulo {
                use super::*;

                #[test]
                fn test_empty() {
                    let mut v: Vec<i32> = vec![];
                    $funzione_sort(&mut v);
                    assert_eq!(v, vec![]);
                }

                #[test]
                fn test_single_element() {
                    let mut v = vec![42];
                    $funzione_sort(&mut v);
                    assert_eq!(v, vec![42]);
                }

                #[test]
                fn test_already_sorted() {
                    let mut v = vec![1, 2, 3, 4, 5];
                    $funzione_sort(&mut v);
                    assert_eq!(v, vec![1, 2, 3, 4, 5]);
                }

                #[test]
                fn test_reversed() {
                    let mut v = vec![5, 4, 3, 2, 1];
                    $funzione_sort(&mut v);
                    assert_eq!(v, vec![1, 2, 3, 4, 5]);
                }

                #[test]
                fn test_random_order() {
                    let mut v = vec![7, 2, 9, 1, 5, 4];
                    $funzione_sort(&mut v);
                    assert_eq!(v, vec![1, 2, 4, 5, 7, 9]);
                }

                #[test]
                fn test_with_duplicates() {
                    let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
                    $funzione_sort(&mut v);
                    assert_eq!(v, vec![1, 1, 2, 3, 3, 4, 5, 5, 6, 9]);
                }

                #[test]
                fn test_all_same_elements() {
                    let mut v = vec![7, 7, 7, 7];
                    $funzione_sort(&mut v);
                    assert_eq!(v, vec![7, 7, 7, 7]);
                }

                #[test]
                fn test_strings() {
                    let mut v = vec!["zucchina", "mela", "banana", "arancia"];
                    $funzione_sort(&mut v);
                    assert_eq!(v, vec!["arancia", "banana", "mela", "zucchina"]);
                }
            }
        };
    }

    test_sorting_algo!(selection_sort_tests, selection_sort);

    test_sorting_algo!(insertion_sort_tests, insertion_sort);
}
