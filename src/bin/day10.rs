use std::{collections::VecDeque, str::FromStr, string::ParseError};
 
#[derive(Debug)]
enum InstructionType {
    AddX(i32),
    NoOp,
}

impl FromStr for InstructionType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(" ") {
            Some((_, val)) => match val.parse::<i32>() {
                Ok(add_value) => Ok(InstructionType::AddX(add_value)),
                Err(_) => unreachable!("Should be able to parse AddX operation"),
            },
            None => match s {
                "noop" => Ok(InstructionType::NoOp),
                _ => unreachable!("Should not happen with provided input"),
            },
        }
    }
}

#[derive(Debug)]
struct Instruction {
    instruction_type: InstructionType,
    cycle_length: usize,
}

impl Instruction {
    fn from(instruction_type: InstructionType) -> Instruction {
        let cycle_length = match instruction_type {
            InstructionType::AddX(_) => 2,
            InstructionType::NoOp => 1,
        };

        Instruction {
            instruction_type,
            cycle_length,
        }
    }
}

#[derive(Debug)]
struct CPU {
    queue: VecDeque<Instruction>,
    active_instruction: Instruction,
    current_cycle: usize,
    register: i32,
}

impl CPU {
    fn new() -> CPU {
        CPU {
            queue: VecDeque::from(vec![]),
            active_instruction: Instruction {
                instruction_type: InstructionType::NoOp,
                cycle_length: 0,
            },
            current_cycle: 0,
            register: 1,
        }
    }

    fn add_to_queue(&mut self, instruction: Instruction) {
        self.queue.push_back(instruction)
    }

    fn add_to_register(&mut self, x: i32) {
        self.register += x;
    }

    fn has_instructions(&self) -> bool {
        self.queue.len() > 0 || self.active_instruction.cycle_length > 0
    }

    fn tick(&mut self) {
        self.current_cycle += 1;

        if self.active_instruction.cycle_length == 0 {
            match self.active_instruction.instruction_type {
                InstructionType::AddX(value) => self.add_to_register(value),
                InstructionType::NoOp => (),
            };

            if let Some(instruction) = self.queue.pop_front() {
                self.active_instruction = instruction;
            }
        }

        self.active_instruction.cycle_length -= 1;
    }

    fn compute_signal_strength(&self) -> i32 {
        self.current_cycle as i32 * self.register
    }
}

struct CRT {
    pixels: Vec<Vec<String>>,
    sprite_pos: i32,
}

impl CRT {
    fn new() -> CRT {
        CRT {
            pixels: vec![vec![String::from("."); 40]; 6],
            sprite_pos: 1,
        }
    }

    fn draw(&mut self, cpu_cycle: usize) {
        let y_pos = self.get_y_pos(cpu_cycle);
        let x_pos = cpu_cycle - (y_pos - 1) * 40;

        let symbol = match self.is_sprite_visible(x_pos as i32 - 1) {
            true => "#",
            false => "."
        };

        self.pixels[y_pos - 1][x_pos - 1] = String::from(symbol);
    }

    fn draw_finished(&self) {
        for row in &self.pixels {
            for symbol in row {
                print!("{} ", symbol);
            }
            println!("\n");
        }
    }

    fn get_y_pos(&self, cpu_cycle: usize) -> usize {
        match cpu_cycle {
            1..=40 => 1,
            41..=80 => 2,
            81..=120 => 3,
            121..=160 => 4,
            161..=200 => 5,
            201..=240 => 6,
            _ => unreachable!("no"),
        }
    }

    fn is_sprite_visible(&self, crt_x_draw_pos: i32) -> bool {
        if crt_x_draw_pos == self.sprite_pos
            || crt_x_draw_pos == self.sprite_pos + 1
            || crt_x_draw_pos == self.sprite_pos - 1
        {
            return true;
        }
        return false;
    }

    fn update_sprite_pos(&mut self, pos: i32) {
        self.sprite_pos = pos
    }
}

fn main() {
    let instructions = include_str!("../inputs/day10.txt")
        .lines()
        .into_iter()
        .filter_map(|s| s.parse::<InstructionType>().ok())
        .map(|i| Instruction::from(i));

    let mut cpu = CPU::new();
    let mut crt = CRT::new();
    
    // Add all instructions to queue
    for instruction in instructions {
        cpu.add_to_queue(instruction);
    }

    let mut signal_strengths = vec![];
    let cycles_of_note = vec![20, 60, 100, 140, 180, 220];
    
    while cpu.has_instructions() {
        cpu.tick();
        crt.update_sprite_pos(cpu.register);
        crt.draw(cpu.current_cycle);
        if cycles_of_note.contains(&cpu.current_cycle) {
            signal_strengths.push(cpu.compute_signal_strength());
        }
    }

    println!(
        "Sum of the six signal strengths: {:?}",
        signal_strengths.iter().sum::<i32>()
    );

    crt.draw_finished();
}
