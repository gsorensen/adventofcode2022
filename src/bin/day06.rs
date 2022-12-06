use std::collections::HashSet;

fn find_first_start_of_marker(
    datastream: &Vec<char>,
    num_distinct_characters: usize,
) -> Option<usize> {
    datastream
        .windows(num_distinct_characters)
        .position(|window| window.iter().collect::<HashSet<_>>().len() == num_distinct_characters)
        .map(|position| position + num_distinct_characters)
}

fn main() -> Result<(), anyhow::Error> {
    let datastream = include_str!("../inputs/day06.txt")
        .chars()
        .collect::<Vec<char>>();

    let first_start_of_packet_marker = match find_first_start_of_marker(&datastream, 4) {
        Some(value) => value,
        _ => unreachable!("Shouldn't happen. Couldn't find start of packet marker"),
    };

    let first_start_of_message_marker = match find_first_start_of_marker(&datastream, 14) {
        Some(value) => value,
        _ => unreachable!("Shouldn't happen. Couldn't find start of message marker"),
    };

    println!(
        "Start of first packet marker: {}",
        first_start_of_packet_marker
    );
    println!(
        "Start of first message marker: {}",
        first_start_of_message_marker
    );

    Ok(())
}
