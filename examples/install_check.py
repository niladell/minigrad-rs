import homeydl


def test_dot_product():
    # Create two matrices
    matrix1 = homeydl.Matrix(2, 2, [[1.0, 2.0], [3.0, 4.0]])
    matrix2 = homeydl.Matrix(2, 2, [[2.0, 0.0], [1.0, 3.0]])

    result = homeydl.matrix_dot(matrix1, matrix2)

    print("Dot Product Result:")
    print(result)

    vec_1 = [0, 1, 2, 4]
    vec_2 = [0, 1, 2, 4]

    vec = homeydl.vector_dot(vec_1, vec_2)
    print(f"Vector dot: {vec}")

    vec = homeydl.vector_multiplication_elementwise(vec_1, vec_2)
    print(f"Vector dot: {vec}")

    if result.to_list() == [[4.0, 6.0], [10.0, 12.0]]:
        print("Test passed!")
    else:
        print("Test failed!")


if __name__ == "__main__":
    test_dot_product()
