use num::Num;
use std::{
    fmt,
    ops::{Add, Index, IndexMut, Mul, Sub},
};

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

impl<T: Num + Clone + Copy> PartialEq for Matrix<T> {
    fn eq(&self, other: &Self) -> bool {
        self.rows == other.rows && self.cols == other.cols && self.data == other.data
    }
}
