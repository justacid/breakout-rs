use piston_window::*;
use paddle::{Paddle, Direction};
use ball::Ball;
use gamesettings::GameSettings;
use brick::Brick;
use na::{Vector2, Vector3};
use rect::Rect;

pub type Vec2 = Vector2<f64>;
pub type Vec3 = Vector3<f64>;

pub struct Game {
    settings: GameSettings,
    paddle: Paddle,
    bricks: Vec<Brick>,
    ball: Ball,
}

impl Game {
    pub fn new(settings: GameSettings) -> Game {
        let paddle_x = settings.board_size.x / 2.0 - settings.paddle_size.x / 2.0;
        let paddle_y = settings.board_size.y - 30.0;

        let paddle = Paddle::new(
            Vec2::new(paddle_x, paddle_y),
            Vec2::new(settings.paddle_size.x, settings.paddle_size.y),
            settings.paddle_color,
            settings.board_size,
            200.0);

        let ball_x = settings.board_size.x / 2.0 - settings.ball_size.x / 2.0;
        let ball = Ball::new(
            Vec2::new(ball_x, paddle_y - 50.0), 
            settings.ball_size,
            settings.ball_color,
            Vec2::new(0.0, 225.0));

        let mut bricks = Vec::new();
        for y in 0..8 {
            for x in 0..7 {
                bricks.push(Brick::new(
                    Vec2::new(50.0 + x as f64 * 55.0, 100.0 + y as f64 * 15.0), 
                    Vec2::new(50.0, 10.0), 
                    Vec3::new(1.0, 1.0, 0.0)));
            }
        }

        Game { 
            settings: settings,
            paddle: paddle, 
            bricks: bricks,
            ball: ball
        }
    }

    pub fn render(&self, c: Context, g: &mut G2d) {
        clear([0.0, 0.0, 0.0, 1.0], g);

        // render borders
        rectangle([self.settings.border_color.x as f32, 
                   self.settings.border_color.y as f32, 
                   self.settings.border_color.z as f32, 1.0],
                  [0.0, 0.0, self.settings.border_thickness, self.settings.board_size.y],
                  c.transform, g);
        rectangle([self.settings.border_color.x as f32, 
                   self.settings.border_color.y as f32, 
                   self.settings.border_color.z as f32, 1.0],
                  [self.settings.board_size.x - self.settings.border_thickness, 
                   0.0,
                   self.settings.border_thickness, self.settings.board_size.y],
                  c.transform, g);
        rectangle([self.settings.border_color.x as f32, 
                   self.settings.border_color.y as f32, 
                   self.settings.border_color.z as f32, 1.0],
                  [0.0, 0.0, self.settings.board_size.x, self.settings.border_thickness],
                  c.transform, g);
        
        self.paddle.render(c, g);
        self.ball.render(c, g);

        for brick in self.bricks.iter() {
            brick.render(c, g);
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.paddle.update(dt);
        self.ball.update(dt);

        let ball_rect = self.ball.get_rect();
        let paddle_rect = self.paddle.get_rect();

        let left_border_rect = Rect::new(0.0, 0.0, 
            self.settings.border_thickness, self.settings.board_size.y);
        let right_border_rect = Rect::new(
            self.settings.board_size.x - self.settings.border_thickness, 
            0.0, self.settings.border_thickness, self.settings.board_size.y);
        let top_border_rect = Rect::new(
            0.0, 0.0, self.settings.board_size.x, self.settings.border_thickness);

        let borders = [left_border_rect, right_border_rect]; 
        let mut new_velocity = self.ball.get_velocity();

        // paddle-ball intersection
        if ball_rect.intersects(paddle_rect) {
            let bcenter = ball_rect.x + ball_rect.width / 2.0;
            let pcenter = paddle_rect.x + paddle_rect.width / 2.0;

            let speed = self.ball.get_velocity();
            let speed_xy = (speed.x*speed.x + speed.y * speed.y).sqrt();
            let pos_x = (bcenter - pcenter) / (paddle_rect.width / 2.0);
            let influence = 0.75;
            new_velocity.x = speed_xy * pos_x * influence;
            new_velocity.y = (speed_xy*speed_xy - new_velocity.x * new_velocity.x).sqrt() * if speed.y > 0.0 { -1.0 } else { 1.0 };
        }
        // ball-left-right-border intersection
        else if borders.iter().any(|border| ball_rect.intersects(border.clone())) {
            new_velocity.x *= -1.0;
        }
        // ball-top-border intersection
        else if ball_rect.intersects(top_border_rect) {
            new_velocity.y *= -1.0;
        }
        // ball-bricks intersection
        else if self.bricks.iter().any(|brick| ball_rect.intersects(brick.get_rect())) {
            new_velocity.y *= -1.0;
            self.bricks.retain(|brick| !ball_rect.intersects(brick.get_rect()));
        }
        
        self.ball.set_velocity(new_velocity);
    }

    pub fn key_pressed(&mut self, key: Key) {
        match key {
            Key::Left => self.paddle.move_dir(Direction::Left),
            Key::Right => self.paddle.move_dir(Direction::Right),
            _ => {}
        }
    }

    pub fn key_released(&mut self, key: Key) {
        match key {
            Key::Left => self.paddle.move_dir(Direction::None),
            Key::Right => self.paddle.move_dir(Direction::None),
            _ => {}
        }
    }
}