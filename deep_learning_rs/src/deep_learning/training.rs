use ndarray::{Array1, Array2};
use ndarray_rand::RandomExt;
use rand::distributions::Uniform;
use crate::deep_learning::structs::*;

#[allow(unused)]
struct TrainingData {
    matrix: Matrix,
    norm: OutNorm,
    detail: Detail,
}

impl TrainingData {
    pub fn normalize(input_data: Array2<f64>, output_data: Array2<f64>) -> Normalize {

    }
    pub fn init_matrix(input_layer_size: u16, hidden_layer_size: u16, output_layer_size: u16) -> Matrix {

    }
    pub fn update_matrix(iterations: u32, learning_rate: f64, za_storage: ZAStorage, delta_storage: DeltaStorage) -> TrainingData {

    }
}