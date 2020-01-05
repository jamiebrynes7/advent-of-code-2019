use machine::Machine;
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const PATH: &str = "src/day9/input.txt";

fn main() -> Result<()> {
    let program = std::fs::read_to_string(PATH)?;
    let machine = Machine::new(&program)?;
    part1(&machine);
    part2(&machine);
    Ok(())
}

fn part1(machine: &Machine) {
    let mut output = Vec::new();
    machine.clone().execute(&mut 1, &mut output);

    println!("OUTPUT: {:?}", output);
}

fn part2(machine: &Machine) {
    let mut output = Vec::new();
    machine.clone().execute(&mut 2, &mut output);

    println!("OUTPUT: {:?}", output);
}
