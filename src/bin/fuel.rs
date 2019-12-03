use structopt::StructOpt;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let args = Cli::from_args();
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);

    let sum: u64  = reader.lines()
        .map(|l| l.unwrap().parse::<u64>().unwrap())
        .map(|mass| required_fuel(mass))
        .sum();

    println!("{}", sum);
    Ok(())
}

/// Fuel required to launch a given module is based on its mass.
/// Specifically, to find the fuel required for a module, take
/// its mass, divide by three, round down, and subtract 2.
fn required_fuel(mass: u64) -> u64 {
    let (mass, overflow) = (mass / 3).overflowing_sub(2);
    if overflow {
        0
    } else {
        mass + required_fuel(mass)
    }
}

#[derive(StructOpt)]
struct Cli {
    path: String,
}
