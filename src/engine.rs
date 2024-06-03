use std::{ io::{ stdout, Write }, thread, time::Duration };

use termion::color::{ self, Fg, Rgb };
use crate::{ entity::{ self, Entity, Player, Position }, MAP_BLOCK, MAP_SIZE_X, MAP_SIZE_Y };

enum GameState {
    Running,
    GameOver,
}

pub struct Engine {
    player: Player,
    food: Entity,
    state: GameState,
    points: u32,
}

const PLAYER_COLOR: Fg<Rgb> = color::Fg(color::Rgb(255, 0, 0));
const FOOD_COLOR: Fg<Rgb> = color::Fg(color::Rgb(0, 255, 0));

impl Engine {
    pub fn new() -> Self {
        Self {
            player: Player {
                entity: Entity::new(Position::new(MAP_SIZE_X / 2, MAP_SIZE_Y / 2), PLAYER_COLOR),
                direction: entity::Direction::Up,
            },
            food: Entity::new(Position::random(), FOOD_COLOR),
            state: GameState::Running,
            points: 0,
        }
    }

    pub fn spawn_food(&mut self) {
        self.food = Entity::new(Position::random(), FOOD_COLOR);
    }

    pub fn draw(&self) {
        std::process::Command::new("clear").status().unwrap();
        for y in 0..MAP_SIZE_Y {
            for x in 0..MAP_SIZE_X {
                if Position::eq_val(&self.food.position, x, y) {
                    print!("{}{}", self.food.color, MAP_BLOCK);
                } else if self.player.entity.position.x == x && self.player.entity.position.y == y {
                    print!("{}{}", PLAYER_COLOR, MAP_BLOCK);
                } else {
                    print!("{}{}", ' ', color::Fg(color::Black));
                }
            }
            println!();
        }
        stdout().flush().unwrap();
    }

    fn move_player(&mut self) {
        let new_position = match self.player.direction {
            entity::Direction::Up =>
                Position::new(self.player.entity.position.x, self.player.entity.position.y - 1),
            entity::Direction::Down =>
                Position::new(self.player.entity.position.x, self.player.entity.position.y + 1),
            entity::Direction::Left =>
                Position::new(self.player.entity.position.x - 1, self.player.entity.position.y),
            entity::Direction::Right =>
                Position::new(self.player.entity.position.x + 1, self.player.entity.position.y),
        };

        if
            new_position.x < 0 ||
            new_position.x >= (MAP_SIZE_X as i32) ||
            new_position.y < 0 ||
            new_position.y >= (MAP_SIZE_Y as i32)
        {
            self.state = GameState::GameOver;
            return;
        }

        if Position::eq(&new_position, &self.food.position) {
            self.points += 1;
            self.spawn_food();
        }

        self.player.entity.position = new_position;
    }

    pub fn run(&mut self) {
        loop {
            self.move_player();
            self.draw();
            thread::sleep(Duration::from_millis(500));
        }
    }
}
