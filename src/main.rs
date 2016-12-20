extern crate piston;
extern crate piston_window;
extern crate nalgebra as na;

mod game;
mod objects;
mod rect;

use piston_window::{PistonWindow, WindowSettings, Event, Button, Input};
use game::{Vec2, Game, GameSettings, GameColor};

fn main() {
    let mut window: PistonWindow = 
        WindowSettings::new("Breakout-Rs!", [480, 800])
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
        brick_cols: 7
    };

    let mut game = Game::new(game_settings);
        
    while let Some(e) = window.next() {
        match e {
            Event::Input(Input::Press(Button::Keyboard(key))) => {
                game.key_pressed(key);
            }

            Event::Input(Input::Release(Button::Keyboard(key))) => {
                game.key_released(key);
            }

            Event::Render(_) => {
                window.draw_2d(&e, |c, g| {
                    game.render(c, g);
                });
            }

            Event::Update(args) => { 
                game.update(args.dt);
            }

            _ => {}
        }
    }
}
