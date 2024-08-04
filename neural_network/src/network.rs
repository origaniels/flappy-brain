use std::f64::consts::E;
use std::fs::File;
use std::{io::{stdout, Result}, time::Instant};
use ratatui::{
    backend::CrosstermBackend, crossterm::{
        event::{self, KeyCode, KeyEventKind}, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand
    }, Terminal
};
use backend::game::Game;
use frontend::interface::Drawable;
use nalgebra::SVector;

use crate::layer::{Layer, SIGMOID};

const LEARNING_RATE: f64 = 0.5;

pub struct Network {
    // input : 
    //      - player height 1
    //      - nearest_pipe_top(x_start, ystart, x_end, y_end) 4
    //      - nearest_pipe_bot(x_start, ystart, x_end, y_end) 4
    //      - 2nearest_pipe_top(x_start, ystart, x_end, y_end) 4
    //      - 2nearest_pipe_bot(x_start, ystart, x_end, y_end) 4
    layer1: Layer<32, 17>,
    layer2: Layer<2, 32>,
    memory1: Layer<32, 17>,
    memory2: Layer<2, 32>
}

impl Network {
    pub fn new()->Self {
        let layer1 = Layer::<32, 17>::default();
        let layer2 = Layer::<2, 32>::default();
        let memory1 = layer1.clone();
        let memory2 = layer2.clone();
        Network {
            layer1,
            layer2,
            memory1,
            memory2
        }
    }

    pub fn from_file(layer1_bias: File, layer1_weights: File, layer2_bias: File, layer2_weights: File) ->Self{
        let mut _self: Network = Network::new();

        _self.layer1.file_to_biases(layer1_bias);
        _self.layer2.file_to_biases(layer2_bias);
        _self.layer1.file_to_weights(layer1_weights);
        _self.layer2.file_to_weights(layer2_weights);
        _self.memory1 = _self.layer1.clone();
        _self.memory2 = _self.layer2.clone();
        _self
    }

    

    pub fn train(
    &mut self,
    duration: u32) {

        for i in 1..=5 {
            let mut game = Game::new();
            let mut score = 0.0;
            let mut out2: SVector<f64, 2> = SVector::<f64, 2>::new(0.0, 0.0);
            game.set_frame(200.0, 200.0);
            while !game.player_is_hit() {
                let first_pipe = game.nearest_pipe();
                let second_pipe = game.second_nearest_pipe();
                let player_height = game.player_height();
                let input = self.data_to_input(first_pipe, second_pipe, player_height);
                let out = self.layer1.forward(input.clone());
                out2 = self.layer2.forward(out);
                let jump = out2.row(0)[0]>out2.row(1)[0];
                if jump {
                    game.jump();
                }
                game.next_state();
                score+=1.0;

                let height: f64 = player_height;
                let velocity: f64 = game.player_velocity();
                let hit_pipe_topline_height: f64 = game.nearest_pipe().3; //y_end_top
                let hit_pipe_botline_height: f64 = game.nearest_pipe().7; //y_end_bot
                let window_height: f64 = 200.0;

                let reward = reward(
                    score,
                    height,
                    velocity,
                    hit_pipe_topline_height,
                    hit_pipe_botline_height,
                    window_height
                );
                
                let first_pipe = game.nearest_pipe();
                let second_pipe = game.second_nearest_pipe();
                let player_height = game.player_height();
                let input = self.data_to_input(first_pipe, second_pipe, player_height);
                
                let outmem = self.memory1.forward(input.clone());
                let mut out2mem = self.memory2.forward(outmem);
                if jump {
                    out2mem.row_mut(0)[0] += reward;
                } else {
                    out2mem.row_mut(1)[0] += reward;
                }
                let expect = out2 + LEARNING_RATE * (out2mem - out2);
                //let expect = SVector::<f64, 2>::new(0.7, 0.3);
                let error = expect-out2;
                //println!("reward:{} : {}", reward, out2);
                let gradient = out2.map(|elem: f64| (SIGMOID.derivative)(&elem));
                let (err, grad) = self.layer2.back_propag(error, gradient);
                (_,_) = self.layer1.back_propag(err, grad);
            }
            println!("round: {}, score: {}, height: {}", i, score, game.player_height());
            self.memory1 = self.layer1.clone();
            self.memory2 = self.layer2.clone();
        }
        self.store_to_file();
    }

    pub fn store_to_file(&mut self) {
        let weights1file = File::create("data/weights_layer1").unwrap();
        let weights2file = File::create("data/weights_layer2").unwrap();
        let bias1file = File::create("data/bias_layer1").unwrap();
        let bias2file = File::create("data/bias_layer2").unwrap();


        self.layer1.weights_to_file(weights1file);
        self.layer2.weights_to_file(weights2file);
        self.layer1.biases_to_file(bias1file);
        self.layer2.biases_to_file(bias2file);
    }

    pub fn replay(&mut self) ->Result<()>{
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;
        term.clear()?;
        let mut last_frame: Instant = Instant::now();
        let mut game = Game::new();
    
        loop {
            if game.player_is_hit() {
                break;
            }
            term.draw(|frame| {
                let area = frame.size();
                game.display(area, frame);
            })?;
    
            if last_frame.elapsed().as_millis()>10 {
                let first_pipe = game.nearest_pipe();
                let second_pipe = game.second_nearest_pipe();
                let player_height = game.player_height();
                let input = self.data_to_input(first_pipe, second_pipe, player_height);
                let out = self.layer1.forward(input.clone());
                let out2 = self.layer2.forward(out);
                //println!("{}", out2.row(0)[0]);
                let jump = out2.row(0)[0]>out2.row(1)[0];
                if jump {
                    game.jump();
                }
                game.next_state();
                last_frame = Instant::now();
            }
        }
    
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn data_to_input(
        &self,
        first_pipe: (f64,f64,f64,f64,f64,f64,f64,f64),
        second_pipe: (f64,f64,f64,f64,f64,f64,f64,f64),
        player_height: f64)->SVector<f64, 17> {
            let x0 = player_height;
            let x1 = first_pipe.0;
            let x2 = first_pipe.1;
            let x3 = first_pipe.2;
            let x4 = first_pipe.3;
            let x5 = first_pipe.4;
            let x6 = first_pipe.5;
            let x7 = first_pipe.6;
            let x8 = first_pipe.7;
            let x9 = second_pipe.0;
            let x10 = second_pipe.1;
            let x11 = second_pipe.2;
            let x12 = second_pipe.3;
            let x13 = second_pipe.4;
            let x14 = second_pipe.5;
            let x15 = second_pipe.6;
            let x16 = second_pipe.7;
            SVector::<f64, 17>::from_column_slice(&[
                x0,
                x1,
                x2,
                x3,
                x4,
                x5,
                x6,
                x7,
                x8,
                x9,
                x10,
                x11,
                x12,
                x13,
                x14,
                x15,
                x16,
            ])

    }
}

const GOAL_SCORE: f64 = 1000.0; // number of frames to survive

const DISTANCE_TO_MIDDLE_IMP:f64 = 0.5;
const GOAL_DISTANCE_IMP:f64 = 0.3;
const DISTANCE_TO_HOLE_IMP:f64 = 0.5;

pub fn reward(
    score: f64,
    height: f64,
    velocity: f64,
    hit_pipe_topline_height: f64,
    hit_pipe_botline_height: f64,
    window_height: f64
    )-> f64 {
    //returns the error
    // if height is closer to middle, good, else bad
    let mut reward = 0.0; //error based on goal score
    // if f64::abs(hit_pipe_topline_height-height)>0.1 {
    //     fitness += velocity * 1.0/(hit_pipe_topline_height-height); // velocity towards pipe->high error, else low error(dodge)
    // }

    // if f64::abs(hit_pipe_topline_height-height)>0.1 {
    //     fitness += velocity * 1.0/(height-hit_pipe_topline_height); // velocity towards pipe->high error, else low error(dodge)
    // }

    reward += velocity * (window_height/2.0 - height) *  DISTANCE_TO_MIDDLE_IMP;
    reward += velocity * ( (hit_pipe_botline_height + hit_pipe_topline_height)/2.0 - height) * DISTANCE_TO_HOLE_IMP;
    1.0/(1.0 + E.powf(-reward))
}
