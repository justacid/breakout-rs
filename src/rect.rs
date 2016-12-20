use game::Vec2;

#[derive(Copy, Clone)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Rect {
        Rect {
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }

    pub fn from_extents(pos: Vec2, extents: Vec2) -> Rect {
        Rect {
            x: pos.x,
            y: pos.y,
            width: extents.x,
            height: extents.y,
        }
    }

    pub fn to_array(&self) -> [f64; 4] {
        [self.x, self.y, self.width, self.height]
    }

    pub fn get_pos(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    pub fn intersects(&self, rect: &Rect) -> bool {
        !((rect.x > self.x + self.width) || (rect.x + rect.width < self.x) ||
          (rect.y > self.y + self.height) || (rect.y + rect.height < self.y))
    }
}
