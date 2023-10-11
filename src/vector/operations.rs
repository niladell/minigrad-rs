// Check for https://stackoverflow.com/a/72804785
// TODO: Change this to any data type

/// Computes the doc product of two vectors
/// 
/// # Example
/// ```
/// use homeydl::vector::operations::vector_dot;
///
/// let v1 = vec![1.0, 2.0, 3.0];
/// let v2 = vec![4.0, 5.0, 6.0];
/// 
/// let v3 = vector_dot(&v1, &v2);
/// 
/// assert_eq!(v3, 32.0);
/// ```
pub fn vector_dot(v1: &[f32], v2: &[f32]) -> f32 {
    let vec_len: usize = v1.len();
    assert_eq!(vec_len, v2.len());
    let mut out: f32 = 0.0;

    for i in 0..vec_len {
        out += v1[i] * v2[i];
    }
    out
}


/// Computes the elementwise multiplication of two vectors
/// 
/// # Example
/// ```
/// use homeydl::vector::operations::vector_multiplication_elementwise;
/// 
/// let v1 = vec![1.0, 2.0, 3.0];
/// let v2 = vec![4.0, 5.0, 6.0];
/// 
/// let v3 = vector_multiplication_elementwise(&v1, &v2);
/// 
/// assert_eq!(v3, vec![4.0, 10.0, 18.0]);
/// ```
pub fn vector_multiplication_elementwise(v1: &[f32], v2: &[f32]) -> Vec<f32> {
    let vec_len: usize = v1.len();
    assert_eq!(vec_len, v2.len());
    let mut out: Vec<f32> = vec![0.0; vec_len];

    for i in 0..vec_len {
        out[i] = v1[i] * v2[i];
    }
    out
}
