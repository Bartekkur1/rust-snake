use std::{ io::{ stdout, Write }, thread, time::Duration };

use device_query::{ DeviceQuery, DeviceState, Keycode };
use termion::color::{ self, Fg, Rgb };
use crate::{
    entity::{ self, Direction, Entity, Player, Position },
    FOOD_BLOCK,
    MAP_SIZE_X,
    MAP_SIZE_Y,
    PLAYER_BLOCK,
};

enum GameState {
    Running,
    GameOver,
}

pub struct Engine {
    player: Player,
    food: Entity,
    state: GameState,
    points: u64,
    running: bool,
    next_render: Duration,
}

const PLAYER_COLOR: Fg<Rgb> = color::Fg(color::Rgb(255, 0, 0));
const FOOD_COLOR: Fg<Rgb> = color::Fg(color::Rgb(0, 255, 0));

impl Engine {
    pub fn new() -> Self {
        Self {
            player: Player {
                entity: Entity::new(Position::new(MAP_SIZE_X / 2, MAP_SIZE_Y / 2), PLAYER_COLOR),
                direction: entity::Direction::Up,
                tail: Vec::new(),
            },
            food: Entity::new(Position::random(), FOOD_COLOR),
            state: GameState::Running,
            points: 0,
            running: true,
            next_render: Duration::from_millis(0),
        }
    }

    pub fn spawn_food(&mut self) {
        let mut food_position = Position::random();
        while
            self.player.entity.position == food_position ||
            self.player.tail.iter().any(|entity| entity.position == food_position)
        {
            food_position = Position::random();
        }
        self.food = Entity::new(Position::random(), FOOD_COLOR);
    }

    fn clear_console() {
        std::process::Command::new("clear").status().unwrap();
    }

    pub fn draw(&self) {
        Self::clear_console();
        println!("Points: {}", self.points);
        for y in 0..MAP_SIZE_Y {
            for x in 0..MAP_SIZE_X {
                if self.food.position == (x, y) {
                    print!("{}{}", self.food.color, FOOD_BLOCK);
                } else if self.player.entity.position == (x, y) {
                    print!("{}{}", PLAYER_COLOR, PLAYER_BLOCK);
                } else if self.player.tail.iter().any(|entity| entity.position == (x, y)) {
                    print!("{}{}", PLAYER_COLOR, PLAYER_BLOCK);
                } else {
                    print!("{}{}", color::Fg(color::Black), '#');
                }
            }
            println!();
        }
        stdout().flush().unwrap();
    }

    fn move_player(&mut self) {
        self.player.move_player();

        if
            self.player.entity.position.x < 0 ||
            self.player.entity.position.x >= (MAP_SIZE_X as i32) ||
            self.player.entity.position.y < 0 ||
            self.player.entity.position.y >= (MAP_SIZE_Y as i32) ||
            self.check_tail_collision()
        {
            self.state = GameState::GameOver;
            return;
        }

        if &self.player.entity.position == &self.food.position {
            self.points += 1;
            self.spawn_food();
            self.player.grow_tail();
        }
    }

    fn check_tail_collision(&self) -> bool {
        self.player.tail.iter().any(|entity| &entity.position == &self.player.entity.position)
    }

    fn handle_input(&mut self, device_state: &DeviceState) {
        let keys: Vec<Keycode> = device_state.get_keys();
        if keys.len() == 0 {
            return;
        }

        if keys.contains(&Keycode::Up) && self.player.direction != Direction::Down {
            self.player.direction = Direction::Up;
        } else if keys.contains(&Keycode::Down) && self.player.direction != Direction::Up {
            self.player.direction = Direction::Down;
        } else if keys.contains(&Keycode::Left) && self.player.direction != Direction::Right {
            self.player.direction = Direction::Left;
        } else if keys.contains(&Keycode::Right) && self.player.direction != Direction::Left {
            self.player.direction = Direction::Right;
        } else if keys.contains(&Keycode::Escape) {
            self.running = false;
        }
    }

    fn check_for_game_over(&mut self) {
        if matches!(self.state, GameState::GameOver) {
            Engine::clear_console();
            println!("Game Over!");
            println!("Points: {}", self.points);
            self.running = false;
        }
    }

    pub fn run(&mut self) {
        let device_state = DeviceState::new();
        while self.running {
            self.handle_input(&device_state);

            if self.next_render <= Duration::from_millis(0) {
                self.move_player();
                self.draw();
                self.next_render = Duration::from_millis(250);
            }

            self.check_for_game_over();
            self.next_render -= Duration::from_millis(10);
            thread::sleep(Duration::from_millis(10));
        }
    }
}
