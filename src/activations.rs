use crate::matrix::Matrix;

pub fn relu(x: f32) -> f32 {
    x.max(0.0)
}

pub fn relu_derivative(x: f32) -> f32 {
    if x > 0.0 {1.0} else {0.0}
}

pub fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

pub fn sigmoid_derivative(x: f32) -> f32 {
    let s = sigmoid(x);
    s * (1.0 - s)
}

pub fn tanh(x: f32) -> f32 {
    x.tanh()
}

pub fn tanh_derivative(x: f32) -> f32 {
    let t = tanh(x);
    1.0 - t * t
}

pub fn softmax(matrix: &Matrix) -> Matrix {
    let max_val = matrix.data.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let exp_data: Vec<f32> = matrix.data.iter().map(|&x| (x-max_val).exp()).collect();
    let sum: f32 = exp_data.iter().sum();

    let data = exp_data.iter().map(|&x| x / sum).collect();
    Matrix { rows: matrix.rows, cols: matrix.cols, data }
}