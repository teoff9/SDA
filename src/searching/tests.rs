#[cfg(test)]
mod tests {
    use crate::searching::search_algos::{search_key, search_max, search_sorted_key};

    macro_rules! test_search_algo {
        ($nome_modulo:ident,$funzione_search:path) => {
            mod $nome_modulo {
                use super::*;

                #[test]
                fn test_empty_slice() {
                    let v: Vec<i32> = vec![];
                    assert_eq!($funzione_search(&v, 5), None);
                }

                #[test]
                fn test_key_at_first_position() {
                    let v = vec![10, 20, 30, 40, 50];
                    assert_eq!($funzione_search(&v, 10), Some(0));
                }

                #[test]
                fn test_key_in_the_middle() {
                    let v = vec![10, 20, 30, 40, 50];
                    assert_eq!($funzione_search(&v, 30), Some(2));
                }

                #[test]
                fn test_key_at_last_position() {
                    let v = vec![10, 20, 30, 40, 50];
                    assert_eq!($funzione_search(&v, 50), Some(4));
                }

                #[test]
                fn test_key_not_found_smaller() {
                    let v = vec![10, 20, 30, 40, 50];
                    assert_eq!($funzione_search(&v, 5), None);
                }

                #[test]
                fn test_key_not_found_larger() {
                    let v = vec![10, 20, 30, 40, 50];
                    assert_eq!($funzione_search(&v, 99), None);
                }

                #[test]
                fn test_with_strings() {
                    let v = vec!["arancia", "banana", "kiwi", "mela"];
                    assert_eq!($funzione_search(&v, "banana"), Some(1));
                    assert_eq!($funzione_search(&v, "pera"), None);
                }
            }
        };
    }

    test_search_algo!(linear_search_tests, search_key);
    test_search_algo!(binary_search_tests, search_sorted_key);

    mod search_max_tests {
        use super::*;

        #[test]
        fn test_empty_slice() {
            let v: Vec<i32> = vec![];
            assert_eq!(search_max(&v, 0, 0), None);
        }

        #[test]
        fn test_single_element() {
            let v = vec![42];
            assert_eq!(search_max(&v, 0, 0), Some(42));
        }

        #[test]
        fn test_max_at_beginning() {
            let v = vec![50, 20, 30, 10];
            assert_eq!(search_max(&v, 0, v.len() - 1), Some(50));
        }

        #[test]
        fn test_max_in_middle() {
            let v = vec![10, 20, 99, 40, 30];
            assert_eq!(search_max(&v, 0, v.len() - 1), Some(99));
        }

        #[test]
        fn test_max_at_end() {
            let v = vec![10, 20, 30, 40, 88];
            assert_eq!(search_max(&v, 0, v.len() - 1), Some(88));
        }

        #[test]
        fn test_partial_range() {
            let v = vec![100, 20, 50, 30, 200];
            assert_eq!(search_max(&v, 1, 3), Some(50));
        }

        #[test]
        fn test_invalid_range() {
            let v = vec![10, 20, 30];
            assert_eq!(search_max(&v, 2, 0), None);
            assert_eq!(search_max(&v, 0, 5), None);
        }

        #[test]
        fn test_with_strings() {
            let v = vec!["mela", "kiwi", "zucchina", "banana"];
            assert_eq!(search_max(&v, 0, v.len() - 1), Some("zucchina"));
        }
    }
}
