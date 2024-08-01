use crate::body::Body;
use rand::Rng;

const PIPE_WIDTH: f64 = 10.;
const PIPE_HOLE: f64 = 15.;


const SCREEN_BORDER_RIGHT: f64 = 200.;

const MOVE_SPEED: f64 = -0.5; // distance to go to when changing to next game state

pub struct Pipe {
    top_pipe: Body,
    bottom_pipe: Body,
}

impl Pipe {
    pub fn new_random(x_start: Option<f64>, max_height: f64)->Self {
        let mut rng = rand::thread_rng();

        let top_pipe_y_end = rng.gen_range(10.0..(max_height-10.0- PIPE_HOLE));
        let x_start: f64 = x_start.unwrap_or(SCREEN_BORDER_RIGHT);

        let top_pipe = Body::new(
            x_start,
            0.0,
            x_start+PIPE_WIDTH,
            top_pipe_y_end);
        let bottom_pipe = Body::new(
            x_start,
            top_pipe_y_end+PIPE_HOLE,
            x_start+PIPE_WIDTH,
            max_height);

        Pipe {top_pipe, bottom_pipe}
    }

    pub fn collides_with(&self, other: &Body)->bool{
        return self.bottom_pipe.collides_with(other) || self.top_pipe.collides_with(other);
    }

    pub fn advance(&mut self) {
        self.bottom_pipe.move_x(MOVE_SPEED);
        self.top_pipe.move_x(MOVE_SPEED);
    }

    pub fn is_offscreen(&self) ->bool{
        return self.top_pipe.x_end()<0.0;
    }

    pub fn x_start(&self)->f64 {
        self.top_pipe.x_start()
    }

    pub fn bottom_pipe(&self) -> &Body {
        &self.top_pipe
    }
    pub fn top_pipe(&self) -> &Body {
        &self.bottom_pipe
    }
}
