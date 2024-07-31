use std::{fmt::Debug, ops::Add};

pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl Matrix {
    pub fn new<T: Into<Option<Vec<f64>>>>(rows: usize, cols: usize, data: T) -> Self {
        let data = if let Some(data) = data.into() {
            assert_eq!(rows * cols, data.len());
            data
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
