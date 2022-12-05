use std::fs;

struct ElfInventory {
    food_calories: Vec<u32>,
}

impl Clone for ElfInventory {
    fn clone(&self) -> Self {
        ElfInventory {
            food_calories: self.food_calories.clone(),
        }
    }
}

impl ElfInventory {
    fn from(input: &str) -> ElfInventory {
        ElfInventory {
            food_calories: input
                .split("\n")
                .filter_map(|x| str::parse::<u32>(x).ok())
                .collect(),
        }
    }

    fn calorie_sum(&self) -> u32 {
        self.food_calories.iter().sum()
    }
}

struct Expedition {
    elf_inventories: Vec<ElfInventory>,
}

impl Clone for Expedition {
    fn clone(&self) -> Self {
        Expedition {
            elf_inventories: self.elf_inventories.clone(),
        }
    }
}

impl Expedition {
    fn calorie_sums(&self) -> Vec<u32> {
        self.elf_inventories
            .iter()
            .map(|i| i.calorie_sum())
            .collect()
    }

    fn max_calorie_sum(&self) -> Option<u32> {
        self.calorie_sums().into_iter().max()
    }

    fn max_n_calorie_sums(&self, n: usize) -> Vec<u32> {
        let mut copied_sums = self.calorie_sums();
        copied_sums.sort();
        copied_sums.reverse();
        copied_sums.into_iter().take(n).collect()
    }
}

impl FromIterator<ElfInventory> for Expedition {
    fn from_iter<T: IntoIterator<Item = ElfInventory>>(iter: T) -> Self {
        let mut e = Expedition {
            elf_inventories: vec![],
        };

        for i in iter {
            e.elf_inventories.push(i);
        }

        e
    }
}

pub fn main() {
    let contents =
        fs::read_to_string("src/inputs/day01.txt").expect("Should have been able to read input");

    let expedition = contents
        .split("\n\n")
        .map(|s| ElfInventory::from(s))
        .collect::<Expedition>();

    let max_calories = expedition.max_calorie_sum().expect("Should be a max value");

    println!(
        "Most calories of food carried by one elf: {}",
        max_calories
    );

    let max_three_calorie_sums = expedition.max_n_calorie_sums(3);
    let calorie_total: u32 = max_three_calorie_sums.iter().sum();

    println!(
        "Total amount of calories of food carried by the three hardest-working elves: {}",
        calorie_total
    );
}
