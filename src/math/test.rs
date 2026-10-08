#[cfg(test)]
mod tests {
    use crate::math::matrix::{Matrix, brute_multiply, strassen_multiply};

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
        let mut m = Matrix::new_const(2, 3, 0);
        m[0][1] = 42;
        m[1][2] = 99;

        assert_eq!(m[0][0], 0);
        assert_eq!(m[0][1], 42);
        assert_eq!(m[1][2], 99);
        assert_eq!(m[1][1], 0);
    }

    #[test]
    fn test_add() {
        let a = Matrix::new_from(vec![vec![1, 2, 3], vec![4, 5, 6]]);
        let b = Matrix::new_from(vec![vec![7, 8, 9], vec![10, 11, 12]]);
        let atteso = Matrix::new_from(vec![vec![8, 10, 12], vec![14, 16, 18]]);

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
        let a = Matrix::new_from(vec![vec![7, 8, 9], vec![10, 11, 12]]);
        let b = Matrix::new_from(vec![vec![1, 2, 3], vec![4, 5, 6]]);
        let atteso = Matrix::new_from(vec![vec![6, 6, 6], vec![6, 6, 6]]);

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
        let a = Matrix::new_from(vec![vec![1, 2, 3], vec![4, 5, 6]]);
        let atteso = Matrix::new_from(vec![vec![3, 6, 9], vec![12, 15, 18]]);

        assert_eq!(&a * 3, atteso);
    }

    #[test]
    fn test_operator_mul() {
        let a = Matrix::new_const(30, 40, 2);
        let b = Matrix::new_const(40, 20, 3);
        let atteso = Matrix::new_const(30, 20, 240);

        assert_eq!(&a * &b, atteso);
        assert_eq!(a * b, atteso);
    }

    macro_rules! test_multiplication_algo {
        ($nome_modulo:ident, $funzione_moltiplicazione:path) => {
            mod $nome_modulo {
                use super::*;

                #[test]
                fn test_multiply_square() {
                    let a = Matrix::new_from(vec![vec![1, 2], vec![3, 4]]);
                    let b = Matrix::new_from(vec![vec![2, 0], vec![1, 2]]);
                    let atteso = Matrix::new_from(vec![vec![4, 4], vec![10, 8]]);

                    assert_eq!($funzione_moltiplicazione(&a, &b), atteso);
                }

                #[test]
                fn test_multiply_non_square() {
                    let a = Matrix::new_from(vec![vec![1, 2, 3], vec![4, 5, 6]]);
                    let b = Matrix::new_from(vec![vec![7, 8], vec![9, 10], vec![11, 12]]);
                    let atteso = Matrix::new_from(vec![vec![58, 64], vec![139, 154]]);

                    assert_eq!($funzione_moltiplicazione(&a, &b), atteso);
                }

                #[test]
                fn test_multiply_vector() {
                    let a = Matrix::new_from(vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
                    let b = Matrix::new_from(vec![vec![7], vec![8]]);
                    let atteso = Matrix::new_from(vec![vec![23], vec![53], vec![83]]);

                    assert_eq!($funzione_moltiplicazione(&a, &b), atteso);
                }

                #[test]
                fn test_multiply_identity() {
                    let a = Matrix::new_from(vec![vec![5, 8, 3], vec![2, 1, 9]]);
                    let identita =
                        Matrix::new_from(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]);

                    assert_eq!($funzione_moltiplicazione(&a, &identita), a);
                }

                #[test]
                fn test_multiply_zero() {
                    let a = Matrix::new_from(vec![vec![1, 2, 3], vec![4, 5, 6]]);
                    let zero = Matrix::new_const(3, 4, 0);
                    let atteso = Matrix::new_const(2, 4, 0);

                    assert_eq!($funzione_moltiplicazione(&a, &zero), atteso);
                }

                #[test]
                fn test_multiply_large_square() {
                    let a = Matrix::new_const(40, 40, 2);
                    let b = Matrix::new_const(40, 40, 3);
                    let atteso = Matrix::new_const(40, 40, 240);

                    assert_eq!($funzione_moltiplicazione(&a, &b), atteso);
                }

                #[test]
                fn test_multiply_large_non_square() {
                    let a = Matrix::new_const(25, 40, 2);
                    let b = Matrix::new_const(40, 30, 3);
                    let atteso = Matrix::new_const(25, 30, 240);

                    assert_eq!($funzione_moltiplicazione(&a, &b), atteso);
                }

                #[test]
                fn test_multiply_100x100_non_trivial() {
                    let mut vec_a = vec![vec![0; 100]; 100];
                    let mut vec_b = vec![vec![0; 100]; 100];
                    let mut vec_atteso = vec![vec![0; 100]; 100];

                    for i in 0..100 {
                        for j in 0..100 {
                            vec_a[i][j] = i as i32;
                            vec_b[i][j] = j as i32;
                            vec_atteso[i][j] = (i as i32) * (j as i32) * 100;
                        }
                    }

                    let a = Matrix::new_from(vec_a);
                    let b = Matrix::new_from(vec_b);
                    let atteso = Matrix::new_from(vec_atteso);

                    assert_eq!($funzione_moltiplicazione(&a, &b), atteso);
                }

                #[test]
                #[should_panic]
                fn test_multiply_panic() {
                    let a = Matrix::new_const(2, 3, 1);
                    let b = Matrix::new_const(4, 2, 1);
                    let _ = $funzione_moltiplicazione(&a, &b);
                }
            }
        };
    }

    test_multiplication_algo!(brute_multiplication_tests, brute_multiply);
    test_multiplication_algo!(strassen_multiplication_tests, strassen_multiply);
}
