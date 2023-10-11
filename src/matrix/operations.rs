use super::Matrix;
use crate::vector::operations::vector_dot;

// Change return to &'static str
pub fn matrix_sum(matrix_1: &Matrix, matrix_2: &Matrix) -> Result<Matrix, String> {
    if matrix_1.rows != matrix_2.rows || matrix_1.cols != matrix_2.cols {
        return Err(format!(
            "Matrix sizes [{}, {}] and [{}, {}] do not match",
            matrix_1.rows, matrix_1.cols, matrix_2.rows, matrix_2.cols
        ));
    }

    let rows: usize = matrix_1.rows;
    let cols: usize = matrix_1.cols;

    let mut data: Vec<Vec<f32>> = vec![vec![0.0; cols]; rows];

    for i in 0..rows {
        for j in 0..cols {
            data[i][j] = matrix_1.get(i, j) + matrix_2.get(i, j);
        }
    }

    return Ok(Matrix::new(rows, cols, data));
}

pub fn matrix_dot(matrix_1: &Matrix, matrix_2: &Matrix) -> Result<Matrix, String> {
    let new_rows: usize = matrix_1.rows;
    let new_cols: usize = matrix_2.cols;

    if matrix_1.cols != matrix_2.rows {
        return Err(format!("Matrix sizes [{}, {}] and [{}, {}] do not match dot product shape",
                matrix_1.rows, matrix_1.cols, matrix_2.rows, matrix_2.cols));
    }

    let mut data: Vec<Vec<f32>> = vec![vec![0.0; new_cols]; new_rows];

    for i in 0..new_rows{
        for j in 0..new_cols{
            data[i][j] = vector_dot(matrix_1.row(i), &matrix_2.column(j));
        }
    }

    return Ok(Matrix::new(new_rows, new_cols, data));
}
