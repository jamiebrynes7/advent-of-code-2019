use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

const PATH: &str = "src/day1/input.txt";

fn mass_to_fuel(mass: u64) -> u64 {
    mass / 3 - 2
}

fn mass_to_fuel_recursive(mass: u64) -> u64 {
    if mass < 6 {
        return 0;
    }

    let fuel = mass_to_fuel(mass);
    fuel + mass_to_fuel_recursive(fuel)
}

fn part_1(mass: &Vec<u64>) {
    let total_fuel: u64 = mass.iter().map(|mass| mass_to_fuel(*mass)).sum();
    println!("Part 1 answer: {}", total_fuel);
}

fn part_2(mass: &Vec<u64>) {
    let total_fuel: u64 = mass.iter().map(|mass| mass_to_fuel_recursive(*mass)).sum();
    println!("Part 2 answer: {}", total_fuel);
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open(PATH)?;
    let reader = BufReader::new(file);

    let parse_results: Result<Vec<u64>, _> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            u64::from_str(&line)
        })
        .collect();

    let mass = parse_results?;

    part_1(&mass);
    part_2(&mass);

    Ok(())
}
