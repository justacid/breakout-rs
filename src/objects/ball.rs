use piston_window::{Context, G2d, rectangle};
use game::{Vec2, GameColor};
use rect::Rect;

pub struct Ball {
    bounds: Rect,
    color: GameColor,
    velocity: Vec2,
}

impl Ball {
    pub fn new(bounds: Rect, color: GameColor, velocity: Vec2) -> Ball {
        Ball {
            bounds: bounds,
            color: color,
            velocity: velocity,
        }
    }

    pub fn render(&self, c: Context, g: &mut G2d) {
        rectangle(self.color.into(), self.bounds.to_array(), c.transform, g);
    }

    pub fn update(&mut self, dt: f64) {
        self.bounds.x += self.velocity.x * dt;
        self.bounds.y += self.velocity.y * dt;
    }

    pub fn get_velocity(&self) -> Vec2 {
        self.velocity
    }

    pub fn set_velocity(&mut self, velocity: Vec2) {
        self.velocity = velocity;
    }

    pub fn get_pos(&self) -> Vec2 {
        self.bounds.get_pos()
    }

    pub fn get_bounds(&self) -> Rect {
        self.bounds
    }
}
