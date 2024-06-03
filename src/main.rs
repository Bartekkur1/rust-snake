mod engine;
mod entity;

pub const MAP_SIZE_X: i32 = 32;
pub const MAP_SIZE_Y: i32 = 16;
pub const MAP_BLOCK: char = '⬤';

fn main() {
    let mut engine = engine::Engine::new();
    engine.run();
}
