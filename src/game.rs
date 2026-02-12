use piston_window::*;
use piston_window::graphics::Context;
use piston_window::graphics::types::Color;
use piston_window::wgpu_graphics::WgpuGraphics;

use rand::{rng, RngExt};

use crate::snake::{Direction, Snake};
use crate::draw::{draw_block, draw_rectangle};

const APPLE_COLOR: Color = [0.80, 0.00, 0.00,1.00];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.00];
const GAME_OVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: Snake,

    apple_exists: bool,
    apple_x: i32,
    apple_y: i32,

    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game { 
            snake: Snake::new(2,2),
            apple_exists: true, 
            apple_x: 6, 
            apple_y: 4, 
            width, 
            height, 
            game_over: false, 
            waiting_time: 0.0 
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Right => Some(Direction::Right),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            _ => None
        };

        if let Some(d) = dir {
            if d == self.snake.head_direction().opposite() {
                return;
            }
            self.update_snake(Some(d));
        }
    }

    pub fn draw(&self, con: &Context, g: &mut WgpuGraphics) {
        self.snake.draw(con,g);

        if self.apple_exists {
            draw_block(APPLE_COLOR, self.apple_x, self.apple_y, con, g);
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1 , self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height , con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1 , self.height, con, g);

        if self.game_over {
            draw_rectangle(GAME_OVER_COLOR, 0, 0, self.width, self.height, con, g);
        }

    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.apple_exists {
            self.add_apple();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    fn check_eating(&mut self) {
        let (head_x, head_y): (i32,i32) = self.snake.head_position();
        if self.apple_exists && self.apple_x == head_x && self.apple_y == head_y {
            self.apple_exists = false;
            self.snake.restore_tail();
        }
    }

    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height -1
    }

    fn add_apple(&mut self) {
        let mut rng = rng();

        let mut new_x = rng.random_range(1..self.width - 1);
        let mut new_y = rng.random_range(1..self.height - 1);

        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.random_range(1..self.width - 1);
            new_y = rng.random_range(1..self.height - 1);
        }

        self.apple_x = new_x;
        self.apple_y = new_y;
        self.apple_exists = true;
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }


    fn restart(&mut self) {
        self.snake = Snake::new(2,2);
        self.waiting_time = 0.0;
        self.apple_exists = true;
        self.apple_x = 6;
        self.apple_y = 4;
        self.game_over = false;
    }
}
