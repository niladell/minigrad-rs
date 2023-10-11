use pyo3::prelude::*;

use crate::matrix::Matrix as rs_Matrix;
use crate::matrix::operations::{
    matrix_sum as rs_matrix_sum,
    matrix_dot as rs_matrix_dot};

#[pyclass]
pub struct Matrix {
    matrix: rs_Matrix,
}

#[pymethods]
impl Matrix {
    #[new]
    pub fn new(rows: usize, cols: usize, data: Vec<Vec<f32>>) -> Self {
        Matrix {
            matrix: rs_Matrix::new(rows, cols, data),
        }
    }

    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.matrix.get(row, col)
    }

    pub fn row(&self, row: usize) -> Vec<f32> {
        self.matrix.row(row).to_vec()
    }

    pub fn column(&self, column: usize) -> Vec<f32> {
        self.matrix.column(column)
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.matrix.to_string())
    }

    fn __getitem__(&self, index: usize) -> PyResult<Vec<f32>> {
        Ok(self.matrix[index].to_vec())
    }

    // Specific Python FNS
    pub fn to_list(&self) -> Vec<Vec<f32>> {
        let mut result = Vec::with_capacity(self.matrix.rows);
        for i in 0..self.matrix.rows {
            result.push(self.matrix.row(i).to_vec());
        }
        result
    }
}

#[pyfunction]
pub fn matrix_sum(matrix_1: &Matrix, matrix_2: &Matrix) -> PyResult<Matrix> {
    let matrix = rs_matrix_sum(&matrix_1.matrix, &matrix_2.matrix).unwrap();
    Ok(Matrix { matrix })
}

#[pyfunction]
pub fn matrix_dot(matrix_1: &Matrix, matrix_2: &Matrix) -> PyResult<Matrix> {
    let matrix = rs_matrix_dot(&matrix_1.matrix, &matrix_2.matrix).unwrap();
    Ok(Matrix { matrix })
}
