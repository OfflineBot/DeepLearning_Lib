use ndarray::{Array1, Array2};

#[allow(unused)]
pub struct Matrix {
    pub w1: Array2<f64>,
    pub b1: Array1<f64>,
    pub w2: Array2<f64>,
    pub b2: Array1<f64>,
}

#[allow(unused)]
pub struct Normalize {
    pub x_mean: Array1<f64>,
    pub x_std: Array1<f64>,
    pub x_norm: Array2<f64>,
    pub y_mean: Array1<f64>,
    pub y_std: Array1<f64>,
    pub y_norm: Array2<f64>,
}

#[allow(unused)]
pub struct OutNorm {
    pub x_mean: Array1<f64>,
    pub x_std: Array1<f64>,
    pub y_mean: Array1<f64>,
    pub y_std: Array1<f64>,
}

#[allow(unused)]
pub struct Detail {
    pub hidden_layers: u16,
}

#[allow(unused)]
pub struct ZAStorage {
    z1: Array2<f64>,
    a1: Array2<f64>,
    z2: Array2<f64>,
}

#[allow(unused)]
pub struct DeltaStorage {
    delta2: Array2<f64>,
    delta1: Array2<f64>,
}