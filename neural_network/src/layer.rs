use std::{default, f64::consts::E, fs::File, io::{IoSlice, IoSliceMut, Read, Write}};

use nalgebra::{DefaultAllocator, Matrix, SMatrix, SVector, VectorN};

pub struct Activation {
    pub function: fn(&f64) -> f64,
    pub derivative: fn(&f64) -> f64,
}

pub const SIGMOID: Activation = Activation {
    function: |x| 1.0 / (1.0 + E.powf(-x)),
    derivative: |x| x * (1.0 - x),
};

pub struct Layer<const SIZE: usize, const BATCHES: usize> {
    weights: SMatrix<f64, SIZE, BATCHES>,
    bias: SVector<f64, SIZE>,
    activation: Activation,
    data: SVector<f64, BATCHES>
}

impl<const SIZE: usize, const BATCHES: usize> Layer<SIZE, BATCHES> {
    pub fn new(weights: SMatrix<f64, SIZE, BATCHES>, biases: SVector<f64, SIZE>, activation: Activation)->Self {
        let data = SVector::<f64, BATCHES>::zeros();
        Layer { weights: weights, bias: biases , activation: activation, data: data}
    }

    pub fn forward(&mut self, input: SVector<f64, BATCHES>) -> SVector<f64, SIZE> {
        self.data = input.clone();
        (self.weights * input + self.bias).map(|x: f64| (self.activation.function)(&x))
    }

    pub fn back_propag(
            &mut self,
            error: SVector<f64, SIZE>,
            mut gradient: SVector<f64, SIZE>) ->(
                SVector<f64, BATCHES>,
                SVector<f64, BATCHES>
            ){
        gradient.component_mul_assign(&error);
        gradient = gradient.map(|x| x * 1.);
        
        // update weights
        let transpose = self.data.clone().transpose();
        self.weights = self.weights + gradient * transpose;

        // update bias
        self.bias = self.bias + gradient;

        let res_error: SVector<f64, BATCHES> = self.weights.transpose() * error;
        let res_gradient: SVector<f64, BATCHES> = self.data.map(|elem| (self.activation.derivative)(&elem));
        (res_error, res_gradient)
    }

    pub fn weights_to_file(&self, mut file: File) {
        for i in 0..SIZE {
            for j in 0..BATCHES {
                let to_bytes = &self.weights.row(i)[j].to_be_bytes();
                let slices = [IoSlice::new(to_bytes)];
                
                let _ = file.write_vectored(&slices);
            }
        }
    }

    pub fn file_to_weights(&mut self, mut file: File) {
        for i in 0..SIZE {
            for j in 0..BATCHES {
                let mut to_bytes:[u8; 8] = Default::default();
                let mut slices = [IoSliceMut::new(&mut to_bytes)];
                
                let _ = file.read_vectored(&mut slices);
                self.weights.row_mut(i)[j] = f64::from_be_bytes(to_bytes);
            }
        }
    }


    pub fn biases_to_file(&self, mut file: File) {
        for i in 0..SIZE {
            for j in 0..BATCHES {
                let to_bytes = &self.bias.row(i)[j].to_be_bytes();
                let slices = [IoSlice::new(to_bytes)];
                
                let _ = file.write_vectored(&slices);
            }
        }
    }

    pub fn file_to_biases(&mut self, mut file: File) {
        for i in 0..SIZE {
            for j in 0..BATCHES {
                let mut to_bytes:[u8; 8] = Default::default();
                let mut slices = [IoSliceMut::new(&mut to_bytes)];
                
                let _ = file.read_vectored(&mut slices);
                self.bias.row_mut(i)[j] = f64::from_be_bytes(to_bytes);
            }
        }
    }
}

impl<const SIZE: usize, const BATCHES: usize> Default for Layer<SIZE, BATCHES> {
    fn default() -> Self {
        let weights: SMatrix<f64, SIZE, BATCHES> = SMatrix::<f64, SIZE, BATCHES>::new_random();
        let biases: SVector<f64, SIZE> = SVector::<f64, SIZE>::new_random();
        let activation = SIGMOID;
        Self::new(weights, biases, activation)
    }
}

