use core::fmt;

use crate::{ MAP_SIZE_X, MAP_SIZE_Y };
use rand::Rng;
use termion::color::{ self, Fg, Rgb };

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: Position::random_even(),
            y: rng.gen_range(0..MAP_SIZE_Y),
        }
    }

    pub fn eq(a: &Position, b: &Position) -> bool {
        a.x == b.x && a.y == b.y
    }

    pub fn eq_val(a: &Position, x: i32, y: i32) -> bool {
        a.x == x && a.y == y
    }

    fn random_even() -> i32 {
        let mut rng = rand::thread_rng();
        let mut num = rng.gen_range(0..MAP_SIZE_X);
        while num % 2 != 0 {
            num = rng.gen_range(0..MAP_SIZE_X);
        }
        num
    }

    pub fn clone(&self) -> Self {
        Self { x: self.x, y: self.y }
    }
}

#[derive(Debug)]
pub struct Entity {
    pub position: Position,
    pub color: color::Fg<color::Rgb>,
}

pub struct Player {
    pub entity: Entity,
    pub direction: Direction,
    pub tail: Vec<Entity>,
}

impl Player {
    pub fn move_player(&mut self) {
        let mut last = self.entity.position.clone();
        match self.direction {
            Direction::Up => {
                self.entity.position.y -= 1;
            }
            Direction::Down => {
                self.entity.position.y += 1;
            }
            Direction::Left => {
                self.entity.position.x -= 2;
            }
            Direction::Right => {
                self.entity.position.x += 2;
            }
        }
        if !self.tail.is_empty() {
            for i in 0..self.tail.len() {
                let temp = self.tail[i].position.clone();
                self.tail[i].position = last.clone();
                last.x = temp.x;
                last.y = temp.y;
            }
        }
    }

    pub fn grow_tail(&mut self) {
        if self.tail.is_empty() {
            self.tail.push(Entity::new(Position::new(-1, -1), self.entity.color));
            return;
        } else {
            let last = self.tail.last().unwrap();
            self.tail.push(
                Entity::new(Position::new(last.position.x, last.position.y), self.entity.color)
            );
        }
    }
}

impl Entity {
    pub fn new(position: Position, color: Fg<Rgb>) -> Self {
        Self { position, color }
    }
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
