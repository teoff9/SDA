use num::Num;
use std::{
    cmp::max,
    fmt,
    ops::{Add, Index, IndexMut, Mul, Sub},
};

#[derive(Clone)]
pub struct Matrix<T: Num + Clone + Copy> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: Num + Clone + Copy> Matrix<T> {
    pub fn new_const(rows: usize, cols: usize, value: T) -> Matrix<T> {
        Matrix {
            data: vec![value; rows * cols],
            rows,
            cols,
        }
    }

    //array of rows
    pub fn new_from(vec_vec: Vec<Vec<T>>) -> Matrix<T> {
        assert!(!vec_vec.is_empty() && vec_vec.iter().all(|r| r.len() == vec_vec[0].len()));
        let rows = vec_vec.len();
        let cols = vec_vec[0].len();
        let mut data = vec![T::zero(); cols * rows];
        (0..rows).into_iter().for_each(|r| {
            (0..cols)
                .into_iter()
                .for_each(|c| data[r * cols + c] = vec_vec[r][c])
        });

        Matrix { data, rows, cols }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn transpose(&mut self) {
        assert!(self.cols() == self.rows());
        let n = self.cols();
        for i in 0..n {
            for j in i + 1..n {
                self.data.swap(i * n + j, j * n + i);
            }
        }
    }

    pub fn pad_to_squared(&self, size: usize) -> Matrix<T> {
        assert!(self.cols() <= size && self.rows() <= size);
        let mut m = Matrix::new_const(size, size, T::zero());
        for i in 0..self.rows() {
            m[i][0..self.cols()].copy_from_slice(&self[i]);
        }
        m
    }
    pub fn crop(&self, rows: usize, cols: usize) -> Matrix<T> {
        assert!(rows <= self.rows() && cols <= self.cols());
        let mut m = Matrix::new_const(rows, cols, T::zero());
        for i in 0..rows {
            m[i][0..cols].copy_from_slice(&self[i][0..cols]);
        }
        m
    }

    pub fn split_quadrants(&self) -> (Matrix<T>, Matrix<T>, Matrix<T>, Matrix<T>) {
        assert!(self.cols() == self.rows() && self.cols() % 4 == 0);

        let s = self.cols() / 2;

        let mut m: [Matrix<T>; 4] = std::array::from_fn(|_| Matrix::new_const(s, s, T::zero()));

        for i in 0..4 {
            for j in 0..s {
                let r = s * (i / 2);
                let c = s * (i % 2);
                m[i][j].copy_from_slice(&self[r + j][c..c + s]);
            }
        }

        let [m0, m1, m2, m3] = m;
        (m0, m1, m2, m3)
    }

    pub fn merge_quadrants(
        c11: &Matrix<T>,
        c12: &Matrix<T>,
        c21: &Matrix<T>,
        c22: &Matrix<T>,
    ) -> Matrix<T> {
        assert!(
            c11.rows() == c11.cols()
                && c11.rows() == c12.rows()
                && c11.cols() == c12.cols()
                && c11.rows() == c21.rows()
                && c11.cols() == c21.cols()
                && c11.rows() == c22.rows()
                && c11.cols() == c22.cols()
        );

        let s = c11.rows();
        let mut m = Matrix::new_const(s * 2, s * 2, T::zero());

        for i in 0..s {
            m[i][0..s].copy_from_slice(&c11[i]);
            m[i][s..s * 2].copy_from_slice(&c12[i]);
            m[i + s][0..s].copy_from_slice(&c21[i]);
            m[i + s][s..s * 2].copy_from_slice(&c22[i]);
        }

        m
    }
}

impl<T: Num + Clone + Copy + fmt::Debug> fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.cols == 0 {
            return f.debug_list().finish();
        }
        f.debug_list().entries(self.data.chunks(self.cols)).finish()
    }
}

impl<T: Num + Clone + Copy> Index<usize> for Matrix<T> {
    type Output = [T];
    fn index(&self, idx: usize) -> &Self::Output {
        let i = idx * self.cols();
        let f = i + self.cols();
        &self.data[i..f]
    }
}

impl<T: Num + Clone + Copy> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        let i = idx * self.cols();
        let f = i + self.cols();
        &mut self.data[i..f]
    }
}

impl<T: Num + Clone + Copy> Add for &Matrix<T> {
    type Output = Matrix<T>;
    fn add(self, rhs: Self) -> Self::Output {
        assert!(self.rows == rhs.rows && self.cols == rhs.cols);
        let data = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(&a, &b)| a + b)
            .collect();
        Matrix {
            data,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl<T: Num + Clone + Copy> Add for Matrix<T> {
    type Output = Matrix<T>;
    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl<T: Num + Clone + Copy> Sub for &Matrix<T> {
    type Output = Matrix<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        assert!(self.rows == rhs.rows && self.cols == rhs.cols);
        let data = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(&a, &b)| a - b)
            .collect();
        Matrix {
            data,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl<T: Num + Clone + Copy> Sub for Matrix<T> {
    type Output = Matrix<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

impl<T: Num + Clone + Copy> Mul<T> for &Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, rhs: T) -> Self::Output {
        let data = self.data.iter().map(|&a| a * rhs).collect();
        Matrix {
            data,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl<T: Num + Clone + Copy> Mul<T> for Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, rhs: T) -> Self::Output {
        &self * rhs
    }
}

impl<T: Num + Clone + Copy> PartialEq for Matrix<T> {
    fn eq(&self, other: &Self) -> bool {
        self.rows == other.rows && self.cols == other.cols && self.data == other.data
    }
}

impl<T: Num + Clone + Copy> Mul<&Matrix<T>> for &Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, rhs: &Matrix<T>) -> Self::Output {
        strassen_multiply(self, rhs)
    }
}

impl<T: Num + Clone + Copy> Mul<Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        strassen_multiply(&self, &rhs)
    }
}

pub fn brute_multiply<T: Num + Clone + Copy>(a: &Matrix<T>, b: &Matrix<T>) -> Matrix<T> {
    assert!(a.cols() == b.rows());
    let mut c = Matrix::new_const(a.rows(), b.cols(), T::zero());

    for i in 0..a.rows() {
        for j in 0..b.cols() {
            c[i][j] = T::zero();
            for k in 0..a.cols() {
                c[i][j] = c[i][j] + a[i][k] * b[k][j];
            }
        }
    }
    c
}

pub fn strassen_multiply<T: Num + Clone + Copy>(a: &Matrix<T>, b: &Matrix<T>) -> Matrix<T> {
    assert!(a.cols() == b.rows());

    let (ar, ac) = (a.rows(), a.cols());
    let (br, bc) = (b.rows(), b.cols());
    let s = (max(a.rows(), max(a.cols(), b.cols()))).next_power_of_two();
    let pad_a = ar != s || ac != s;
    let pad_b = br != s || bc != s;
    let temp_a;
    let temp_b;

    let m1 = if pad_a {
        temp_a = a.pad_to_squared(s);
        &temp_a
    } else {
        a
    };

    let m2 = if pad_b {
        temp_b = b.pad_to_squared(s);
        &temp_b
    } else {
        b
    };

    let res = strassen_recursive(m1, m2);

    if pad_a || pad_b {
        res.crop(ar, bc)
    } else {
        res
    }
}

pub fn strassen_recursive<T: Num + Clone + Copy>(m1: &Matrix<T>, m2: &Matrix<T>) -> Matrix<T> {
    if m1.rows() <= 16 {
        brute_multiply(m1, m2)
    } else {
        let (a, b, c, d) = m1.split_quadrants();
        let (e, f, g, h) = m2.split_quadrants();

        let p1 = strassen_recursive(&a, &(&f - &h));
        let p2 = strassen_recursive(&(&a + &b), &h);
        let p3 = strassen_recursive(&(&c + &d), &e);
        let p4 = strassen_recursive(&d, &(&g - &e));
        let p5 = strassen_recursive(&(&a + &d), &(&e + &h));
        let p6 = strassen_recursive(&(&b - &d), &(&g + &h));
        let p7 = strassen_recursive(&(&a - &c), &(&e + &f));

        let c11 = &(&(&p5 + &p4) - &p2) + &p6;
        let c12 = &p1 + &p2;
        let c21 = &p3 + &p4;
        let c22 = &(&(&p1 + &p5) - &p3) - &p7;

        Matrix::merge_quadrants(&c11, &c12, &c21, &c22)
    }
}
