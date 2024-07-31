mod matrix;
use matrix::Matrix;

mod neuron;
use neuron::Neuron;

fn main() {
    let output = input_layer();
    println!("Output: {}", output);
}

fn input_layer() -> f64 {
    let input = Matrix::new(3, 1, Some(vec![5.0, 1.0, 3.0]));
    hidden_layer(input.transpose())
}

fn hidden_layer(input: Matrix) -> f64 {
    let weight1 = Matrix::new(3, 1, Some(vec![0.8, 0.3, 1.0]));
    let weight2 = Matrix::new(3, 1, Some(vec![0.2, 0.5, 0.4]));
    let neuron1 = Neuron::new(weight1, 0.0, sigmoid);
    let neuron2 = Neuron::new(weight2, 0.0, sigmoid);
    let output1 = neuron1.forward(&input);
    let output2 = neuron2.forward(&input);
    let hidden_output = Matrix::new(2, 1, Some(vec![output1, output2]));
    let output = output_layer(hidden_output.transpose());
    output
}

fn output_layer(input: Matrix) -> f64 {
    let weight = Matrix::new(2, 1, Some(vec![0.3, 0.5]));
    let bias = -0.5;
    let func = sigmoid;
    let neuron = Neuron::new(weight, bias, func);
    neuron.forward(&input)
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

#[allow(dead_code)]
fn tanh(x: f64) -> f64 {
    x.tanh()
}
