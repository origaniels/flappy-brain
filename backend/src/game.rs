use crate::{bird::Bird, pipe::Pipe};

const PIPE_SPACING: f64 = 50.;

pub struct Game {
    pub pipes: Vec<Pipe>,
    pub player: Bird,
    game_over: bool,
    frame_height: f64,
    frame_width: f64,
    already_initialized: bool
}

impl Game {
    pub fn new()->Self {
        let player = Bird::new();
        let pipes: Vec<Pipe> = vec![];
        Game {
            pipes,
            player,
            game_over: false,
            frame_height: 42.0,
            frame_width: 1515.0,
            already_initialized: false
        }
    }

    pub fn jump(&mut self) {
        self.player.flap();
        if self.player_is_hit() {
            self.game_over = true;
        }
    }

    pub fn player_is_hit(&self)->bool{
        for pipe in self.pipes.iter() {
            if self.player.is_hit(&pipe) {
                return true;
            }
        }
        if self.player.is_offscreen(self.frame_height) {
            return true
        }
        false
    }

    pub fn set_frame(&mut self, height: f64, width: f64) {
        if !self.already_initialized {
            self.already_initialized = true;
            self.frame_height = height;
            self.frame_width = width;
        }
         
    }

    pub fn next_state(&mut self) {
        let mut x_start = match self.pipes.len() {
            0=>{
                self.frame_width
            },
            n=>self.pipes[n-1].x_start()
        };
        while self.pipes.len()<10 {
            let new_pipe = Pipe::new_random(Some(x_start + PIPE_SPACING), self.frame_height);
            x_start = new_pipe.x_start();
            self.pipes.push(new_pipe);
        }


        for pipe in self.pipes.iter_mut() {
            pipe.advance();
        }
        self.player.next_state();

        if self.pipes[0].is_offscreen() {
            self.pipes.remove(0);
        }
        if self.player_is_hit() {
            self.game_over = true;
        }
    }

    pub fn nearest_pipe(&self)->(f64, f64, f64, f64, f64, f64, f64, f64) {
        let player_x = self.player.x_start();
        for pipe in self.pipes.iter() {
            if pipe.x_end()>player_x {
                return pipe.data()
            }
        }

        return (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
    }

    pub fn second_nearest_pipe(&self)->(f64, f64, f64, f64, f64, f64, f64, f64) {
        let player_x = self.player.x_start();
        let mut nearest_found = false;
        for pipe in self.pipes.iter() {
            if nearest_found {
                return pipe.data()
            }
            if pipe.x_end()>player_x {
                nearest_found = true;
            }
        }

        return (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
    }

    pub fn player_height(&self)->f64 {
        self.player.y_end()
    }

    pub fn player_velocity(&self)->f64 {
        self.player.velocity()
    }
}
