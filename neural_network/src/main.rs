use std::{fs::File, os::linux::net, vec};

use nalgebra::SVector;
use neural_network::{layer::{Layer, SIGMOID}, network::Network};

fn train(
    layer1: &mut Layer<5, 2>,
    layer2: &mut Layer<1, 5>,
    duration: u32,
    inputs: &Vec<SVector<f64, 2>>,
    expect: Vec<SVector<f64, 1>>) {
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

fn print_res(
    layer1: &mut Layer<5, 2>,
    layer2: &mut Layer<1, 5>,
    input: SVector<f64, 2>
) {
    let out = layer1.forward(input.clone());
    let out2 = layer2.forward(out);
    println!("{} xor {}: {}", input[0], input[1], out2[0]);
}

fn main() {
    let mut files_found = true;
    let weights1file = match File::open("data/weights_layer1") {
        Ok(f)=>f,
        Err(_)=>{files_found = false; File::create("data/weights_layer1").unwrap()}
    };

    let weights2file = match File::open("data/weights_layer2") {
        Ok(f)=>f,
        Err(_)=>{files_found = false; File::create("data/weights_layer2").unwrap()}
    };

    let bias1file = match File::open("data/bias_layer1") {
        Ok(f)=>f,
        Err(_)=>{files_found = false; File::create("data/bias_layer1").unwrap()}
    };

    let bias2file = match File::open("data/bias_layer2") {
        Ok(f)=>f,
        Err(_)=>{files_found = false; File::create("data/bias_layer2").unwrap()}
    };
    let mut network = Network::new();
    if files_found {
        network = Network::from_file(
            bias1file,
            weights1file,
            bias2file,
            weights2file);
    }

    network.train(100);
    network.replay();
}
