use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

pub mod matrix;
pub mod vector;

pub mod matrix_bindings;
pub mod vector_bindings;

#[pymodule]
fn homeydl(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<matrix_bindings::Matrix>()?;

    m.add_function(wrap_pyfunction!(matrix_bindings::matrix_sum, m)?)?;
    m.add_function(wrap_pyfunction!(matrix_bindings::matrix_dot, m)?)?;

    // If you have other classes or functions to add, do it here.
    // For example, if you had a function named `vector_dot` in the `vector_bindings` module:
    m.add_function(wrap_pyfunction!(vector_bindings::vector_dot, m)?)?;
    m.add_function(wrap_pyfunction!(vector_bindings::vector_multiplication_elementwise, m)?)?;
    Ok(())
    }

// If you want to expose individual functions to Python, you can define them here.
// For example:
// #[pyfunction]
// fn some_function() -> PyResult<()> {
//     // ... function implementation ...
// }

// Then, in the `homeydl` function, you can add:
// m.add_function(wrap_pyfunction!(some_function, m)?)?;



// /// Formats the sum of two numbers as string.
// #[pyfunction]
// fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//     Ok((a + b).to_string())
// }

// /// A Python module implemented in Rust.
// #[pymodule]
// fn test(_py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
//     Ok(())    // }

