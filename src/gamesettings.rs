use na::{Vector2, Vector3};

pub type Vec2 = Vector2<f64>;
pub type Vec3 = Vector3<f64>;

#[derive(Clone)]
pub struct GameSettings {
    pub board_size: Vec2,
    pub ball_size: Vec2,
    pub paddle_size: Vec2,
    pub paddle_color: Vec3,
    pub ball_color: Vec3,
    pub border_color: Vec3,
    pub border_thickness: f64
}

#[allow(dead_code)]
impl GameSettings {
    pub fn new(board_size: Vec2) -> GameSettings {
        GameSettings {
            board_size: board_size,
            ball_size: Vec2::new(5.0, 5.0),
            paddle_size: Vec2::new(75.0, 10.0),
            paddle_color: Vec3::new(1.0, 0.0, 0.0),
            ball_color: Vec3::new(1.0, 1.0, 1.0),
            border_color: Vec3::new(0.0, 1.0, 1.0),
            border_thickness: 5.0
        }
    }

    pub fn ball_size(&mut self, size: Vec2) -> &mut GameSettings {
        self.ball_size = size;
        self
    }

    pub fn paddle_size(&mut self, size: Vec2) -> &mut GameSettings {
        self.paddle_size = size;
        self
    }

    pub fn paddle_color(&mut self, color: Vec3) -> &mut GameSettings {
        self.paddle_color = color;
        self
    }

    pub fn ball_color(&mut self, color: Vec3) -> &mut GameSettings {
        self.ball_color = color;
        self
    }

    pub fn border_color(&mut self, color: Vec3) -> &mut GameSettings {
        self.border_color = color;
        self
    }

    pub fn border_thickness(&mut self, thickness: f64) -> &mut GameSettings {
        self.border_thickness = thickness;
        self
    }
}