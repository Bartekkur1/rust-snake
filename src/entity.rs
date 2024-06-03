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
            x: rng.gen_range(0..=MAP_SIZE_X),
            y: rng.gen_range(0..=MAP_SIZE_Y),
        }
    }

    pub fn eq(a: &Position, b: &Position) -> bool {
        a.x == b.x && a.y == b.y
    }

    pub fn eq_val(a: &Position, x: i32, y: i32) -> bool {
        a.x == x && a.y == y
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
