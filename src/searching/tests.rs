#[cfg(test)]
mod tests {
    use crate::searching::search_algos::{
        search_key, search_max, search_max_subarray, search_sorted_key,
    };

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

    // --- Nuovi test per search_max_subarray ---
    mod search_max_subarray_tests {
        use super::*;

        #[test]
        fn test_single_element() {
            let v = vec![5];
            // Deve restituire (indice_inizio, indice_fine, somma)
            assert_eq!(search_max_subarray(&v, 0, 0), (0, 0, 5));
        }

        #[test]
        fn test_all_positive() {
            let v = vec![1, 2, 3, 4];
            // Se sono tutti positivi, il max subarray è l'intero array
            assert_eq!(search_max_subarray(&v, 0, 3), (0, 3, 10));
        }

        #[test]
        fn test_all_negative() {
            let v = vec![-5, -2, -9];
            // Se sono tutti negativi, il max subarray è il singolo elemento meno negativo
            assert_eq!(search_max_subarray(&v, 0, 2), (1, 1, -2));
        }

        #[test]
        fn test_mixed_classic_case() {
            // Esempio classico (Kadane's algorithm test)
            // L'array massimo contiguo è [4, -1, 2, 1] che somma a 6, da indice 3 a indice 6
            let v = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
            assert_eq!(search_max_subarray(&v, 0, v.len() - 1), (3, 6, 6));
        }

        #[test]
        fn test_mixed_with_large_numbers() {
            let v = vec![
                13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7,
            ];
            // Array massimo: [18, 20, -7, 12] -> somma 43, da indice 7 a 10
            assert_eq!(search_max_subarray(&v, 0, v.len() - 1), (7, 10, 43));
        }

        #[test]
        fn test_partial_range() {
            let v = vec![10, -5, 20, -50, 30, 20];
            // Se cerchiamo solo nei primi 3 elementi (indici 0..=2), il massimo è [10, -5, 20] = 25
            assert_eq!(search_max_subarray(&v, 0, 2), (0, 2, 25));
            // Se cerchiamo nell'intero array, il massimo è [30, 20] = 50, da indice 4 a 5
            assert_eq!(search_max_subarray(&v, 0, 5), (4, 5, 50));
        }
    }
}
