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

    // let event = events::receiver();
    // while world.snake.is_alive {
    //     match event.recv().ok().unwrap() {
    //         Key::Null => {
    //             world.move_snake();
    //         }
    //         Key::Char('a') | Key::Left => {
    //             world.turn_snake(3);
    //             // world.move_snake();
    //             file.write_all("left\n".as_bytes());
    //         }
    //         Key::Char('d') | Key::Right => {
    //             world.turn_snake(1);
    //             // world.move_snake();
    //             file.write_all("right\n".as_bytes());
    //         }
    //         Key::Char('w') | Key::Up => {
    //             world.turn_snake(0);
    //             // world.move_snake();
    //             file.write_all("up\n".as_bytes());
    //         }
    //         Key::Char('s') | Key::Down => {
    //             world.turn_snake(2);
    //             // world.move_snake();
    //             file.write_all("down\n".as_bytes());
    //         }
    //         Key::Char('q') | Key::Ctrl('c') => break,
    //         _ => (),
    //     }
    //     // world.move_snake();
    //     world.draw().ok().unwrap();
    // }

    //     for _i in 0..25 {
    //         world.draw().ok().unwrap();
    //         thread::sleep(time::Duration::from_millis(50));
    //         world.move_snake(None);
    //     }

    // let mut stdin = async_stdin();
    // // let mut stdin = io::stdin();
    // let mut before = Instant::now();
    // let mut last = 1;
    //
    // while world.snake.is_alive {
    //     let mut interval = (25. + 150. / (1. + E.powf(world.speed as f32 / 25.))) as u64;
    //     interval = 300;
    //     let now = Instant::now();
    //     let dt = (now.duration_since(before).subsec_millis()) as u64;
    //
    //     if dt < interval {
    //         sleep(Duration::from_millis(interval - dt));
    //         continue;
    //     }
    //     // sleep(Duration::from_millis(interval));
    //
    //     before = now;
    //
    //     let mut key_bytes = [0];
    //     // let keys = stdin.keys();
    //     stdin.read(&mut key_bytes).unwrap();
    //
    //     let cur = key_bytes[0];
    //
    //     if cur != last {
    //         file.write(&[cur, b' ', last, b'\n']);
    //         match cur {
    //             b'w' => world.turn_snake(0),
    //             b's' => world.turn_snake(2),
    //             b'a' => world.turn_snake(3),
    //             b'd' => world.turn_snake(1),
    //             b'q' => return,
    //             _ => {}
    //         };
    //     }
    //     last = cur;
    //     world.move_snake();
    //
    //     world.draw().ok().unwrap();
    // }
}
