use rand::prelude::*;
use rand_distr::{Distribution, Normal};
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
    det: Option<f64>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize, data: Option<Vec<f64>>) -> Self {
        if rows < 1 || cols < 1 {
            panic!("Invalid matrix dimensions. Rows and cols must be greater than 0.");
        }
        match data {
            Some(data) => {
                if data.len() != rows * cols {
                    panic!("Invalid matrix data. Data length must be equal to rows * cols.");
                } else {
                    Self { rows, cols, data, det: None }
                }
            }
            None => Self { rows, cols, data: vec![0.0; rows * cols], det: None },
        }
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        if row >= self.rows || col >= self.cols {
            panic!("Index out of bounds. Row: {}, Col: {}", row, col);
        }
        self.data[row * self.cols + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        if row >= self.rows || col >= self.cols {
            panic!("Index out of bounds. Row: {}, Col: {}", row, col);
        }
        self.data[row * self.cols + col] = value;
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn data(&self) -> &Vec<f64> {
        &self.data
    }

    pub fn size(&self) -> usize {
        self.rows * self.cols
    }

    pub fn row(&self, row: usize) -> Vec<f64> {
        if row >= self.rows {
            panic!("Index out of bounds. Row: {}", row);
        }
        let start = row * self.cols;
        let end = start + self.cols;
        self.data[start..end].to_vec()
    }

    pub fn col(&self, col: usize) -> Vec<f64> {
        if col >= self.cols {
            panic!("Index out of bounds. Col: {}", col);
        }
        (0..self.rows).map(|i| self.data[i * self.cols + col]).collect()
    }
}

impl Add for Matrix {
    type Output = Matrix;
    fn add(self, matrix: Matrix) -> Self::Output {
        if self.rows != matrix.rows || self.cols != matrix.cols {
            panic!("Matrix dimensions must be equal.");
        }
        let mut result = self.clone();
        for i in 0..result.data.len() {
            result.data[i] += matrix.data[i];
        }
        result
    }
}

impl Add<f64> for Matrix {
    type Output = Matrix;
    fn add(self, scalar: f64) -> Self::Output {
        let mut matrix = self.clone();
        for i in 0..matrix.data.len() {
            matrix.data[i] += scalar;
        }
        matrix
    }
}

impl Add<Matrix> for f64 {
    type Output = Matrix;
    fn add(self, matrix: Matrix) -> Matrix {
        matrix + self
    }
}

impl Sub for Matrix {
    type Output = Matrix;
    fn sub(self, matrix: Matrix) -> Self::Output {
        if self.rows != matrix.rows || self.cols != matrix.cols {
            panic!("Matrix dimensions must be equal.");
        }
        let mut result = self.clone();
        for i in 0..result.data.len() {
            result.data[i] -= matrix.data[i];
        }
        result
    }
}

impl Sub<f64> for Matrix {
    type Output = Matrix;
    fn sub(self, scalar: f64) -> Self::Output {
        let mut matrix = self.clone();
        for i in 0..matrix.data.len() {
            matrix.data[i] -= scalar;
        }
        matrix
    }
}

impl Sub<Matrix> for f64 {
    type Output = Matrix;
    fn sub(self, matrix: Matrix) -> Matrix {
        let mut result = matrix.clone();
        for i in 0..result.data.len() {
            result.data[i] = self - result.data[i];
        }
        result
    }
}

impl Mul for Matrix {
    type Output = Matrix;
    fn mul(self, matrix: Matrix) -> Self::Output {
        if self.rows != matrix.rows || self.cols != matrix.cols {
            panic!("Matrix dimensions must be equal.");
        }
        let mut result = self.clone();
        for i in 0..result.data.len() {
            result.data[i] *= matrix.data[i];
        }
        result
    }
}

impl Mul<f64> for Matrix {
    type Output = Matrix;
    fn mul(self, scalar: f64) -> Self::Output {
        let mut matrix = self.clone();
        for i in 0..matrix.data.len() {
            matrix.data[i] *= scalar;
        }
        matrix
    }
}

impl Mul<Matrix> for f64 {
    type Output = Matrix;
    fn mul(self, matrix: Matrix) -> Matrix {
        matrix * self
    }
}

pub trait MatrixOps {
    type Output;
    fn dot(&self, matrix: &Matrix) -> Self::Output;
    fn initialize(&mut self, n_inputs: usize);
    fn inv(&self) -> Self::Output;
    fn transpose(&self) -> Self::Output;
    fn det(&self) -> f64;
    fn cofactor(&self, row: usize, col: usize) -> f64;
}

impl MatrixOps for Matrix {
    type Output = Matrix;

    fn dot(&self, matrix: &Matrix) -> Self::Output {
        if self.cols != matrix.rows {
            panic!("Matrix dimensions should be m x n and n x p.");
        }
        let mut result = Matrix::new(self.rows, matrix.cols, None);
        for i in 0..self.rows {
            for j in 0..matrix.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.get(i, k) * matrix.get(k, j);
                }
                result.set(i, j, sum);
            }
        }
        result
    }

    fn initialize(&mut self, n_inputs: usize) {
        let standard_deviation = (2.0_f64 / n_inputs as f64).sqrt();
        let normal = Normal::new(0_f64, standard_deviation).unwrap();
        self.data.iter_mut().for_each(|x| *x = normal.sample(&mut thread_rng()));
    }

    fn inv(&self) -> Self::Output {
        if self.cols == self.rows {
            todo!()
        } else {
            panic!("Matrix must be square.");
        }
    }

    fn transpose(&self) -> Self::Output {
        let mut matrix = Matrix::new(self.cols, self.rows, None);
        for i in 0..self.rows {
            for j in 0..self.cols {
                matrix.set(j, i, self.get(i, j));
            }
        }
        matrix
    }

    fn det(&self) -> f64 {
        if self.cols == self.rows {
            todo!()
        } else {
            panic!("Matrix must be square.");
        }
    }

    fn cofactor(&self, row: usize, col: usize) -> f64 {
        if self.cols == self.rows {
            todo!()
        } else {
            panic!("Matrix must be square.");
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_matrix() {
        let matrix = Matrix::new(2, 2, Some(vec![1.0, 2.0, 3.0, 4.0]));
        assert_eq!(matrix.rows, 2);
        assert_eq!(matrix.cols, 2);
        assert_eq!(matrix.data, vec![1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn should_create_empty_matrix() {
        let matrix = Matrix::new(2, 2, None);
        assert_eq!(matrix.rows, 2);
        assert_eq!(matrix.cols, 2);
        assert_eq!(matrix.data, vec![0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    #[should_panic]
    pub fn should_not_create_matrix() {
        let _ = Matrix::new(0, 0, None);
    }

    #[test]
    fn should_get_matrix_element() {
        let matrix = Matrix::new(2, 2, Some(vec![1.0, 2.0, 3.0, 4.0]));
        assert_eq!(matrix.get(0, 0), 1.0);
        assert_eq!(matrix.get(0, 1), 2.0);
        assert_eq!(matrix.get(1, 0), 3.0);
        assert_eq!(matrix.get(1, 1), 4.0);
    }

    #[test]
    #[should_panic]
    fn should_not_get_matrix_element() {
        let matrix = Matrix::new(2, 2, Some(vec![1.0, 2.0, 3.0, 4.0]));
        matrix.get(2, 2);
    }

    #[test]
    fn should_set_matrix_element() {
        let mut matrix = Matrix::new(2, 2, Some(vec![1.0, 2.0, 3.0, 4.0]));
        matrix.set(0, 0, 5.0);
        assert_eq!(matrix.get(0, 0), 5.0);
    }

    #[test]
    #[should_panic]
    fn should_not_set_matrix_element() {
        let mut matrix = Matrix::new(2, 2, Some(vec![1.0, 2.0, 3.0, 4.0]));
        matrix.set(2, 2, 5.0);
    }

    #[test]
    fn should_get_matrix_rows() {
        let matrix = Matrix::new(2, 2, None);
        assert_eq!(matrix.rows(), 2);
    }

    #[test]
    fn should_get_matrix_cols() {
        let matrix = Matrix::new(2, 2, None);
        assert_eq!(matrix.cols(), 2);
    }

    #[test]
    fn should_get_matrix_data() {
        let matrix = Matrix::new(2, 2, Some(vec![1.0, 2.0, 3.0, 4.0]));
        assert_eq!(*matrix.data(), vec![1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn should_get_matrix_size() {
        let matrix = Matrix::new(2, 2, None);
        assert_eq!(matrix.size(), 4);
    }

    #[test]
    fn should_get_matrix_row() {
        let matrix = Matrix::new(2, 2, Some(vec![1.0, 2.0, 3.0, 4.0]));
        assert_eq!(matrix.row(0), vec![1.0, 2.0]);
        assert_eq!(matrix.row(1), vec![3.0, 4.0]);
    }

    #[test]
    #[should_panic]
    fn should_not_get_matrix_row() {
        let matrix = Matrix::new(2, 2, None);
        matrix.row(2);
    }

    #[test]
    fn should_get_matrix_col() {
        let matrix = Matrix::new(2, 2, Some(vec![1.0, 2.0, 3.0, 4.0]));
        assert_eq!(matrix.col(0), vec![1.0, 3.0]);
        assert_eq!(matrix.col(1), vec![2.0, 4.0]);
    }

    #[test]
    #[should_panic]
    fn should_not_get_matrix_col() {
        let matrix = Matrix::new(2, 2, None);
        matrix.col(2);
    }

    #[test]
    fn should_add_matrix() {
        let matrix1 = Matrix::new(2, 2, Some(vec![1.0, 2.0, 3.0, 4.0]));
        let matrix2 = Matrix::new(2, 2, Some(vec![1.0, 2.0, 3.0, 4.0]));
        let result = matrix1 + matrix2;
        assert_eq!(result.data, vec![2.0, 4.0, 6.0, 8.0]);
    }

    #[test]
    #[should_panic]
    fn should_not_add_matrix() {
        let matrix1 = Matrix::new(2, 2, Some(vec![1.0, 2.0, 3.0, 4.0]));
        let matrix2 = Matrix::new(2, 3, Some(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]));
        let _ = matrix1 + matrix2;
    }

    #[test]
    fn should_add_f64() {
        let matrix = Matrix::new(2, 2, Some(vec![1.0, 2.0, 3.0, 4.0]));
        let result = matrix + 1.0;
        assert_eq!(result.data, vec![2.0, 3.0, 4.0, 5.0]);
    }

    #[test]
    fn should_add_f64_to_matrix() {
        let matrix = Matrix::new(2, 2, Some(vec![1.0, 2.0, 3.0, 4.0]));
        let result = 1.0 + matrix;
        assert_eq!(result.data, vec![2.0, 3.0, 4.0, 5.0]);
    }

    #[test]
    fn should_subtract_matrix() {
        let matrix1 = Matrix::new(2, 2, Some(vec![1.0, 2.0, 3.0, 4.0]));
        let matrix2 = Matrix::new(2, 2, Some(vec![1.0, 2.0, 3.0, 4.0]));
        let result = matrix1 - matrix2;
        assert_eq!(result.data, vec![0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    #[should_panic]
    fn should_not_subtract_matrix() {
        let matrix1 = Matrix::new(2, 2, Some(vec![1.0, 2.0, 3.0, 4.0]));
        let matrix2 = Matrix::new(2, 3, Some(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]));
        let _ = matrix1 - matrix2;
    }

    #[test]
    fn should_subtract_f64() {
        let matrix = Matrix::new(2, 2, Some(vec![1.0, 2.0, 3.0, 4.0]));
        let result = matrix - 1.0;
        assert_eq!(result.data, vec![0.0, 1.0, 2.0, 3.0]);
    }

    #[test]
    fn should_subtract_f64_from_matrix() {
        let matrix = Matrix::new(2, 2, Some(vec![1.0, 2.0, 3.0, 4.0]));
        let result = 1.0 - matrix;
        assert_eq!(result.data, vec![0.0, -1.0, -2.0, -3.0]);
    }

    #[test]
    fn should_multiply_matrix() {
        let matrix1 = Matrix::new(2, 2, Some(vec![1.0, 2.0, 3.0, 4.0]));
        let matrix2 = Matrix::new(2, 2, Some(vec![1.0, 2.0, 3.0, 4.0]));
        let result = matrix1 * matrix2;
        assert_eq!(result.data, vec![1.0, 4.0, 9.0, 16.0]);
    }

    #[test]
    #[should_panic]
    fn should_not_multiply_matrix() {
        let matrix1 = Matrix::new(2, 3, Some(vec![1.0, 2.0, 3.0, 4.0]));
        let matrix2 = Matrix::new(3, 4, Some(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]));
        let _ = matrix1 * matrix2;
    }

    #[test]
    fn should_multiply_f64() {
        let matrix = Matrix::new(2, 2, Some(vec![1.0, 2.0, 3.0, 4.0]));
        let result = matrix * 2.0;
        assert_eq!(result.data, vec![2.0, 4.0, 6.0, 8.0]);
    }

    #[test]
    fn should_multiply_f64_to_matrix() {
        let matrix = Matrix::new(2, 2, Some(vec![1.0, 2.0, 3.0, 4.0]));
        let result = 2.0 * matrix;
        assert_eq!(result.data, vec![2.0, 4.0, 6.0, 8.0]);
    }

    #[test]
    fn testing_initialize() {
        let mut matrix = Matrix::new(1, 10, None);
        matrix.initialize(10);
        assert_eq!(matrix.data.len(), 10);
        assert_ne!(matrix.data, vec![0.0; 10]);
    }

    #[test]
    fn should_transpose_matrix() {
        let matrix = Matrix::new(2, 3, Some(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]));
        let transposed_matrix = matrix.transpose();
        assert_eq!(transposed_matrix.rows, 3);
        assert_eq!(transposed_matrix.cols, 2);
        assert_eq!(transposed_matrix.data, vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0]);
    }

    #[test]
    fn should_print_matrix() {
        let matrix = Matrix::new(2, 2, Some(vec![1.0, 2.0, 3.0, 4.0]));
        assert_eq!(format!("{:?}", matrix), "Matrix { rows: 2, cols: 2, data: [1.0, 2.0, 3.0, 4.0] }");
    }

    #[test]
    fn should_equate_matrix() {
        let matrix1 = Matrix::new(2, 2, Some(vec![1.0, 2.0, 3.0, 4.0]));
        let matrix2 = Matrix::new(2, 2, Some(vec![1.0, 2.0, 3.0, 4.0]));
        assert_eq!(matrix1, matrix2);
    }

    #[test]
    fn should_not_equate_matrix() {
        let matrix1 = Matrix::new(2, 2, Some(vec![1.0, 2.0, 3.0, 4.0]));
        let matrix2 = Matrix::new(2, 2, Some(vec![1.0, 2.0, 3.0, 5.0]));
        assert_ne!(matrix1, matrix2);
    }

    #[test]
    fn should_clone_matrix() {
        let matrix = Matrix::new(2, 2, Some(vec![1.0, 2.0, 3.0, 4.0]));
        let cloned_matrix = matrix.clone();
        assert_eq!(cloned_matrix, matrix);
    }
}