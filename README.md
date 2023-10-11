# Rust MiniGrad with Python Bindings

## Introduction

Rust MiniGrad is an experimental automatic differentiation library built in Rust with Python bindings. This is just a learning project.

**Status**: 🚧 Under Construction 🚧

## Examples

You can find detailed examples in the `examples` folder. Here's a quick look:

### Matrix Operations

#### In Rust

\```rust
use minigrad::matrix::{Matrix, matrix_sum, matrix_multiply};

let a = Matrix::new(2, 2, vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
let b = Matrix::new(2, 2, vec![vec![2.0, 3.0], vec![4.0, 5.0]]);

let sum = matrix_sum(&a, &b);
let product = matrix_multiply(&a, &b);

println!("{}", sum);
println!("{}", product);
\```

#### In Python
\```
import homeydl

matrix1 = homeydl.Matrix(2, 2, [[1.0, 2.0], [3.0, 4.0]])
matrix2 = homeydl.Matrix(2, 2, [[2.0, 0.0], [1.0, 3.0]])

sum = matrix1 + matrix2
product = matrix1 * matrix2

print(sum)
print(product)
\```


#### Automatic Differentiation

[ ] Yet to be implemented.


## Installation

\```bash
git clone https://github.com/your_username/rust_minigrad.git
cd rust_minigrad
cargo build --release
\```

For Python bindings you can install [matrurin](https://github.com/PyO3/maturin) and run:

\```bash
matrurin develop
\```

## TODOs

- [x] Implement core matrix operations.
- [ ] Add support for more matrix operations.
- [ ] Add support for automatic differentiation.
- [ ] Integrate GPU acceleration.
- [x] Add python bindings.
- [ ] Improve Python bindings for easier integration.
- [ ] Add more examples and benchmarks.
- [ ] Write some sort of docs and references?.

