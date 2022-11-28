pub mod day01;

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
        _ => println!("Invalid day chosen.")
    }

}
