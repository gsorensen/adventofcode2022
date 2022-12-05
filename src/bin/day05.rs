use std::str::FromStr;

#[derive(Debug)]
struct Instruction {
    start_stack: usize,
    target_stack: usize,
    crates_to_move: usize,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_whitespace().take(3);
        let crates_to_move_str = it.next();
        let start_stack_str = it.next();
        let target_stack_str = it.next();

        let (crates_to_move, start_stack, target_stack) =
            match (crates_to_move_str, start_stack_str, target_stack_str) {
                (Some(crates_to_move), Some(start_stack), Some(target_stack)) => {
                    (crates_to_move, start_stack, target_stack)
                }
                _ => unreachable!("Invalid input"),
            };

        let crates_to_move = crates_to_move.parse::<usize>()?;
        let start_stack = start_stack.parse::<usize>()?;
        let target_stack = target_stack.parse::<usize>()?;

        Ok(Instruction {
            start_stack,
            target_stack,
            crates_to_move,
        })
    }
}

#[derive(Debug, Clone)]
struct Stack {
    crates: Vec<char>,
}

impl Stack {
    fn push(&mut self, item: char) {
        self.crates.push(item)
    }

    fn pop_multiple(&mut self, amount: usize) -> Vec<char> {
        let new_len = self.crates.len() - amount;

        self.crates.drain(new_len..).rev().collect::<Vec<char>>()
    }

    fn pop_multiple_in_order(&mut self, amount: usize) -> Vec<char> {
        let new_len = self.crates.len() - amount;

        self.crates.drain(new_len..).collect::<Vec<char>>()
    }
}

#[derive(Debug)]
struct Crane {
    stacks: Vec<Stack>,
}

impl Crane {
    fn from(stacks: Vec<Stack>) -> Crane {
        Crane { stacks }
    }

    fn push_at(&mut self, index: usize, item: char) {
        self.stacks[index].push(item)
    }

    fn pop_multiple_at(&mut self, index: usize, amount: usize) -> Vec<char> {
        self.stacks[index].pop_multiple(amount)
    }

    fn pop_multiple_at_in_order(&mut self, index: usize, amount: usize) -> Vec<char> {
        self.stacks[index].pop_multiple_in_order(amount)
    }

    fn perform_9000(&mut self, instruction: &Instruction) {
        let crates_to_move =
            self.pop_multiple_at(instruction.start_stack - 1, instruction.crates_to_move);

        for c in crates_to_move.iter() {
            self.push_at(instruction.target_stack - 1, *c)
        }
    }

    fn perform_9001(&mut self, instruction: &Instruction) {
        let crates_to_move =
            self.pop_multiple_at_in_order(instruction.start_stack - 1, instruction.crates_to_move);

        for c in crates_to_move.iter() {
            self.push_at(instruction.target_stack - 1, *c);
        }
    }

    fn display_top_crates(&self) -> String {
        self.stacks
            .iter()
            .filter_map(|stack| stack.crates.last())
            .into_iter()
            .collect()
    }
}

impl FromStr for Crane {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.lines().rev();

        let stack_indices = match it.next() {
            Some(line) => line
                .split("")
                .enumerate()
                .filter_map(|(count, item)| match item.parse::<usize>() {
                    Ok(_) => Some(count - 1),
                    _ => None,
                })
                .collect::<Vec<usize>>(),
            None => unreachable!("Shouldn't happen"),
        };

        let mut crane = Crane::from(vec![Stack { crates: vec![] }; stack_indices.len()]);

        while let Some(line) = it.next() {
            for (index, stack_index) in stack_indices.iter().enumerate() {
                let item = line.chars().nth(*stack_index);
                match item {
                    Some(item) if item.is_alphabetic() => crane.push_at(index, item),
                    _ => continue,
                }
            }
        }

        Ok(crane)
    }
}

fn main() -> Result<(), anyhow::Error> {
    let (drawing_str, instructions_str) = include_str!("../inputs/day05.txt")
        .split_once("\n\n")
        .unwrap();

    let mut crane_9000 = drawing_str.parse::<Crane>()?;
    let mut crane_9001 = drawing_str.parse::<Crane>()?;

    let instructions = instructions_str
        .lines()
        .filter_map(|x| {
            x.replace("move ", "")
                .replace("from ", "")
                .replace("to ", "")
                .parse::<Instruction>()
                .ok()
        })
        .collect::<Vec<Instruction>>();

    for instruction in instructions {
        crane_9000.perform_9000(&instruction);
        crane_9001.perform_9001(&instruction);
    }

    let top_crates = crane_9000.display_top_crates();
    let top_crates_9001 = crane_9001.display_top_crates();

    println!(
        "Crates on top of each stack when using the 9000: {}",
        top_crates
    );
    println!(
        "Crates on top of each stack when using the 9001: {}",
        top_crates_9001
    );

    Ok(())
}
