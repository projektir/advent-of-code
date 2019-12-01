use std::fs::File;
use std::io::{prelude::*, BufReader};

type Result<T> = ::std::result::Result<T, Box<dyn (::std::error::Error)>>;

fn main() -> Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut sum: i32 = 0;

    for line in reader.lines() {
        let mass: i32 = line?.parse::<i32>()?;

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
