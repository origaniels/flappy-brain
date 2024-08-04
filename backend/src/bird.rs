use crate::{body::Body, pipe::Pipe};


const BIRD_START_HEIGHT: f64 = 10.0;
const BIRD_START_X: f64 = 10.0;

const BIRD_WIDTH: f64 = 3.0;
const BIRD_MAX_VELOCITY: f64 = 0.3;

const GRAVITY_PULL: f64 = 0.1;
pub struct Bird {
    body: Body,
    velocity: f64
}

impl Bird {
    pub fn new()->Self{
        let body = Body::new(
            BIRD_START_X,
            BIRD_START_HEIGHT,
            BIRD_START_X+BIRD_WIDTH, 
            BIRD_START_HEIGHT+BIRD_WIDTH/2.0);
        Bird {body, velocity: BIRD_MAX_VELOCITY}
    }

    pub fn flap(&mut self) {
        self.velocity = -1.0;
    }

    pub fn is_hit(&self, pipe: &Pipe) ->bool{
        return pipe.collides_with(&self.body);
    }

    pub fn body(&self)->&Body {
        &self.body
    }

    pub fn is_offscreen(&self, height: f64) ->bool{
        self.body.y_end()>height || self.body.y_start()<0.0
    }

    pub fn next_state(&mut self) {
        self.body.move_y(self.velocity);
        if self.velocity<BIRD_MAX_VELOCITY {
            self.velocity = f64::min(self.velocity + GRAVITY_PULL, BIRD_MAX_VELOCITY);
        }
    }

    pub fn x_start(&self)->f64 {
        return self.body.x_start();
    }

    pub fn y_end(&self)->f64 {
        self.body.y_end()
    }

    pub fn velocity(&self) -> f64 {
        self.velocity
    }
}
