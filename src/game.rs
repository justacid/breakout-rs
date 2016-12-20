use piston_window::{Context, G2d, Key, clear, rectangle};
use na::Vector2;
use objects::*;
use rect::Rect;

pub type Vec2 = Vector2<f64>;

#[derive(Copy, Clone)]
pub enum GameColor {
    Red,
    Yellow,
    Cyan,
    White,
}

impl Into<[f32; 4]> for GameColor {
    fn into(self) -> [f32; 4] {
        match self {
            GameColor::Red => [1.0, 0.0, 0.0, 1.0],
            GameColor::Yellow => [1.0, 1.0, 0.0, 1.0],
            GameColor::Cyan => [0.0, 1.0, 1.0, 1.0],
            GameColor::White => [1.0, 1.0, 1.0, 1.0],
        }
    }
}

#[derive(Copy, Clone)]
pub struct GameSettings {
    pub board_size: Vec2,
    pub ball_size: Vec2,
    pub paddle_size: Vec2,
    pub paddle_color: GameColor,
    pub ball_color: GameColor,
    pub border_color: GameColor,
    pub brick_color: GameColor,
    pub border_thickness: f64,
    pub brick_rows: u32,
    pub brick_cols: u32,
}

pub struct Game {
    settings: GameSettings,
    paddle: Paddle,
    ball: Ball,
    bricks: Vec<Brick>,
    pinput: PaddleInput,
    paused: bool,
    left_border: Rect,
    top_border: Rect,
    right_border: Rect,
}

impl Game {
    pub fn new(settings: GameSettings) -> Game {
        let left = Rect {
            x: 0.0,
            y: 0.0,
            width: settings.border_thickness,
            height: settings.board_size.y,
        };
        let top = Rect {
            x: 0.0,
            y: 0.0,
            width: settings.board_size.x,
            height: settings.border_thickness,
        };
        let right = Rect {
            x: settings.board_size.x - settings.border_thickness,
            y: 0.0,
            width: settings.border_thickness,
            height: settings.board_size.y,
        };

        Game {
            paddle: Game::reset_paddle(&settings),
            ball: Game::reset_ball(&settings),
            bricks: Game::reset_bricks(&settings),
            paused: true,
            pinput: PaddleInput {
                left_pressed: false,
                right_pressed: false,
            },
            settings: settings,
            left_border: left,
            top_border: top,
            right_border: right,
        }
    }

    fn reset_ball(settings: &GameSettings) -> Ball {
        let paddle_y = settings.board_size.y - 30.0;
        let ball_x = settings.board_size.x / 2.0 - settings.ball_size.x / 2.0;
        let position = Vec2::new(ball_x, paddle_y - 50.0);
        Ball::new(Rect::from_extents(position, settings.ball_size),
                  settings.ball_color,
                  Vec2::new(0.0, 225.0))
    }

    fn reset_paddle(settings: &GameSettings) -> Paddle {
        let paddle_x = settings.board_size.x / 2.0 - settings.paddle_size.x / 2.0;
        let paddle_y = settings.board_size.y - 30.0;
        let position = Vec2::new(paddle_x, paddle_y);

        Paddle::new(Rect::from_extents(position, settings.paddle_size),
                    settings.paddle_color,
                    settings.board_size,
                    200.0)
    }

    fn reset_bricks(settings: &GameSettings) -> Vec<Brick> {
        let mut bricks = Vec::new();
        for y in 0..settings.brick_rows {
            for x in 0..settings.brick_cols {
                bricks.push(Brick::new(Rect::new(50.0 + x as f64 * 55.0,
                                                 100.0 + y as f64 * 15.0,
                                                 50.0,
                                                 10.0),
                                       GameColor::Yellow));
            }
        }
        bricks
    }

    pub fn render(&self, c: Context, g: &mut G2d) {
        clear([0.0, 0.0, 0.0, 1.0], g);

        // render borders
        rectangle(GameColor::Cyan.into(),
                  self.left_border.to_array(),
                  c.transform,
                  g);
        rectangle(GameColor::Cyan.into(),
                  self.top_border.to_array(),
                  c.transform,
                  g);
        rectangle(GameColor::Cyan.into(),
                  self.right_border.to_array(),
                  c.transform,
                  g);

        self.paddle.render(c, g);
        self.ball.render(c, g);

        for brick in self.bricks.iter() {
            brick.render(c, g);
        }
    }

    pub fn update(&mut self, dt: f64) {
        // update paddle first, we allow paddle movement even when
        // the game is paused
        self.paddle.update(dt, self.pinput);

        // reset game and pause
        if self.ball.get_pos().y > self.settings.board_size.y + 5.0 {
            self.ball = Game::reset_ball(&self.settings);
            self.paddle = Game::reset_paddle(&self.settings);
            self.bricks = Game::reset_bricks(&self.settings);
            self.paused = true;
        }

        // dont update ball or check collisions if game is paused
        if self.paused {
            return;
        }

        self.ball.update(dt);

        let ball_rect = self.ball.get_bounds();
        let paddle_rect = self.paddle.get_bounds();
        let mut new_velocity = self.ball.get_velocity();

        // paddle-ball intersection
        if ball_rect.intersects(&paddle_rect) {
            let bcenter = ball_rect.x + ball_rect.width / 2.0;
            let pcenter = paddle_rect.x + paddle_rect.width / 2.0;

            let speed = self.ball.get_velocity();
            let speed_xy = (speed.x * speed.x + speed.y * speed.y).sqrt();
            let pos_x = (bcenter - pcenter) / (paddle_rect.width / 2.0);
            let influence = 0.75;
            new_velocity.x = speed_xy * pos_x * influence;
            new_velocity.y = (speed_xy * speed_xy - new_velocity.x * new_velocity.x).sqrt() *
                             if speed.y > 0.0 { -1.0 } else { 1.0 };
        }
        // ball-left-right-border intersection
        else if ball_rect.intersects(&self.left_border) ||
                  ball_rect.intersects(&self.right_border) {
            new_velocity.x *= -1.0;
        }
        // ball-top-border intersection
        else if ball_rect.intersects(&self.top_border) {
            new_velocity.y *= -1.0;
        }
        // ball-bricks intersection
        else if self.bricks.iter().any(|brick| ball_rect.intersects(&brick.get_bounds())) {
            new_velocity.y *= -1.0;
            self.bricks.retain(|brick| !ball_rect.intersects(&brick.get_bounds()));
        }

        self.ball.set_velocity(new_velocity);
    }

    pub fn key_pressed(&mut self, key: Key) {
        match key {
            Key::Left => self.pinput.left_pressed = true,
            Key::Right => self.pinput.right_pressed = true,
            _ => {}
        }
    }

    pub fn key_released(&mut self, key: Key) {
        match key {
            Key::Left => self.pinput.left_pressed = false,
            Key::Right => self.pinput.right_pressed = false,
            Key::Space => self.paused = !self.paused,
            _ => {}
        }
    }
}
