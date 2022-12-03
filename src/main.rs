pub mod day01;
pub mod day02;
pub mod day03;

use clap::Parser;

#[derive(Parser, Debug)]
struct Input {
    #[arg(short)]
    day_chosen: i8,
}

fn main() {
    let args = Input::parse();
    
    println!("Advent of Code 2022");
    println!("###################");
    match args.day_chosen {
        1 => day01::run(),
        2 => day02::run(),
        3 => day03::run(),
        _ => println!("Invalid day chosen.")
    }

}
