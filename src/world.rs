use crate::snake::Snake;
use rand::{thread_rng, Rng};
use std::collections::HashSet;
use std::io::{self, Write};
use std::iter::FromIterator;

use crossterm::{
    cursor,
    style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, disable_raw_mode, enable_raw_mode},
    ExecutableCommand, Result,
};

pub struct World {
    pub snake: Snake,
    size: (usize, usize),
    food_position: (isize, isize),
    available_positions: HashSet<(isize, isize)>,
    score: usize,
    pub speed: usize,
    reward: usize,
    out: std::io::Stdout,
    is_pause: bool,
}

impl World {
    // creting new game world
    pub fn new(size: (usize, usize), snake_size: usize, reward: usize) -> io::Result<Self> {
        let direction: usize = thread_rng().gen_range(0..4);
        let x = thread_rng().gen_range(snake_size..size.0 - snake_size - 1) as isize;
        let y = thread_rng().gen_range(snake_size..size.1 - snake_size - 1) as isize;
        let head_position: (isize, isize) = (x, y);
        let mut available_positions: HashSet<(isize, isize)> = HashSet::new();
        for i in 0..size.0 {
            for j in 0..size.1 {
                available_positions.insert((i as isize, j as isize));
            }
        }
        let snake = Snake::new(head_position, direction, snake_size);
        let mut out = io::stdout();
        if let Err(e) = enable_raw_mode() {
            panic!("{}", e.to_string())
        }
        write!(
            out,
            "{}{}",
            cursor::Hide,
            terminal::Clear(terminal::ClearType::All)
        )?;
        let mut world = World {
            snake,
            size,
            food_position: (-1, -1),
            available_positions,
            score: 0,
            speed: 1,
            reward,
            out,
            is_pause: false,
        };
        world.init_food();
        Ok(world)
    }

    // finding available positions and adding food there
    fn init_food(&mut self) {
        if self.snake.is_alive {
            let snake_positions: HashSet<(isize, isize)> =
                self.snake.blocks.iter().cloned().collect::<HashSet<_, _>>();
            let current_available_positions =
                Vec::from_iter(self.available_positions.difference(&snake_positions));
            let idx = thread_rng().gen_range(0..current_available_positions.len());
            self.food_position.0 = current_available_positions[idx].0;
            self.food_position.1 = current_available_positions[idx].1;
        }
    }

    // returns a 2d array that represents the state of each cell in the game
    // 0 - empty
    // 1 - snake body
    // 2 - snake head
    // 3 - food
    fn get_state(&self) -> Vec<Vec<isize>> {
        let mut grid: Vec<Vec<isize>> = Vec::new();
        for _i in 0..self.size.0 {
            let row = vec![0; self.size.1];
            grid.push(row);
        }
        grid[self.food_position.0 as usize][self.food_position.1 as usize] = 3;
        for block in self.snake.blocks.iter() {
            grid[block.0 as usize][block.1 as usize] = 1;
        }
        grid[self.snake.blocks[0].0 as usize][self.snake.blocks[0].1 as usize] = 2;
        grid
    }

    pub fn turn_snake(&mut self, action: usize) {
        if !self.is_pause {
            self.snake.turn(action);
        }
    }

    // turning the snake in the current direction
    // and checking for eating and self-intersections
    pub fn move_snake(&mut self) {
        if self.is_pause {
            return;
        }
        let mut new_food_needed = false;
        if self.snake.is_alive {
            let move_result = self.snake.step();
            let mut new_head = move_result.0;
            let old_tail = move_result.1;
            let size: (isize, isize) = (self.size.0 as isize, self.size.1 as isize);
            new_head.0 = (size.0 + new_head.0) % size.0;
            new_head.1 = (size.1 + new_head.1) % size.1;
            self.snake.blocks[0].0 = new_head.0;
            self.snake.blocks[0].1 = new_head.1;
            if self.snake.blocks[1..].contains(&new_head) {
                self.snake.is_alive = false;
            }
            if new_head == self.food_position {
                self.snake.blocks.push(old_tail);
                new_food_needed = true;
                self.score += self.reward;
                self.speed += 1;
            }
        }
        if new_food_needed {
            self.init_food();
        }
    }

    // rendering the world in terminal
    pub fn draw(&mut self) -> Result<()> {
        if self.is_pause {
            return Ok(());
        }
        let grid = self.get_state();
        self.out.execute(cursor::MoveTo(0, 0))?;
        let wall = format!("{}  {}", SetBackgroundColor(Color::White), ResetColor);
        let plank = wall.repeat(self.size.1 + 2);
        writeln!(self.out, "{}\r", plank)?;
        for i in grid.iter().take(self.size.0) {
            write!(self.out, "{}", wall)?;
            for j in i.iter().take(self.size.1) {
                match j {
                    1 => write!(
                        self.out,
                        "{}  {}",
                        SetBackgroundColor(Color::Green),
                        ResetColor
                    ),
                    2 => write!(
                        self.out,
                        "{}  {}",
                        SetBackgroundColor(Color::DarkGreen),
                        ResetColor
                    ),
                    3 => write!(
                        self.out,
                        "{}  {}",
                        SetBackgroundColor(Color::Red),
                        ResetColor
                    ),
                    _ => write!(self.out, "  "),
                }?;
            }
            writeln!(self.out, "{}\r", wall)?;
        }
        writeln!(self.out, "{}\r", plank)?;
        writeln!(
            self.out,
            "{}{}{}  Your score is: {}\r",
            SetBackgroundColor(Color::White),
            SetForegroundColor(Color::Black),
            cursor::MoveTo(0, 1 + self.size.0 as u16),
            self.score
        )?;
        self.out.flush()?;
        Ok(())
    }

    // cleaning up the terminal
    pub fn clean(&mut self) -> Result<()> {
        self.out.execute(cursor::Show)?;
        disable_raw_mode()?;
        Ok(())
    }

    // pause/unpause
    pub fn pause(&mut self) {
        self.is_pause = !self.is_pause;
    }
}
