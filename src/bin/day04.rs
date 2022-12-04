use anyhow;
use std::str::FromStr;

#[derive(Debug)]
struct SectionAssignment {
    lower: usize,
    upper: usize,
}

impl FromStr for SectionAssignment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lower, upper) = match s.split_once("-") {
            Some((lower, upper)) => (lower, upper),
            None => unreachable!("Invalid input"),
        };

        let lower = lower.parse::<usize>()?;
        let upper = upper.parse::<usize>()?;

        Ok(SectionAssignment { lower, upper })
    }
}

impl SectionAssignment {
    fn fully_contained_in(&self, other: &SectionAssignment) -> bool {
        self.lower >= other.lower && self.upper <= other.upper
    }

    fn partially_contained_in(&self, other: &SectionAssignment) -> bool {
        (self.lower >= other.lower && self.lower <= other.upper)
            || (self.upper <= other.upper && self.lower >= other.lower)
    }
}

#[derive(Debug)]
struct AssignmentPair {
    first: SectionAssignment,
    second: SectionAssignment,
}

impl FromIterator<SectionAssignment> for AssignmentPair {
    fn from_iter<T: IntoIterator<Item = SectionAssignment>>(iter: T) -> Self {
        let mut it = iter.into_iter();
        let first = it.next();
        let second = it.next();

        let (first, second) = match (first, second) {
            (Some(first), Some(second)) => (first, second),
            _ => unreachable!("Should not happen, input must be very wrong"),
        };

        AssignmentPair { first, second }
    }
}

fn main() {
    let assignment_pairs = include_str!("../inputs/day04.txt")
        .lines()
        .map(|line| {
            line.split(",")
                .filter_map(|range| range.parse::<SectionAssignment>().ok())
                .collect::<AssignmentPair>()
        })
        .collect::<Vec<AssignmentPair>>();

    let pairs_with_fully_contained_ranges = assignment_pairs
        .iter()
        .filter(|pair| {
            pair.first.fully_contained_in(&pair.second)
                || pair.second.fully_contained_in(&pair.first)
        })
        .count();

    println!(
        "Number of pairs where one range is fully contained in the other: {}",
        pairs_with_fully_contained_ranges
    );

    let pairs_with_partially_contained_ranges = assignment_pairs
        .iter()
        .filter(|pair| {
            pair.first.partially_contained_in(&pair.second)
                || pair.second.partially_contained_in(&pair.first)
        })
        .count();

    println!(
        "Number of pairs where one range is partially contained in the other: {}",
        pairs_with_partially_contained_ranges
    );
}
