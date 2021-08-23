// DIRECTIONS: 0: UP; 1: RIGHT; 2: DOWN; 3: LEFT
const DIRECTION: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub struct Snake {
    pub is_alive: bool,
    pub blocks: Vec<(isize, isize)>,

    // direction is an integer number from 0 to 3
    pub direction: usize,
}

impl Snake {
    // creating a new snake
    pub fn new(head_position: (isize, isize), direction: usize, length: usize) -> Self {
        let mut blocks: Vec<(isize, isize)> = Vec::new();
        blocks.push(head_position);
        let mut pos: (isize, isize) = (head_position.0, head_position.1);
        for _i in 1..length {
            pos.0 = pos.0 - DIRECTION[direction].0;
            pos.1 = pos.1 - DIRECTION[direction].1;
            blocks.push(pos);
        }
        Snake {
            is_alive: true,
            blocks,
            direction,
        }
    }

    // changing the current direction
    pub fn turn(&mut self, action: usize) {
        if (self.direction % 2) != (action % 2) {
            self.direction = action;
        }
    }

    // moving one step in the current direction
    pub fn step(&mut self) -> ((isize, isize), (isize, isize)) {
        let tail: (isize, isize) = self.blocks.pop().unwrap();
        let new_head: (isize, isize) = (
            self.blocks[0].0 + DIRECTION[self.direction].0,
            self.blocks[0].1 + DIRECTION[self.direction].1,
        );
        self.blocks.insert(0, new_head);
        (new_head, tail)
    }
}
