use std::vec;

use nalgebra::SVector;
use neural_network::layer::{Layer, SIGMOID};

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
    let mut layer1: Layer<5, 2> = Layer::<5, 2>::default();
    let mut layer2: Layer<1, 5> = Layer::<1, 5>::default();

    let inputs: Vec<SVector<f64, 2>> = vec![
        SVector::<f64, 2>::new(1., 0.),
        SVector::<f64, 2>::new(1., 1.),
        SVector::<f64, 2>::new(0., 1.),
        SVector::<f64, 2>::new(0., 0.),
    ];

    let expect: Vec<SVector<f64, 1>> = vec![
        SVector::<f64, 1>::new(1.),
        SVector::<f64, 1>::new(0.),
        SVector::<f64, 1>::new(1.),
        SVector::<f64, 1>::new(0.),
    ];
    train(&mut layer1, &mut layer2, 10000, &inputs, expect);
    for input in inputs {
        print_res(&mut layer1, &mut layer2, input)
    }
}
