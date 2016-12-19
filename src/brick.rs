use piston_window::*;
use na::{Vector2, Vector3};
use rect::Rect;

pub type Vec2 = Vector2<f64>;
pub type Vec3 = Vector3<f64>;

pub struct Brick {
    pos: Vec2,
    bounds: Vec2,
    color: Vec3
}

impl Brick {
    pub fn new(pos: Vec2, bounds: Vec2, color: Vec3) -> Brick {
        Brick { pos: pos, bounds: bounds, color: color }
    }

    pub fn render(&self, c: Context, g: &mut G2d) {
        rectangle([self.color.x as f32, self.color.y as f32, self.color.z as f32, 1.0],
                  [self.pos.x, self.pos.y, self.bounds.x, self.bounds.y],
                  c.transform, g);
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