use std::f32::consts::E;
use std::thread::{self, sleep};
use std::time::Duration;

use crossterm::event::{KeyCode, KeyEvent};

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
        match last {
            Some(event) => {
                if event == KeyEvent::from(KeyCode::Char('q')) {
                    break;
                } else if event == KeyEvent::from(KeyCode::Char('a'))
                    || event == KeyEvent::from(KeyCode::Left)
                {
                    world.turn_snake(3);
                } else if event == KeyEvent::from(KeyCode::Char('d'))
                    || event == KeyEvent::from(KeyCode::Right)
                {
                    world.turn_snake(1);
                } else if event == KeyEvent::from(KeyCode::Char('w'))
                    || event == KeyEvent::from(KeyCode::Up)
                {
                    world.turn_snake(0);
                } else if event == KeyEvent::from(KeyCode::Char('s'))
                    || event == KeyEvent::from(KeyCode::Down)
                {
                    world.turn_snake(2);
                }
            }
            None => {}
        }
        world.move_snake();
        match world.draw() {
            Err(e) => panic!("Drawing error: {}", e.to_string()),
            Ok(_) => {}
        };
    }
}
