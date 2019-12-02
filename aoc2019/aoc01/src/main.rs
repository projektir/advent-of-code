use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("aoc01/input.txt")?;

    let mut sum: i32 = 0;

    for line in input.lines() {
        let mass: i32 = line.parse::<i32>()?;

        let fuel = fuel(mass);
        sum += fuel;
    }

    println!("{}", sum);

    Ok(())
}

fn fuel(mass: i32) -> i32 {
    let mut fuel_mass = mass / 3 - 2;

    if fuel_mass > 0 {
        fuel_mass += fuel(fuel_mass);
        fuel_mass
    } else {
        0
    }
}
