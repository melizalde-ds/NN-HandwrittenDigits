use std::{fmt::Debug, ops::Add};

pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl Matrix {
    pub fn new<T: Into<Option<Vec<f64>>>>(rows: usize, cols: usize, data: T) -> Self {
        let data = if let Some(data) = data.into() {
            match data.len() {
                len if len == rows * cols => data,
                _ => vec![0.0; rows * cols],
            }
        } else {
            vec![0.0; rows * cols]
        };
        Self { rows, cols, data }
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.cols + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        self.data[row * self.cols + col] = value;
    }

    #[allow(dead_code)]
    pub fn rows(&self) -> usize {
        self.rows
    }

    #[allow(dead_code)]
    pub fn cols(&self) -> usize {
        self.cols
    }

    #[allow(dead_code)]
    pub fn data(&self) -> &[f64] {
        &self.data
    }

    pub fn dot(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.cols, other.rows);
        let mut result = Matrix::new(self.rows, other.cols, None);
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.get(i, k) * other.get(k, j);
                }
                result.set(i, j, sum);
            }
        }
        result
    }

    #[allow(dead_code)]
    pub fn func(&self, func: fn(f64) -> f64) -> Matrix {
        let mut result = Matrix::new(self.rows, self.cols, None);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.set(i, j, func(self.get(i, j)));
            }
        }
        result
    }
}

impl Add<f64> for Matrix {
    type Output = Matrix;

    fn add(self, other: f64) -> Matrix {
        let mut result = Matrix::new(self.rows, self.cols, None);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.set(i, j, self.get(i, j) + other);
            }
        }
        result
    }
}

impl Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for i in 0..self.rows {
            for j in 0..self.cols {
                s.push_str(&self.get(i, j).to_string());
                s.push_str(" ");
            }
            s.push_str("\n");
        }
        write!(f, "{}", s)
    }
}

// Test the Matrix struct
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_matrix_null() {
        let rows = 2;
        let cols = 2;
        let matrix = Matrix::new(rows, cols, None);
        assert_eq!(matrix.rows(), rows);
        assert_eq!(matrix.cols(), cols);
        assert_eq!(matrix.data(), &[0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn create_matrix_data() {
        let rows = 2;
        let cols = 2;
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let matrix = Matrix::new(rows, cols, Some(data.clone()));
        assert_eq!(matrix.rows(), rows);
        assert_eq!(matrix.cols(), cols);
        assert_eq!(matrix.data(), &data);
    }

    #[test]
    fn create_matrix_data_fail() {
        let rows = 2;
        let cols = 2;
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let matrix = Matrix::new(rows, cols, Some(data.clone()));
        assert_eq!(matrix.rows(), rows);
        assert_eq!(matrix.cols(), cols);
        assert_eq!(matrix.data(), &[0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn get_set() {
        let rows = 2;
        let cols = 2;
        let mut matrix = Matrix::new(rows, cols, None);
        matrix.set(0, 0, 1.0);
        matrix.set(0, 1, 2.0);
        matrix.set(1, 0, 3.0);
        matrix.set(1, 1, 4.0);
        assert_eq!(matrix.get(0, 0), 1.0);
        assert_eq!(matrix.get(0, 1), 2.0);
        assert_eq!(matrix.get(1, 0), 3.0);
        assert_eq!(matrix.get(1, 1), 4.0);
    }

    #[test]
    fn dot() {
        let rows = 2;
        let cols = 2;
        let mut matrix = Matrix::new(rows, cols, None);
        matrix.set(0, 0, 1.0);
        matrix.set(0, 1, 2.0);
        matrix.set(1, 0, 3.0);
        matrix.set(1, 1, 4.0);
        let matrix2 = Matrix::new(rows, cols, vec![1.0; rows * cols]);
        let matrix3 = matrix.dot(&matrix2);
        assert_eq!(matrix3.data(), &[3.0, 3.0, 7.0, 7.0]);
    }

    #[test]
    fn func() {
        let rows = 2;
        let cols = 2;
        let mut matrix = Matrix::new(rows, cols, None);
        matrix.set(0, 0, 1.0);
        matrix.set(0, 1, 2.0);
        matrix.set(1, 0, 3.0);
        matrix.set(1, 1, 4.0);
        let matrix2 = matrix.func(|x| x + 1.0);
        assert_eq!(matrix2.data(), &[2.0, 3.0, 4.0, 5.0]);
    }

    #[test]
    fn add() {
        let rows = 2;
        let cols = 2;
        let mut matrix = Matrix::new(rows, cols, None);
        matrix.set(0, 0, 1.0);
        matrix.set(0, 1, 2.0);
        matrix.set(1, 0, 3.0);
        matrix.set(1, 1, 4.0);
        let matrix2 = matrix + 1.0;
        assert_eq!(matrix2.data(), &[2.0, 3.0, 4.0, 5.0]);
    }

    #[test]
    fn debug() {
        let rows = 2;
        let cols = 2;
        let mut matrix = Matrix::new(rows, cols, None);
        matrix.set(0, 0, 1.0);
        matrix.set(0, 1, 2.0);
        matrix.set(1, 0, 3.0);
        matrix.set(1, 1, 4.0);
        assert_eq!(format!("{:?}", matrix), "1 2 \n3 4 \n");
    }

    #[test]
    #[should_panic]
    fn dot_fail() {
        let rows = 2;
        let cols = 2;
        let matrix = Matrix::new(rows, cols, None);
        let matrix2 = Matrix::new(rows, cols, None);
        matrix.dot(&matrix2);
        assert_eq!(matrix.data(), &[1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    #[should_panic]
    fn add_fail() {
        let rows = 2;
        let cols = 2;
        let matrix = Matrix::new(rows, cols, None);
        let matrix = matrix + 1.0;
        assert_eq!(matrix.data(), &[0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    #[should_panic]
    fn get_fail() {
        let rows = 2;
        let cols = 2;
        let matrix = Matrix::new(rows, cols, None);
        matrix.get(2, 2);
    }

    #[test]
    #[should_panic]
    fn set_fail() {
        let rows = 2;
        let cols = 2;
        let mut matrix = Matrix::new(rows, cols, None);
        matrix.set(2, 2, 1.0);
    }
}
