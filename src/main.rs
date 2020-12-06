use std::io::Read;
use std::{error, fs, result};

use itertools::Itertools;

fn main() -> result::Result<(), Box<dyn error::Error>> {
    let mut buf = String::new();

    fs::File::open("input.txt")?.read_to_string(&mut buf)?;

    let answer1: usize = buf
        .split("\n\n")
        .map(|s| s.chars().filter(|c| *c != '\n').unique().count())
        .sum();

    let answer2: usize = buf
        .split("\n\n")
        .map(|s| {
            let m = s.split('\n').filter(|p| p.len() != 0).count();
            s.chars()
                .filter(|c| *c != '\n')
                .map(|c| (c, c))
                .into_group_map()
                .iter()
                .filter(|(_, v)| v.len() == m)
                .count()
        })
        .sum();

    println!("answer 1 is {}", answer1);
    println!("answer 2 is {}", answer2);
    Ok(())
}
