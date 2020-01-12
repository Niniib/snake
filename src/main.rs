use crate::game::Game;
use crate::point::Point;
use crossterm::terminal::{enable_raw_mode, exit, size};
use crossterm::{cursor, ErrorKind, QueueableCommand};
use std::collections::VecDeque;
use std::io::stdout;

mod game;
mod point;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: snake 30 (FPS)");
        exit();
    }
    clear_screen();
    enable_raw_mode().unwrap();
    stdout().queue(cursor::Hide).unwrap();

    let (columns, rows) = get_terminal_size().unwrap();

    let mut snake = VecDeque::new();
    snake.push_back(Point::new((columns / 2) as i32, (rows / 2) as i32));

    let mut game = Game {
        snake,
        food: Point::new(((columns / 2) + 2) as i32, (rows / 2) as i32),
        size: Point::new(columns as i32, rows as i32),
        velocity: Point::new(0, 0),
    };
    game.velocity = Point { x: -1, y: 0 };
    game.do_loop(u32::from_str_radix(&args[1], 10).unwrap());
}

fn get_terminal_size() -> Result<(u16, u16), ErrorKind> {
    size()
}

pub fn clear_screen() {
    (0..55).for_each(|_| println!());
}
