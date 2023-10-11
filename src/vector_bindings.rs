use pyo3::prelude::*;

use crate::vector::operations::{
    vector_dot as rs_vector_dot,
    vector_multiplication_elementwise as rs_vector_multiplication_elementwise
};

#[pyfunction]
pub fn vector_dot(v1: Vec<f32>, v2: Vec<f32>) -> f32 {
    rs_vector_dot(&v1, &v2)
}

#[pyfunction]
pub fn vector_multiplication_elementwise(v1: Vec<f32>, v2: Vec<f32>) -> Vec<f32> {
    rs_vector_multiplication_elementwise(&v1, &v2)
}
