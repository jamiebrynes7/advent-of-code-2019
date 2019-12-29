use std::error::Error;
use machine::Machine;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const PATH: &str = "src/day7/input.txt";

fn main() -> Result<()> {
    let program = std::fs::read_to_string(PATH)?;
    let machine = Machine::new(&program)?;

    part1::exe(&machine);
    part2::exe(&machine);

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

mod part2 {
    use machine::{Machine};
    use crate::permutations;
    use std::{
        sync::{
            atomic::{AtomicBool, Ordering},
            mpsc::{Sender, Receiver, channel, TryRecvError},
            Arc
        },
        thread,
    };

    pub fn exe(machine: &Machine) {
        let perms = permutations(&mut [5, 6, 7, 8, 9], 5, 5);

        let max = perms.iter()
            .map(|p| (p, run_permutation(machine, p)))
            .max_by(|first, second| first.1.cmp(&second.1))
            .unwrap();

        println!("Maximum signal: {} produced by phase permutation: {:?}", max.1, max.0);
    }

    pub fn run_permutation(machine: &Machine, perm: &[i64]) -> i64 {
        // Create N + 1 channels.
        let (mut txs, mut rxs): (Vec<Sender<i64>>, Vec<Receiver<i64>>) = (0..perm.len() + 1).map(|_| channel::<i64>()).unzip();

        // Rotate recv channel forward one place.
        rxs.rotate_right(1);

        // Grab the receiver for the final machine and the sender for the first one.
        let my_rx = rxs.pop().unwrap();
        let my_tx = txs.pop().unwrap();

        // Seed the channels. Note that since we rotated, we need to skip the "first" permutation
        // Since we will seed that one manually.
        my_tx.send(perm[0]).unwrap();
        my_tx.send(0).unwrap();
        for (i, p) in perm.iter().skip(1).enumerate() {
            txs[i].clone().send(*p).unwrap();
        }

        let mut i: usize = 0;
        let trackers = (0..perm.len()).map(|_| Arc::new(AtomicBool::new(false))).collect::<Vec<Arc<AtomicBool>>>();
        for mut rx in rxs {
            let mut m = machine.clone();
            let mut tx = txs[i].clone();
            let tracker = trackers[i].clone();
            thread::spawn(move || {
                m.execute(&mut rx, &mut tx);
                tracker.store(true, Ordering::SeqCst);
            });
            i += 1;
        }

        // While any threads are running..
        let mut output = 0;
        while trackers.iter().any(|b| !b.load(Ordering::SeqCst)) {
            match my_rx.try_recv() {
                Ok(result) => {
                    output = result;

                    // Fire and forget, the receive pipe may be closed if this is the final output.
                    my_tx.send(output);
                },
                Err(e) => match e {
                    TryRecvError::Empty => {},
                    TryRecvError::Disconnected => eprintln!("Pipe disconnected?")
                }
            }
        }

        if let Ok(result) = my_rx.try_recv() {
            output = result;
        }

        output
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
