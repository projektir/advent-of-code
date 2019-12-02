use std::fs::File;
use std::io::{prelude::*, BufReader};

type Result<T> = std::result::Result<T, Box<dyn (std::error::Error)>>;

fn main() -> Result<()> {
    let file = File::open("aoc03/input.txt")?;
    let reader = BufReader::new(file);

    let mut sum: i32 = 0;

    for line in reader.lines() {}

    Ok(())
}
