use piston_window::{Context, G2d, rectangle};
use game::{Vec2, GameColor};
use rect::Rect;

#[derive(Copy, Clone)]
pub struct PaddleInput {
    pub left_pressed: bool,
    pub right_pressed: bool,
}

pub struct Paddle {
    bounds: Rect,
    color: GameColor,
    board_size: Vec2,
    speed: f64,
}

impl Paddle {
    pub fn new(bounds: Rect, color: GameColor, board_size: Vec2, speed: f64) -> Paddle {
        Paddle {
            bounds: bounds,
            color: color,
            board_size: board_size,
            speed: speed,
        }
    }

    pub fn render(&self, c: Context, g: &mut G2d) {
        rectangle(self.color.into(), self.bounds.to_array(), c.transform, g);
    }

    pub fn update(&mut self, dt: f64, input: PaddleInput) {
        if input.left_pressed {
            let new_x = self.bounds.x - self.speed * dt;
            if new_x > 5.0 {
                self.bounds.x -= self.speed * dt;
            }
        }
        if input.right_pressed {
            let new_x = self.bounds.x - self.speed * dt;
            if new_x < self.board_size.x - self.bounds.width - 5.0 {
                self.bounds.x += self.speed * dt;
            }
        }
    }

    pub fn get_bounds(&self) -> Rect {
        self.bounds
    }
}
