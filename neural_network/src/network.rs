use nalgebra::SVector;

use crate::layer::{Layer, SIGMOID};

pub struct Network {
    // input : 
    //      - player height 1
    //      - nearest_pipe_top(x_start, ystart, x_end, y_end) 4
    //      - nearest_pipe_bot(x_start, ystart, x_end, y_end) 4
    //      - 2nearest_pipe_top(x_start, ystart, x_end, y_end) 4
    //      - 2nearest_pipe_bot(x_start, ystart, x_end, y_end) 4
    layer1: Layer<32, 17>,
    layer2: Layer<1, 32>
}

impl Network {
    pub fn train(
    &mut self,
    duration: u32) {
        let mut layer1 = &mut self.layer1;
        let mut layer2= &mut self.layer2;

        for i in 1..=duration {
            for j in 0..inputs.len() {
                let out = layer1.forward(inputs[j].clone());
                let out2 = layer2.forward(out);

                let error = expect[j]-out2;
                let gradient = out2.map(|elem: f64| (SIGMOID.derivative)(&elem));
                
                let (err, grad) = layer2.back_propag(error, gradient);
                (_,_) = layer1.back_propag(err, grad)
            }
        }
}
}

const GOAL_SCORE: f64 = 1000.0; // number of frames to survive

pub fn fit(final_score: f64, action_idx: f64)-> SVector<f64, 1> {
    //returns the error
    SVector::<f64, 1>::new(action_idx/final_score - final_score/GOAL_SCORE)
}
