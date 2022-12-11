use std::{collections::VecDeque, str::FromStr, string::ParseError};

#[derive(Debug, Copy, Clone)]
enum WorryModifier {
    IncreaseBy(usize),
    MultiplyBy(usize),
    Square,
}

#[derive(Debug, Copy, Clone)]
struct Operation {
    modifier: WorryModifier,
}

impl Operation {
    fn new(&self, old: usize) -> usize {
        match self.modifier {
            WorryModifier::IncreaseBy(x) => old + x,
            WorryModifier::MultiplyBy(x) => old * x,
            WorryModifier::Square => old * old,
        }
    }
}

impl FromStr for Operation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, parsed_str) = s
            .trim()
            .strip_prefix("Operation: new = ")
            .expect("Should have that prefix split")
            .split_once(" ")
            .unwrap();
        let modifier = match parsed_str.split_once(" ") {
            Some((op, value)) => match (op, value) {
                ("*", "old") => WorryModifier::Square,
                ("*", x) => WorryModifier::MultiplyBy(x.parse::<usize>().unwrap()),
                ("+", x) => WorryModifier::IncreaseBy(x.parse::<usize>().unwrap()),
                _ => unreachable!("Shouldn't happen with {} and {}", op, value),
            },
            _ => unreachable!("Shouldn't happen when parsed string is {parsed_str}"),
        };

        Ok(Operation { modifier })
    }
}

#[derive(Debug, Copy, Clone)]
struct Test {
    divide_by: usize,
    if_true: usize,
    otherwise: usize,
}

impl Test {
    fn determine_target(&self, worry: usize) -> usize {
        match worry % self.divide_by == 0 {
            true => self.if_true,
            false => self.otherwise,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    test: Test,
    items_inspected: usize,
    supermodulo: usize,
}

impl Monkey {
    fn from(s: &str) -> Monkey {
        // NOTE: I feel like there has to be a better way to parse stuff like this
        let mut input = s.split("\n").skip(1);

        // First line is starting items in the order that it will be inspected
        let starting_items = input
            .next()
            .expect("Should ne a next value")
            .trim()
            .strip_prefix("Starting items: ")
            .expect("Should have that prefix split")
            .split(",")
            .filter_map(|f| f.trim().parse::<usize>().ok())
            .collect::<Vec<usize>>();
        // Second line is the function of how the worry level changes after inspection
        let operation = input.next().unwrap().parse::<Operation>().unwrap();

        // Third line is the test that will be performed to evaluate your worry level, which
        // decides which monkey to throw"" the item to
        let divide_by = input
            .next()
            .expect("Should be a next value")
            .trim()
            .strip_prefix("Test: divisible by ")
            .expect("Should have that prefix split")
            .parse::<usize>()
            .expect("Should have that parsed")
            .to_owned();

        // The if/else monkey are line 4-5
        let if_true = input
            .next()
            .expect("Should be next")
            .trim()
            .strip_prefix("If true: throw to monkey ")
            .expect("Should have that prefix split")
            .parse::<usize>()
            .expect("Shoulv have that")
            .to_owned();

        let otherwise = input
            .next()
            .expect("Should have next")
            .trim()
            .strip_prefix("If false: throw to monkey ")
            .expect("Shoud have that prefix split")
            .parse::<usize>()
            .expect("Should have that")
            .to_owned();

        let test = Test {
            divide_by,
            if_true,
            otherwise,
        };

        Monkey {
            items: VecDeque::from(starting_items),
            operation,
            test,
            items_inspected: 0,
            supermodulo: 1,
        }
    }

    fn inspect_item(&mut self) -> Option<(usize, usize)> {
        let item = self.items.pop_front()?;
        self.items_inspected += 1;
        let new_worry_level = self.operation.new(item) % self.supermodulo;
        let target = self.test.determine_target(new_worry_level);

        Some((target, new_worry_level))
    }

    fn add_item(&mut self, item: usize) {
        self.items.push_back(item)
    }
}

struct Flock {
    monkeys: Vec<Monkey>,
}

impl Flock {
    fn from(monkeys: Vec<Monkey>) -> Self {
        Flock { monkeys }
    }

    fn round(&mut self) {
        let num_monkeys = self.monkeys.len();

        for ind in 0..num_monkeys {
            while let Some((target_id, item_to_throw)) = self.monkeys[ind].inspect_item() {
                self.monkeys[target_id].add_item(item_to_throw);
            }
        }
    }

    fn determine_monkey_business(&self) -> usize {
        let mut items_inspected = self
            .monkeys
            .clone()
            .iter()
            .map(|m| m.items_inspected)
            .collect::<Vec<usize>>();
        items_inspected.sort_by(|&a, &b| b.cmp(&a));

        println!("{items_inspected:?}");

        items_inspected.iter().take(2).fold(1, |acc, x| acc * x)
    }
}

fn main() {
    let rounds = 10000;
    let notes = include_str!("../inputs/day11.txt").split("\n\n");
    
    let monkeys: Vec<Monkey> = notes.map(|n| Monkey::from(n)).collect::<Vec<Monkey>>();
    let mut flock = Flock::from(monkeys);
    let supermodulo = flock.monkeys.iter().map(|m| m.test.divide_by).product::<usize>();

    for monkey in flock.monkeys.iter_mut() {
        monkey.supermodulo = supermodulo;
    }

    for _ in 1..=rounds {
        flock.round();
    }
    
    for (idx, monkey) in flock.monkeys.iter().enumerate() {
        println!("Monkey {} inspected {:?} items", idx, monkey.items_inspected);
    }
    println!("Total num items inspected: {}", flock.monkeys.iter().map(|m| m.items_inspected).fold(0, |acc, e| acc + e));

    println!("Monkey business: {}", flock.determine_monkey_business());
}
