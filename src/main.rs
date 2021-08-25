use std::f32::consts::E;
use std::thread::{self, sleep};
use std::time::Duration;

use crossterm::event::KeyCode;

mod events;
mod snake;
mod world;

use events::KeyEventQueue;
use world::World;

const SIZE: (usize, usize) = (20, 20);

const SNAKE_SIZE: usize = 3;

const EAT_REWARD: usize = 1;

fn main() {
    let mut world = World::new(SIZE, SNAKE_SIZE, EAT_REWARD).ok().unwrap();

    let event_queue = KeyEventQueue::new();
    let thread_event_queue = event_queue.clone();
    thread::spawn(move || thread_event_queue.send_events());

    while world.snake.is_alive {
        let interval = (50. + 250. / (1. + E.powf(world.speed as f32 / 25.))) as u64;
        sleep(Duration::from_millis(interval));
        let last = event_queue.get_last_event();
        if let Some(event) = last {
            match event.code {
                KeyCode::Char('q') => break,
                KeyCode::Char(' ') => world.pause(),
                KeyCode::Char('a') | KeyCode::Left => world.turn_snake(3),
                KeyCode::Char('d') | KeyCode::Right => world.turn_snake(1),
                KeyCode::Char('w') | KeyCode::Up => world.turn_snake(0),
                KeyCode::Char('s') | KeyCode::Down => world.turn_snake(2),
                _ => {}
            };
        }
        world.move_snake();
        if let Err(e) = world.draw() {
            panic!("Drawing error: {}", e.to_string())
        }
    }
    world.clean().expect("Error with cleaning the terminal up");
}
