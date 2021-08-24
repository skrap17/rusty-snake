use crossterm::event::{poll, read, Event, KeyEvent};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Clone)]
pub struct KeyEventQueue {
    inner_queue: Arc<Mutex<VecDeque<KeyEvent>>>,
}

impl KeyEventQueue {
    pub fn new() -> Self {
        Self {
            inner_queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn get_last_event(&self) -> Option<KeyEvent> {
        let locked_queue = self.inner_queue.lock();
        match locked_queue {
            Ok(mut queue) => {
                let last_element = queue.pop_back();
                queue.clear();
                last_element
            }
            Err(e) => panic!("{}", e.to_string()),
        }
    }

    fn add_event(&self, event: KeyEvent) {
        let locked_queue = self.inner_queue.lock();
        match locked_queue {
            Ok(mut queue) => {
                queue.push_back(event);
            }
            Err(e) => panic!("{}", e.to_string()),
        }
    }

    pub fn send_events(&self) -> crossterm::Result<()> {
        loop {
            if poll(Duration::from_millis(10))? {
                if let Event::Key(event) = read()? {
                    self.add_event(event)
                }
            }
        }
    }
}
