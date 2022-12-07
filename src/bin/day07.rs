use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
struct File {
    size: usize,
    path: String,
}

impl FromStr for File {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let size = match s.split_once(" ") {
            Some((size, _)) => {
                let file_size = size.parse::<usize>()?;
                file_size
            }
            _ => unreachable!("Shouldn't happen"),
        };

        Ok(File {
            size,
            path: String::from("/"),
        })
    }
}

fn main() -> Result<(), anyhow::Error> {
    // This ended up very hacky :)
    let browse_log = std::fs::read_to_string("src/inputs/day07.txt").unwrap();

    let mut commands = browse_log.lines();
    let mut current_directory = vec![];
    let mut directories = HashSet::new();
    let mut files: Vec<File> = vec![];

    while let Some(command) = commands.next() {
        if command.contains("cd ..") {
            current_directory.pop();
        } else if command.contains("cd ") {
            let (_, dir) = command.split_once(" ").unwrap();
            let dir = dir.strip_prefix("cd ").unwrap();
            current_directory.push(dir);
            directories.insert(current_directory.join("/"));
        }

        if command.chars().any(|c| c.is_numeric()) {
            let mut file = command.parse::<File>().unwrap();
            file.path = format!("{}{}", file.path, current_directory.join("/"));
            files.push(file);
        }
    }

    let directory_sizes = &directories
        .into_iter()
        .map(|dir| {
            files
                .iter()
                .filter(|file| file.path.contains(&dir))
                .map(|file| file.size)
                .sum::<usize>()
        })
        .collect::<Vec<usize>>();

    let sum = directory_sizes
        .into_iter()
        .filter(|&s| s <= &100000)
        .sum::<usize>();

    println!(
        "Sum of total sizes of directories that have a size of at most 100 000: {}",
        sum
    );

    const SPACE_AVAILABLE: usize = 70000000;
    let root_dir_space = directory_sizes.iter().max().unwrap();
    let unused_space = SPACE_AVAILABLE - root_dir_space;
    let space_needed = 30000000 - unused_space;

    let mut smallest_size_deletable = directory_sizes
        .iter()
        .filter(|&size| size >= &space_needed)
        .map(|size| *size)
        .collect::<Vec<usize>>();

    smallest_size_deletable.sort();

    println!(
        "Size of smallest directory that can be deleted to free up desired space: {}",
        smallest_size_deletable.first().unwrap()
    );

    Ok(())
}
