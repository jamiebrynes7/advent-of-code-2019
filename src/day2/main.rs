use std::error::Error;
use std::fs::File;
use std::io::Read;
use machine::{Machine, Input, Output};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const PATH: &str = "src/day2/input.txt";

fn main() -> Result<()> {
    let mut program = String::new();
    File::open(PATH)?.read_to_string(&mut program)?;

    let machine = Machine::new(&program)?;

    part_1(machine.clone());
    part_2(machine.clone());
    Ok(())
}

fn part_1(mut machine: Machine) {
    machine.set_force(1, 12);
    machine.set_force(2, 2);
    machine.execute(&InputSource{}, &mut OutputSink{});
    println!("{}", machine.read(0));
}

fn part_2(mut machine: Machine) {
    const TARGET: i64 = 19690720;

    for noun in 0..99 {
        for verb in 0..99 {
            let mut machine = machine.clone();
            machine.set_force(1, noun);
            machine.set_force(2, verb);
            machine.execute(&InputSource{}, &mut OutputSink{});
            if machine.read(0) == TARGET {
                println!("{}", 100 * noun + verb);
                return;
            }
        }
    }

    println!("Could not find combination")
}

struct InputSource {}

impl Input for InputSource {
    fn get(&self) -> i64 {
        0
    }
}

struct OutputSink {}

impl Output for OutputSink {
    fn write(&mut self, val: i64) {}
}
