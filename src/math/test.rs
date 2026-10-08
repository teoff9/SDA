#[cfg(test)]
mod tests {
    use crate::math::matrix::Matrix;

    #[test]
    fn test_new_const() {
        let m = Matrix::new_const(2, 3, 7);
        assert_eq!(m.rows(), 2);
        assert_eq!(m.cols(), 3);
        assert_eq!(m[0][0], 7);
        assert_eq!(m[1][2], 7);
    }

    #[test]
    fn test_new_from() {
        let vec_vec = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let m = Matrix::new_from(vec_vec);
        assert_eq!(m.rows(), 2);
        assert_eq!(m.cols(), 3);

        assert_eq!(m[0][0], 1);
        assert_eq!(m[0][2], 3);
        assert_eq!(m[1][0], 4);
        assert_eq!(m[1][2], 6);
    }

    #[test]
    fn test_index_mut() {
        let mut m = Matrix::new_const(2, 2, 0);
        m[0][1] = 42;
        m[1][0] = 99;

        assert_eq!(m[0][0], 0);
        assert_eq!(m[0][1], 42);
        assert_eq!(m[1][0], 99);
        assert_eq!(m[1][1], 0);
    }

    #[test]
    fn test_add() {
        let a = Matrix::new_from(vec![vec![1, 2], vec![3, 4]]);
        let b = Matrix::new_from(vec![vec![5, 6], vec![7, 8]]);
        let atteso = Matrix::new_from(vec![vec![6, 8], vec![10, 12]]);

        assert_eq!(&a + &b, atteso);
    }

    #[test]
    #[should_panic]
    fn test_add_panic() {
        let a = Matrix::new_const(2, 3, 1);
        let b = Matrix::new_const(3, 2, 1);
        let _ = &a + &b;
    }

    #[test]
    fn test_sub() {
        let a = Matrix::new_from(vec![vec![5, 6], vec![7, 8]]);
        let b = Matrix::new_from(vec![vec![1, 2], vec![3, 4]]);
        let atteso = Matrix::new_from(vec![vec![4, 4], vec![4, 4]]);

        assert_eq!(&a - &b, atteso);
    }

    #[test]
    #[should_panic]
    fn test_sub_panic() {
        let a = Matrix::new_const(2, 3, 1);
        let b = Matrix::new_const(3, 2, 1);
        let _ = &a - &b;
    }

    #[test]
    fn test_scalar_mul() {
        let a = Matrix::new_from(vec![vec![1, 2], vec![3, 4]]);
        let atteso = Matrix::new_from(vec![vec![3, 6], vec![9, 12]]);

        assert_eq!(&a * 3, atteso);
    }

    macro_rules! test_multiplication_algo {
        ($nome_modulo:ident, $funzione_moltiplicazione:path) => {
            mod $nome_modulo {
                use super::*;

                #[test]
                fn test_multiply_2x2() {
                    let a = Matrix::new_from(vec![vec![1, 2], vec![3, 4]]);
                    let b = Matrix::new_from(vec![vec![2, 0], vec![1, 2]]);
                    let atteso = Matrix::new_from(vec![vec![4, 4], vec![10, 8]]);

                    assert_eq!($funzione_moltiplicazione(&a, &b), atteso);
                }

                #[test]
                fn test_multiply_identity() {
                    let a = Matrix::new_from(vec![vec![5, 8], vec![2, 1]]);
                    let identita = Matrix::new_from(vec![vec![1, 0], vec![0, 1]]);

                    assert_eq!($funzione_moltiplicazione(&a, &identita), a);
                }

                #[test]
                fn test_multiply_zero() {
                    let a = Matrix::new_from(vec![vec![1, 2], vec![3, 4]]);
                    let zero = Matrix::new_const(2, 2, 0);

                    assert_eq!($funzione_moltiplicazione(&a, &zero), zero);
                }

                #[test]
                fn test_multiply_4x4() {
                    let a = Matrix::new_from(vec![
                        vec![1, 0, 2, 1],
                        vec![0, 3, 1, 1],
                        vec![2, 1, 0, 2],
                        vec![1, 1, 1, 1],
                    ]);
                    let b = Matrix::new_from(vec![
                        vec![2, 1, 0, 1],
                        vec![1, 2, 1, 0],
                        vec![0, 1, 2, 1],
                        vec![1, 0, 1, 2],
                    ]);
                    let atteso = Matrix::new_from(vec![
                        vec![3, 3, 5, 5],
                        vec![4, 7, 6, 3],
                        vec![7, 4, 3, 6],
                        vec![4, 4, 4, 4],
                    ]);

                    assert_eq!($funzione_moltiplicazione(&a, &b), atteso);
                }
            }
        };
    }

    // use crate::matrix::multiply::{brute_multiply, strassen_multiply};
    // test_multiplication_algo!(brute_multiplication_tests, brute_multiply);
    // test_multiplication_algo!(strassen_multiplication_tests, strassen_multiply);
}
