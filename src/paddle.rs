use piston_window::*;
use na::{Vector2, Vector3};
use rect::Rect;

pub type Vec2 = Vector2<f64>;
pub type Vec3 = Vector3<f64>;

pub enum Direction {
    Left,
    Right,
    None
}

pub struct Paddle {
    pos: Vec2,
    bounds: Vec2,
    color: Vec3,
    board_bounds: Vec2,
    speed: f64,
    dir: Direction
}

impl Paddle {
    pub fn new(pos: Vec2, bounds: Vec2, color: Vec3, board_bounds: Vec2, speed: f64) -> Paddle {
        Paddle {
            pos: pos, bounds: bounds, color: color, 
            board_bounds: board_bounds, speed: speed, 
            dir: Direction::None
        }
    }

    pub fn render(&self, c: Context, g: &mut G2d) {
        rectangle([self.color.x as f32, self.color.y as f32, self.color.z as f32, 1.0],
                  [self.pos.x, self.pos.y, self.bounds.x, self.bounds.y],
                  c.transform, g);
    }

    pub fn update(&mut self, dt: f64) {
        match self.dir {
            Direction::Left => {
                if self.pos.x - self.speed * dt > 5.0 {
                    self.pos.x -= self.speed * dt;
                }
            }

            Direction::Right => {
                if self.pos.x + self.speed * dt < self.board_bounds.x - self.bounds.x - 5.0 {
                    self.pos.x += self.speed * dt;
                }
            }

            Direction::None => {}
        }
    }

    pub fn move_dir(&mut self, dir: Direction) {
        self.dir = dir;
    }

    pub fn get_rect(&self) -> Rect {
        Rect {
            x: self.pos.x,
            y: self.pos.y,
            width: self.bounds.x,
            height: self.bounds.y
        }
    }
}