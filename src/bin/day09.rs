use std::{collections::HashSet, str::FromStr, string::ParseError};

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl FromStr for Direction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Direction::Right),
            "L" => Ok(Direction::Left),
            "D" => Ok(Direction::Down),
            "U" => Ok(Direction::Up),
            _ => unreachable!("Couldn't parse direciton"),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    steps: isize,
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(" ") {
            Some((dir_str, step_str)) => {
                let steps = step_str.parse::<isize>().unwrap();
                let direction = dir_str.parse::<Direction>().unwrap();

                Ok(Instruction { direction, steps })
            }
            None => unreachable!("Couldn't parse instruction"),
        }
    }
}

trait Knot {
    fn update_pos(&mut self, pos: (isize, isize));
    fn get_pos(&self) -> (isize, isize);
    fn move_in(&mut self, direction: &Direction);
    fn get_prev_pos(&self) -> &HashSet<(isize, isize)>;
    fn determine_direction_to_move(
        &self,
        head: &Box<dyn Knot>,
        direction: &Direction,
    ) -> Option<Direction>;
}

#[derive(Debug)]
struct Head {
    pos: (isize, isize),
    prev_pos: HashSet<(isize, isize)>,
}

impl Head {
    fn new() -> Head {
        Head {
            pos: (0, 0),
            prev_pos: HashSet::new(),
        }
    }
}

impl Knot for Head {
    fn update_pos(&mut self, pos: (isize, isize)) {
        self.prev_pos.insert(self.pos);
        self.pos = pos;
    }

    fn get_pos(&self) -> (isize, isize) {
        self.pos
    }

    fn move_in(&mut self, direction: &Direction) {
        match direction {
            Direction::Left => self.update_pos((self.pos.0 - 1, self.pos.1)),
            Direction::Right => self.update_pos((self.pos.0 + 1, self.pos.1)),
            Direction::Up => self.update_pos((self.pos.0, self.pos.1 + 1)),
            Direction::Down => self.update_pos((self.pos.0, self.pos.1 - 1)),
            _ => unreachable!("Head can't move diagonal!"),
        }
    }

    fn get_prev_pos(&self) -> &HashSet<(isize, isize)> {
        &self.prev_pos
    }

    fn determine_direction_to_move(
        &self,
        head: &Box<dyn Knot>,
        direction: &Direction,
    ) -> Option<Direction> {
        Some(*direction)
    }
}

#[derive(Debug)]
struct Tail {
    pos: (isize, isize),
    prev_pos: HashSet<(isize, isize)>,
}

impl Tail {
    fn new() -> Tail {
        Tail {
            pos: (0, 0),
            prev_pos: HashSet::new(),
        }
    }
}

impl Knot for Tail {
    fn update_pos(&mut self, pos: (isize, isize)) {
        self.prev_pos.insert(self.pos);
        self.pos = pos;
    }

    fn get_pos(&self) -> (isize, isize) {
        self.pos
    }

    fn move_in(&mut self, direction: &Direction) {
        match direction {
            Direction::Left => self.update_pos((self.pos.0 - 1, self.pos.1)),
            Direction::Right => self.update_pos((self.pos.0 + 1, self.pos.1)),
            Direction::Up => self.update_pos((self.pos.0, self.pos.1 + 1)),
            Direction::Down => self.update_pos((self.pos.0, self.pos.1 - 1)),
            Direction::DownRight => self.update_pos((self.pos.0 + 1, self.pos.1 - 1)),
            Direction::UpRight => self.update_pos((self.pos.0 + 1, self.pos.1 + 1)),
            Direction::DownLeft => self.update_pos((self.pos.0 - 1, self.pos.1 - 1)),
            Direction::UpLeft => self.update_pos((self.pos.0 - 1, self.pos.1 + 1)),
        }
    }

    fn get_prev_pos(&self) -> &HashSet<(isize, isize)> {
        &self.prev_pos
    }

    fn determine_direction_to_move(
        &self,
        head: &Box<dyn Knot>,
        direction: &Direction,
    ) -> Option<Direction> {
        let delta_x = self.pos.0 as isize - head.get_pos().0 as isize;
        let delta_y = self.pos.1 as isize - head.get_pos().1 as isize;

        let direction_to_move = match (delta_x, delta_y) {
            ( 1, 0) | (-1, 0) | (0, 1) | (0, -1) | (1, 1) | (1, -1) | (-1, 1) | (-1, -1) | (0,0) => None,
            (-2, 0) => Some(Direction::Right),
            ( 2, 0) => Some(Direction::Left),
            ( 0, 2) => Some(Direction::Down),
            ( 0,-2) => Some(Direction::Up),
            (-2,-1) | (-2, -2) | (-1, -2) => Some(Direction::UpRight),
            (2,1)   | (2, 2) | (1,2) => Some(Direction::DownLeft),
            (-1,2) | (-2, 2) | (-2, 1) => Some(Direction::DownRight),
            (2,-1) | (2, -2) | (1, -2) => Some(Direction::UpLeft),
            _ => unreachable!("shouldn't happen {}, {}", delta_x, delta_y)
        };

        return direction_to_move;
    }
}

struct Rope {
    knots: Vec<Box<dyn Knot>>,
}

impl Rope {
    fn new(n: usize) -> Rope {
        let mut knots: Vec<Box<dyn Knot>> = vec![];
        knots.push(Box::new(Head::new()));
        for _ in 0..n {
            knots.push(Box::new(Tail::new()));
        }
        Rope { knots }
    }

    fn process_instruction(&mut self, instruction: &Instruction) {
        for _ in 1..=instruction.steps {
            let mut direction_to_move = instruction.direction;
            for ind in 0..self.knots.len() {
                if ind == 0 {
                    self.knots[0].move_in(&direction_to_move);
                } else {
                    match self.knots[ind]
                        .determine_direction_to_move(&self.knots[ind - 1], &direction_to_move)
                    {
                        Some(dir) => {
                            self.knots[ind].move_in(&dir);
                            direction_to_move = dir;
                        },
                        _ => continue,
                    }
                }
            }
        }
    }
}

fn main() {
    let instructions = include_str!("../inputs/day09.txt")
        .lines()
        .map(|s| s.parse::<Instruction>().unwrap())
        .collect::<Vec<Instruction>>();

    let num_tails = 1;
    let mut rope = Rope::new(num_tails);
    for instruction in instructions.iter() {
        rope.process_instruction(&instruction);
    }
    rope.knots[num_tails].update_pos((0, 0));

    println!(
        "Number of unique positions tail visited for 1 tail: {:?}",
        rope.knots[num_tails].get_prev_pos().len()
    );
    let num_tails = 9;
    let mut rope = Rope::new(num_tails);
    for instruction in instructions.iter() {
        rope.process_instruction(&instruction);
    }
    rope.knots[num_tails].update_pos((0, 0));

    println!(
        "Number of unique positions tail visited for last of 9 tails: {:?}",
        rope.knots[num_tails].get_prev_pos().len()
    );
}
