use std::error::Error;
use std::fs::File;
use std::io::Read;
use machine::{Machine, Input, Output};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const PATH: &str = "src/day5/input.txt";

fn main() -> Result<()> {
    let mut program = String::new();
    File::open(PATH)?.read_to_string(&mut program)?;

    let machine = Machine::new(&program)?;
    part_1(machine.clone());
    part_2(machine);
    Ok(())
}

fn part_1(mut machine: Machine) {
    let mut output = OutputSource::default();
    machine.execute(&InputSource{ id: 1 }, &mut output);

    if !output.was_success() {
        println!("Tests failed!");
        return;
    }

    println!("Code: {}", output.code())
}

fn part_2(mut machine: Machine) {
    let mut output = OutputSource::default();
    machine.execute(&InputSource { id: 5}, &mut output);

    println!("Code: {}", output.code())
}

struct InputSource { pub id: i64 }

impl Input for InputSource {
    fn get(&self) -> i64 {
        self.id
    }
}

#[derive(Default)]
struct OutputSource {
    results: Vec<i64>
}

impl OutputSource {
    pub fn code(&self) -> i64 {
        *self.results.last().unwrap_or(&-1)
    }

    pub fn was_success(&self) -> bool {
        self.results[0..self.results.len() - 1].iter().all(|v| *v == 0)
    }
}

impl Output for OutputSource {
    fn write(&mut self, val: i64) {
        self.results.push(val)
    }
}