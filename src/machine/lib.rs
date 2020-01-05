use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;
use std::sync::mpsc::{Receiver, Sender};

type Data = i64;

#[derive(Clone)]
pub struct Machine {
    memory: Vec<Option<Data>>,
    ip: usize,
    base: Data,
}

pub trait Input {
    fn get(&mut self) -> Data;
}

pub trait Output {
    fn write(&mut self, val: Data);
}

impl Machine {
    pub fn new(program: &str) -> Result<Machine, Box<dyn Error>> {
        let memory = program
            .split(',')
            .map(|s| i64::from_str(s))
            .collect::<Result<Vec<Data>, _>>()?;

        Ok(Machine {
            memory: memory.iter().map(|v| Some(*v)).collect(),
            ip: 0,
            base: 0,
        })
    }

    pub fn set_force(&mut self, addr: usize, val: Data) {
        if self.memory.len() <= addr {
            self.memory.resize(addr + 1, None);
        }

        self.memory[addr] = Some(val);
    }

    /// Reads a value at a given address.
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to read at.
    pub fn read(&self, addr: usize) -> Data {
        let value = self.memory.get(addr);
        value.map_or(0, |opt| opt.unwrap_or(0))
    }

    /// Runs the machine to completion and returns the output.
    ///
    /// * `input` - The input data source.
    /// * `output` - The output data sink.
    pub fn execute<I: Input, O: Output>(&mut self, input: &mut I, output: &mut O) {
        loop {
            if self.step(input, output) {
                break;
            }
        }
    }

    /// Executes a single step of the machine and returns whether the machine halted.
    ///
    /// * `input` - The input data source.
    /// * `output` - The output data sink.
    fn step<I: Input, O: Output>(&mut self, input: &mut I, output: &mut O) -> bool {
        let mut stop = false;

        match self.opcode() {
            1 => {
                self.write(3, self.param(1) + self.param(2));
                self.ip += 4;
            }
            2 => {
                self.write(3, self.param(1) * self.param(2));
                self.ip += 4;
            }
            3 => {
                self.write(1, input.get());
                self.ip += 2;
            }
            4 => {
                output.write(self.param(1));
                self.ip += 2;
            }
            5 => {
                if self.param(1) != 0 {
                    self.ip = self.param(2) as usize;
                } else {
                    self.ip += 3;
                }
            }
            6 => {
                if self.param(1) == 0 {
                    self.ip = self.param(2) as usize;
                } else {
                    self.ip += 3;
                }
            }
            7 => {
                let val = if self.param(1) < self.param(2) { 1 } else { 0 };
                self.write(3, val);
                self.ip += 4;
            }
            8 => {
                let val = if self.param(1) == self.param(2) { 1 } else { 0 };
                self.write(3, val);
                self.ip += 4;
            }
            9 => {
                self.base += self.param(1);
                self.ip += 2;
            }
            99 => stop = true,
            _ => panic!("Unknown op code: {}", self.opcode()),
        }

        stop
    }

    /// Gets the data associated with a given parameter.
    ///
    /// # Arguments
    ///
    /// * `offset` - The offset of the parameter.
    fn param(&self, offset: usize) -> Data {
        let address = match self.mode(offset) {
            Mode::Position => self.read(self.ip + offset),
            Mode::Immediate => (self.ip + offset) as i64,
            Mode::Relative => self.read(self.ip + offset) + self.base,
        };

        self.read(address as usize)
    }

    /// Writes a value using a parameter.
    ///
    /// # Arguments
    ///
    /// * `offset` - The offset of the parameter.
    /// * `value` - The value to write.
    fn write(&mut self, offset: usize, value: Data) {
        let address = match self.mode(offset) {
            Mode::Position => self.read(self.ip + offset),
            Mode::Immediate => (self.ip + offset) as i64,
            Mode::Relative => self.read(self.ip + offset) + self.base,
        };

        if self.memory.len() <= address as usize {
            self.memory.resize(address as usize + 1, None);
        }

        self.memory[address as usize] = Some(value);
    }

    /// Returns the mode of the parameter specified at a given offset.
    ///
    /// # Arguments
    ///
    /// * `offset` - The parameter offset to get the mode for.
    fn mode(&self, offset: usize) -> Mode {
        let op = self.read(self.ip);
        // Typically to get the N-th digit, we would do:
        //      num / (10 ^ n - 1) % 10
        // But we have a 2 digit offset, hence, offset = 1 => N = 3
        let mode = op / (10 as i64).pow(offset as u32 + 1) % 10;

        match mode {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            _ => panic!("Unknown mode encountered: {}", mode),
        }
    }

    /// Returns the current opcode
    fn opcode(&self) -> Data {
        self.read(self.ip) % 100
    }
}

enum Mode {
    Immediate,
    Position,
    Relative,
}

impl Input for i64 {
    fn get(&mut self) -> i64 {
        *self
    }
}

impl Output for i64 {
    fn write(&mut self, val: i64) {
        *self = val;
    }
}

impl Output for Sender<i64> {
    fn write(&mut self, val: i64) {
        self.send(val);
    }
}

impl Output for Vec<i64> {
    fn write(&mut self, val: i64) {
        self.push(val);
    }
}

impl Input for Receiver<i64> {
    fn get(&mut self) -> i64 {
        self.recv().unwrap()
    }
}
