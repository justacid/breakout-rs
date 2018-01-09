extern crate piston;
extern crate piston_window;
extern crate glfw_window;
extern crate nalgebra as na;

mod game;
mod objects;
mod rect;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use piston_window::PistonWindow;
use glfw_window::GlfwWindow;
use game::{Vec2, Game, GameSettings, GameColor};

fn main() {
    let mut window: PistonWindow<GlfwWindow> = WindowSettings::new("Breakout-Rs!", [480, 800])
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    let game_settings = GameSettings {
        board_size: Vec2::new(480.0, 800.0),
        ball_size: Vec2::new(5.0, 5.0),
        paddle_size: Vec2::new(75.0, 10.0),
        paddle_color: GameColor::Red,
        ball_color: GameColor::White,
        border_color: GameColor::Cyan,
        brick_color: GameColor::Yellow,
        border_thickness: 5.0,
        brick_rows: 8,
        brick_cols: 7,
    };

    let mut game = Game::new(game_settings);
    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
            if let Some(Button::Keyboard(key)) = e.press_args() {
                game.key_pressed(key);
            };

            if let Some(Button::Keyboard(key)) = e.release_args() {
                game.key_released(key);
            };

            if let Some(_) = e.render_args() {
                window.draw_2d(&e, |c, g| {
                    game.render(c, g);
                });
            };

            if let Some(args) = e.update_args() {
                game.update(args.dt);
            };
    }
}
