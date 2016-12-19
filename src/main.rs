extern crate piston;
extern crate piston_window;
extern crate nalgebra as na;

mod gamesettings;
mod game;
mod paddle;
mod ball;
mod brick;
mod rect;

use piston_window::*;
use na::{Vector2, Vector3};

pub type Vec2 = Vector2<f64>;
pub type Vec3 = Vector3<f64>;

fn main() {
    let mut window: PistonWindow = 
        WindowSettings::new("Breakout-Rs!", [480, 800])
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    let game_settings = gamesettings::GameSettings::new(Vec2::new(480.0, 800.0));
    let mut game = game::Game::new(game_settings);
        
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
