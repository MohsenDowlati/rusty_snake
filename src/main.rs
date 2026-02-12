extern crate rand;
extern crate piston_window;

mod draw;
mod snake;
mod game;

use piston_window::graphics::clear;
use piston_window::*;
use piston_window::graphics::types::Color;

use game::Game;
use draw::to_coord;

const BACKGROUND_COLOR: Color = [0.5,0.5,0.5,1.0];
fn main() {
   let (width, height) = (30,20);

   let mut window: PistonWindow = WindowSettings::new(
    "Snake",
    [to_coord(width),to_coord(height)],
   ).exit_on_esc(true).build().unwrap();

   let mut game = Game::new(width, height);
   while let Some(event) = window.events.next(&mut window.window) {
       if let Some(args) = event.resize_args() {
           if args.draw_size[0] > 0 && args.draw_size[1] > 0 {
               window.event(&event);
           }
       } else {
           window.event(&event);
       }

       if let Some(Button::Keyboard(key)) = event.press_args() {
           game.key_pressed(key);
       }

       window.draw_2d(&event, |c, g, _device| {
           clear(BACKGROUND_COLOR, g);
           game.draw(&c, g);
       });

       event.update(|arg| {
           game.update(arg.dt);
       });
   }
}
