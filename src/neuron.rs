use std::fmt::Debug;

use crate::matrix::Matrix;

pub struct Neuron {
    weights: Matrix,
    bias: f64,
    func: fn(f64) -> f64,
}

impl Neuron {
    #[allow(dead_code)]
    pub fn new(weights: Matrix, bias: f64, func: fn(f64) -> f64) -> Neuron {
        Neuron {
            weights,
            bias,
            func,
        }
    }

    #[allow(dead_code)]
    pub fn get_weights(&self) -> &Matrix {
        &self.weights
    }

    #[allow(dead_code)]
    pub fn get_bias(&self) -> f64 {
        self.bias
    }

    #[allow(dead_code)]
    pub fn get_func(&self) -> fn(f64) -> f64 {
        self.func
    }

    #[allow(dead_code)]
    pub fn set_weights(&mut self, weights: Matrix) {
        self.weights = weights;
    }

    #[allow(dead_code)]
    pub fn set_bias(&mut self, bias: f64) {
        self.bias = bias;
    }

    pub fn forward(&self, inputs: &Matrix) -> f64 {
        let weighted_sum = self.weights.dot(inputs).sum() + self.bias;
        (self.func)(weighted_sum)
    }
}

impl Debug for Neuron {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Neuron {{ weights: {:?}, bias: {}, func: {:?} }}",
            self.weights, self.bias, self.func
        )
    }
}
