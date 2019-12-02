use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("aoc03/input.txt")?;

    for _line in input.lines() {}

    Ok(())
}
