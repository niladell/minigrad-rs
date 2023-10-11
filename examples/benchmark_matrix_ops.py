import timeit
import numpy as np
from memory_profiler import memory_usage

def benchmark_homeydl(n):
    import homeydl  # Moved import statement inside the function
    matrix1 = homeydl.Matrix(n, n, [[float(i+j) for i in range(n)] for j in range(n)])
    matrix2 = homeydl.Matrix(n, n, [[float(i+j) for i in range(n)] for j in range(n)])
    result = homeydl.matrix_dot(matrix1, matrix2)

def benchmark_numpy(n):
    matrix1 = np.random.rand(n, n)
    matrix2 = np.random.rand(n, n)
    result = np.dot(matrix1, matrix2)

def main():
    # Define the sizes to test
    sizes = [10, 100, 1000]

    # Time the benchmarks
    for size in sizes:
        homeydl_time = timeit.timeit("benchmark_homeydl({})".format(size), globals=globals(), number=10)
        numpy_time = timeit.timeit("benchmark_numpy({})".format(size), globals=globals(), number=10)

        print(f"Size: {size}")
        print(f"homeydl time: {homeydl_time}")
        print(f"numpy time: {numpy_time}")

        # Memory usage
        homeydl_mem = max(memory_usage((benchmark_homeydl, (size,))))
        numpy_mem = max(memory_usage((benchmark_numpy, (size,))))

        print(f"homeydl memory: {homeydl_mem}")
        print(f"numpy memory: {numpy_mem}")

if __name__ == '__main__':
    main()
