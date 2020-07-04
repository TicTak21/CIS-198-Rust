/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
  let mat_len = mat1.len();
  let row_len = mat2[0].len();

  assert_eq!(mat_len, row_len);

  let mut result_mat: Matrix = Vec::new();

  for i in 0..mat_len {
    let mut sum = 0.0;

    for j in 0..row_len {
      sum += mat1[i][j] * mat2[j][i];
    }

    result_mat.push(vec![sum]);
  }

  return result_mat;
}
