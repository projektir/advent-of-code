use std::io::{self, Read, Write};
use std::io::{prelude::*, BufReader};
use std::fs::File;

type Result<T> = ::std::result::Result<T, Box<dyn (::std::error::Error)>>;

fn main() -> Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        writeln!(io::stdout(), "{}", line?)?;
    }

    Ok(())
}
