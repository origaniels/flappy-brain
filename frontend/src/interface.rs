use std::cmp::{max, min};

use ratatui::{layout::Rect, style::{Color, Style}, widgets::Widget, Frame};

use backend::{body::Body, game::Game};

pub trait Drawable {
    fn display(&mut self, area: Rect, frame: &mut Frame);
}


impl Drawable for Game {
    fn display(&mut self, area: Rect, frame: &mut Frame) {
        self.set_frame(area.height as f64, area.width as f64);

        for pipe in &self.pipes {
            frame.render_widget(
            BodyWidget::new(pipe.top_pipe(), area.height, area.width),
                    area);
            frame.render_widget(
            BodyWidget::new(pipe.bottom_pipe(), area.height, area.width),
                    area);
        }

        frame.render_widget(
            BodyWidget::new(self.player.body(), area.height, area.width),
                    area);
    }
}


pub struct BodyWidget {
    x_start: u16,
    y_start: u16,
    x_end: u16,
    y_end: u16,
}

impl BodyWidget {
    pub fn new(body: &Body, max_height: u16, max_width: u16)->Self {
        let x_start = wrap_into(body.x_start(), max_width);
        let y_start = wrap_into(body.y_start(), max_height);
        let x_end = wrap_into(body.x_end(), max_width);
        let y_end = wrap_into(body.y_end(), max_height);
        BodyWidget {
            x_start,
            y_start,
            x_end,
            y_end
        }
    }
}

pub fn wrap_into(val: f64, _max: u16)->u16 {
    min(max(val as u16, 0), _max)
}

impl Widget for BodyWidget {
    fn render(self, _area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized {
            for i in self.y_start..self.y_end {
                let mut line = String::from("");
                for _ in self.x_start..self.x_end {
                    line.push('█')
                }
                buf.set_string(self.x_start, i, line, Style::default().fg(Color::Red));
            }
    }
}
