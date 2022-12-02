use std::fs;

#[derive(Debug)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl Outcome {
    fn determine(player: &Choice, opponent: &Choice) -> Outcome {
        match (player, opponent) {
            (Choice::Rock, Choice::Paper)
            | (Choice::Paper, Choice::Scissors)
            | (Choice::Scissors, Choice::Rock) => Outcome::Loss,
            (Choice::Rock, Choice::Scissors)
            | (Choice::Scissors, Choice::Paper)
            | (Choice::Paper, Choice::Rock) => Outcome::Win,
            _ => Outcome::Draw,
        }
    }

    fn from(s: &str) -> Option<Outcome> {
        let outcome = match s {
            _ if s.contains("X") => Some(Outcome::Loss),
            _ if s.contains("Y") => Some(Outcome::Draw),
            _ if s.contains("Z") => Some(Outcome::Win),
            _ => None,
        }?;

        Some(outcome)
    }
}

#[derive(Debug)]
struct Player {
    choice: Choice,
}

#[derive(Debug)]
struct Opponent {
    choice: Choice,
}

impl Opponent {
    fn from(s: &str) -> Option<Opponent> {
        let choice = match s {
            _ if s.contains("A") => Some(Choice::Rock),
            _ if s.contains("B") => Some(Choice::Paper),
            _ if s.contains("C") => Some(Choice::Scissors),
            _ => None,
        }?;

        Some(Opponent { choice })
    }
}

impl Player {
    fn from(s: &str) -> Option<Player> {
        let choice = match s {
            _ if s.contains("X") => Some(Choice::Rock),
            _ if s.contains("Y") => Some(Choice::Paper),
            _ if s.contains("Z") => Some(Choice::Scissors),
            _ => None,
        }?;

        Some(Player { choice })
    }

    fn determine_choice(opponent_move: &Choice, necessary_outcome: &Outcome) -> Player {
        let choice = match (opponent_move, necessary_outcome) {
            (Choice::Rock, Outcome::Win)
            | (Choice::Scissors, Outcome::Loss)
            | (Choice::Paper, Outcome::Draw) => Choice::Paper,
            (Choice::Rock, Outcome::Draw)
            | (Choice::Paper, Outcome::Loss)
            | (Choice::Scissors, Outcome::Win) => Choice::Rock,
            (Choice::Rock, Outcome::Loss)
            | (Choice::Paper, Outcome::Win)
            | (Choice::Scissors, Outcome::Draw) => Choice::Scissors,
        };

        Player { choice }
    }
}

fn determine_score(player_choice: &Choice, outcome: &Outcome) -> u32 {
    let choice_score = match player_choice {
        &Choice::Rock => 1,
        &Choice::Paper => 2,
        &Choice::Scissors => 3,
    };

    let outcome_score = match outcome {
        &Outcome::Win => 6,
        &Outcome::Draw => 3,
        &Outcome::Loss => 0,
    };

    choice_score + outcome_score
}

pub fn run() {
    println!("Day 02 - Problem 1");
    let contents = fs::read_to_string("src/day02/strategy_guide.txt")
        .expect("Should have been able to read file");
    let data_points = contents.split("\n");

    let player_choices = &data_points
        .clone()
        .filter_map(|x| Player::from(x))
        .map(|p| p.choice)
        .collect::<Vec<Choice>>();

    let opponent_choices = &data_points
        .clone()
        .filter_map(|x| Opponent::from(x))
        .map(|o| o.choice)
        .collect::<Vec<Choice>>();

    let outcomes = &player_choices
        .iter()
        .zip(opponent_choices)
        .map(|(player_choice, opponent_choice)| Outcome::determine(player_choice, opponent_choice))
        .collect::<Vec<Outcome>>();

    let total_score = &player_choices
        .iter()
        .zip(outcomes)
        .map(|(choice, outcome)| determine_score(&choice, &outcome))
        .sum::<u32>();

    println!(
        "Your total score before the encryption scheme is known is {}",
        total_score
    );

    println!("Day 02 - Problem 2");

    let necessary_outcomes = &data_points
        .clone()
        .filter_map(|s| Outcome::from(&s))
        .collect::<Vec<Outcome>>();

    let player_choices = opponent_choices
        .iter()
        .zip(necessary_outcomes)
        .map(|(choice, outcome)| Player::determine_choice(&choice, &outcome))
        .map(|p| p.choice);

    let total_score_new = player_choices
        .zip(necessary_outcomes)
        .map(|(choice, outcome)| determine_score(&choice, &outcome))
        .sum::<u32>();

    println!(
        "Your total score before the encryption scheme is known is {}",
        total_score_new
    );
}
