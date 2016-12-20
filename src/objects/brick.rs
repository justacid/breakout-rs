use piston_window::{Context, G2d, rectangle};
use game::GameColor;
use rect::Rect;

pub struct Brick {
    bounds: Rect,
    color: GameColor
}

impl Brick {
    pub fn new(bounds: Rect, color: GameColor) -> Brick {
        Brick { bounds: bounds, color: color }
    }

    pub fn render(&self, c: Context, g: &mut G2d) {
        rectangle(self.color.into(), self.bounds.to_array(), c.transform, g);
    }

    pub fn get_bounds(&self) -> Rect {
        self.bounds
    }
}