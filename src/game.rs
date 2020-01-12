use crate::point::Point;
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use crossterm::terminal::disable_raw_mode;
use crossterm::{cursor, QueueableCommand};
use rand::Rng;
use std::collections::VecDeque;
use std::io::{stdout, Write};
use std::process::exit;
use std::thread;
use std::time::Duration;

pub struct Game {
    pub snake: VecDeque<Point>,
    pub food: Point,
    pub size: Point,
    pub velocity: Point,
}

impl Game {
    fn logic(&mut self) {
        let mut head = self.snake[0];
        head = head.add(self.velocity);
        if self.snake.contains(&head) {
            Game::quit();
        }
        self.snake.push_front(head);

        if head.x() > self.size.x() || head.y() > self.size.y() || head.x() < 0 || head.y() < 0 {
            Game::quit();
        }
        if head.x() == self.food.x() && head.y() == self.food.y() {
            let random_x = Game::random_in_range(1, self.size.x());
            let random_y = Game::random_in_range(1, self.size.y());
            self.food = Point {
                x: random_x,
                y: random_y,
            };
        } else {
            self.snake.pop_back();
        }
    }

    fn random_in_range(min: i32, max: i32) -> i32 {
        rand::thread_rng().gen_range(min, max)
    }

    fn quit() {
        disable_raw_mode().unwrap();
        stdout().queue(cursor::Show).unwrap();
        println!("Game Over!");
        exit(0);
    }

    fn draw(&self) {
        for x in 0..self.size.x {
            for y in 0..self.size.y {
                let p: Point = Point { x, y };
                stdout().queue(cursor::MoveTo(x as u16, y as u16)).unwrap();
                if self.snake.contains(&p) {
                    print!("O");
                } else if self.food.x() == p.x() && self.food.y == p.y() {
                    print!("*");
                } else {
                    print!(" ");
                }
                stdout().flush().unwrap();
            }
        }
    }

    fn input(&mut self) {
        if poll(Duration::from_millis(50)).unwrap() {
            match read().unwrap() {
                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    ..
                }) => {
                    self.velocity = Point { x: 0, y: 1 };
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Up, ..
                }) => {
                    self.velocity = Point { x: 0, y: -1 };
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    ..
                }) => {
                    self.velocity = Point { x: -1, y: 0 };
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    ..
                }) => {
                    self.velocity = Point { x: 1, y: 0 };
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                }) => {
                    Game::quit();
                }
                _ => {}
            }
        }
    }

    pub fn do_loop(&mut self, fps: u32) {
        loop {
            self.input();
            self.logic();
            self.draw();
            thread::sleep(Duration::from_secs_f64(1.0 / fps as f64));
        }
    }
}
