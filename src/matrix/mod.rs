pub mod operations;
pub use operations::{matrix_sum, matrix_dot};

use std::ops::Index;
use std::fmt;

pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    data: Box<[f32]>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize, data: Vec<Vec<f32>>) -> Self {
        assert_eq!(rows, data.len());
        assert_eq!(cols, data[0].len());
        let linear_data: Box<[f32]> = data.into_iter().flatten().collect();
        Matrix { rows, cols, data: linear_data}
    }

    pub fn zeros(rows: usize, cols: usize) -> Self {
        let data = vec![0.0; rows * cols];
        Matrix { rows, cols, data: data.into_boxed_slice() }
    }

    pub fn ones(rows: usize, cols: usize) -> Self {
        let data = vec![1.0; rows * cols];
        Matrix { rows, cols, data: data.into_boxed_slice() }
    }

    pub fn row(&self, row: usize) -> &[f32] {
        // Rows are just slices
        &self.data[row * self.cols..(1+row) * self.cols]
    }

    pub fn column(&self, column: usize) -> Vec<f32> {
        // Columns are composed into Vecs
        //(is there a more optimal way of doing it?)
        let mut out = vec![0.0; self.rows];
        for i in 0..self.rows {
            out[i] = self.data[column + i * self.cols]
        }
        out
    }

    pub fn get(&self, row: usize, col: usize) -> f32 {
        assert!(row < self.rows && col < self.cols, "Index out of bounds");
        self.data[row * self.cols + col]
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        for i in 0..self.rows {
            out.push_str(format!("{:?}\n", self.row(i)).as_str());
        }
        out.push_str(format!("Size: ({}x{})", self.rows, self.cols).as_str());
        write!(f, "{}", out)
    }
}

impl Index<usize> for Matrix {
    type Output = [f32];

    fn index(&self, row: usize) -> &Self::Output {
        &self.row(row)
    }
}
