mod matrix;
use std::vec;

mod neuron;
use neuron::Neuron;

use matrix::Matrix;

fn main() {
    let rows = 5;
    let cols = 5;
    let matrix = Matrix::new(rows, cols, None);
    println!("{:?}", matrix);
    let matrix2 = Matrix::new(rows, cols, vec![1.0; rows * cols]);
    let matrix3 = matrix.dot(&matrix2);
    println!("{:?}", matrix3);
    test_matrix();
}

#[allow(dead_code)]
fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

#[allow(dead_code)]
fn relu(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        0.0
    }
}

fn test_matrix() {
    let rows = 2;
    let cols = 2;
    let mut matrix = Matrix::new(rows, cols, None);
    matrix.set(0, 0, 1.0);
    matrix.set(0, 1, 2.0);
    matrix.set(1, 0, 3.0);
    matrix.set(1, 1, 4.0);
    println!("{:?}", matrix);
    println!("{}", matrix.get(0, 0));
    println!("{}", matrix.get(0, 1));
    println!("{}", matrix.get(1, 0));
    println!("{}", matrix.get(1, 1));
    let matrix2 = Matrix::new(rows, cols, vec![1.0; rows * cols]);
    let matrix3 = matrix.dot(&matrix2);
    println!("{:?}", matrix3);
}

#[allow(dead_code)]
fn test_neuron() {
    let weights = Matrix::new(2, 2, vec![1.0; 4]);
    let bias = 1.0;
    let neuron = Neuron::new(weights, bias, sigmoid);
    println!("{:?}", neuron);
}
