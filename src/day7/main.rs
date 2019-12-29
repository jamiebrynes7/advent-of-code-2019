use std::error::Error;
use machine::{Machine, Input};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const PATH: &str = "src/day7/input.txt";

fn main() -> Result<()> {
    let program = std::fs::read_to_string(PATH)?;
    let machine = Machine::new(&program)?;

    part1::exe(&machine);

    Ok(())
}

mod part1 {
    use machine::{Input, Machine};
    use crate::permutations;

    pub fn exe(machine: &Machine) {
        let perms = permutations(&mut [0, 1, 2, 3, 4], 5, 5);

        let max = perms.iter()
            .map(|p| (p, run_permutation(machine, p)))
            .max_by(|first, second| first.1.cmp(&second.1))
            .unwrap();

        println!("Maximum signal: {} produced by phase permutation: {:?}", max.1, max.0);
    }

    fn run_permutation(machine: &Machine, perm: &[i64]) -> i64 {
        let mut output = 0;

        for phase in perm {
            let mut input = AmplifierSource::new(*phase, output);
            machine.clone().execute(&mut input, &mut output);
        }

        output
    }

    struct AmplifierSource {
        pub phase: Option<i64>,
        pub input: i64
    }

    impl AmplifierSource {
        pub fn new(phase: i64, input: i64) -> Self {
            AmplifierSource {
                phase: Some(phase),
                input
            }
        }
    }

    impl Input for AmplifierSource {
        fn get(&mut self) -> i64 {
            self.phase.take().unwrap_or(self.input)
        }
    }
}

fn permutations(sequence: &mut [i64], size: usize, n: usize) -> Vec<Vec<i64>> {
    if size == 1 {
        return vec![sequence.to_vec()];
    }

    let mut results = vec![];

    for i in 0..size {
        results.append(&mut permutations(sequence, size - 1, n));

        if size % 2 == 1 {
            sequence.swap(0, size - 1);
        } else {
            sequence.swap(i, size - 1);
        }
    }

    results
}
