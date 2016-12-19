use piston_window::*;
use na::{Vector2, Vector3};
use rect::Rect;

pub type Vec2 = Vector2<f64>;
pub type Vec3 = Vector3<f64>;

pub struct Ball {
    pos: Vec2,
    bounds: Vec2,
    color: Vec3,
    velocity: Vec2,
}

impl Ball {
    pub fn new(pos: Vec2, bounds: Vec2, color: Vec3, velocity: Vec2) -> Ball {
        Ball { pos: pos, bounds: bounds, color: color, velocity: velocity }
    }

    pub fn render(&self, c: Context, g: &mut G2d) {
        rectangle([self.color.x as f32, self.color.y as f32, self.color.z as f32, 1.0],
                  [self.pos.x, self.pos.y, self.bounds.x, self.bounds.y],
                  c.transform, g);
    }

    pub fn update(&mut self, dt: f64) {
        self.pos += self.velocity * dt;
    }

    pub fn get_velocity(&self) -> Vec2 {
        self.velocity.clone()
    }

    pub fn set_velocity(&mut self, velocity: Vec2) {
        self.velocity = velocity;
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