use std::{fs, collections::HashSet};

#[derive(Debug, PartialEq, Clone)]
struct Item {
    priority: u32,
}

impl Item {
    fn from(c: char) -> Option<Item> {
       let priority = match c {
           // This relates the ASCII value of chars to the mapping we want
           // which is lowercase a is 1 up to uppercase Z being 52
           'a'..='z' => Some(c as u32 - 96),
           'A'..='Z' => Some(c as u32 - 38),
           _ => None
       }?; 
       
       Some(Item {
           priority
       })
    }
}

#[derive(Debug, Clone)]
struct Compartment {
    items: Vec<Item>,
}

impl Compartment {
    fn from(s: &str) -> Option<Compartment> {
        let items = s
            .chars()
            .filter_map(|c| Item::from(c))
            .collect::<Vec<Item>>();
   
        if items.len() == 0 {
            return None
        }

        Some(Compartment {
            items
        })
    }

    fn has_item(&self, item: &Item) -> bool {
        self.items.contains(item)
    }
}

#[derive(Debug, Clone)]
struct Rucksack {
    first_compartment: Compartment,
    second_compartment: Compartment,
}

impl Rucksack {
    fn from(s: &str) -> Option<Rucksack> {
        let compartment_split = s.len()/2;
        let first_compartment = Compartment::from(&s[0..compartment_split])?;
        let second_compartment = Compartment::from(&s[compartment_split..])?;

        Some(Rucksack {
            first_compartment,
            second_compartment
        })
    }

    fn priorities_in_both_compartments(&self) -> Vec<u32> {
        self.first_compartment.items
            .iter()
            .filter(|&i| self.second_compartment.has_item(i))
            .map(|i| i.priority)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<u32>>()
    }

    fn inventory(&self) -> Vec<Item> {
        let mut first = self.first_compartment.items.clone();
        let mut second = self.second_compartment.items.clone();

        first.append(&mut second);

        first
    }

    fn has_item(&self, item: &Item) -> bool {
        self.first_compartment.items.contains(item) || self.second_compartment.items.contains(item)
    } 
}

//
#[derive(Debug)]
struct Group {
    rucksacks: Vec<Rucksack>
}

impl Group {
    fn shared_items_priorities(&self) -> Option<Vec<u32>> {
        if self.rucksacks.len() != 3 {
            return None;
        }
        
        let first = &self.rucksacks[0];
        let second = &self.rucksacks[1];    
        let third = &self.rucksacks[2];

        Some(first.inventory() 
            .into_iter()
            .filter(|item| second.has_item(item))
            .filter(|item| third.has_item(item))
            .map(|item| item.priority)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<u32>>())
    }
}

pub fn main() {
    let data = fs::read_to_string("src/inputs/day03.txt")
        .expect("Should have been able to read input")
        .split("\n")
        .filter_map(|s| Rucksack::from(s))
        .collect::<Vec<Rucksack>>();

    let priority_sum_total = data 
        .iter()
        .map(|rucksack| 
             rucksack
             .priorities_in_both_compartments()
             .into_iter()
             .sum::<u32>())
        .into_iter()
        .sum::<u32>();
    
    println!("Sum of priorities in both compartments: {}", priority_sum_total);

    let group_priority_sum_total = data 
        .chunks(3)
        .map(|chunk| Group {rucksacks: chunk.to_vec()})
        .filter_map(|group| group.shared_items_priorities())
        .map(|priorities|  priorities.into_iter().sum::<u32>())
        .sum::<u32>();

    println!("Sum of priorities for all groups total: {}", group_priority_sum_total);
}
