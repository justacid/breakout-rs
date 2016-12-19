
#[derive(Copy, Clone)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64
}

impl Rect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Rect {
        Rect { x: x, y: y, width: width, height: height }
    } 

    pub fn intersects(&self, rect: Rect) -> bool {
        !((rect.x > self.x + self.width) ||
        (rect.x + rect.width < self.x) ||
        (rect.y > self.y + self.height) ||
        (rect.y + rect.height < self.y))
    }
}