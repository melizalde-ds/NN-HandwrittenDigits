use crate::matrix::MatrixOps;

mod matrix;

fn main() {
    let mut matrix1 = matrix::Matrix::new(5, 5, None);
    println!("{:?}", matrix1);
    matrix1.initialize(8);
    println!("{:?}", matrix1);
}