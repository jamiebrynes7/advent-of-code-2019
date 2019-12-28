use std::str::FromStr;
use std::error::Error;

type Data = i64;

#[derive(Clone)]
pub struct Machine {
    memory: Vec<Data>,
    ip: usize,
}

pub trait Input {
    fn get(&self) -> Data;
}

pub trait Output {
    fn write(&mut self, val: Data);
}

impl Machine {
    pub fn new(program: &str) -> Result<Machine, Box<dyn Error>> {
        let memory = program.split(',').map(|s| i64::from_str(s)).collect::<Result<Vec<Data>, _>>()?;

        Ok(Machine {
            memory,
            ip: 0
        })
    }

    pub fn set_force(&mut self, addr: usize, val: Data) {
        self.memory[addr] = val;
    }

    /// Reads a value at a given address.
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to read at.
    pub fn read(&self, addr: usize) -> Data {
        let value = self.memory.get(addr);
        *value.unwrap_or_else(|| panic!("Address out of range: {}", addr))
    }

    /// Runs the machine to completion and returns the output.
    ///
    /// * `input` - The input data source.
    /// * `output` - The output data sink.
    pub fn execute<I: Input, O: Output>(&mut self, input: &I, output: &mut O) {
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
    fn step<I: Input, O: Output>(&mut self, input: &I, output: &mut O) -> bool {
        let mut stop = false;

        match self.opcode() {
            1 => {
                self.write(3, self.param(1) + self.param(2));
                self.ip += 4;
            },
            2 => {
                self.write(3, self.param(1) * self.param(2));
                self.ip += 4;
            },
            3 => {
                self.write(1, input.get());
                self.ip += 2;
            },
            4 => {
                output.write(self.param(1));
                self.ip += 2;
            },
            5 => {
                if self.param(1) != 0 {
                    self.ip = self.param(2) as usize;
                } else {
                    self.ip += 3;
                }
            },
            6 => {
                if self.param(1) == 0 {
                    self.ip = self.param(2) as usize;
                } else {
                    self.ip += 3;
                }
            },
            7 => {
                let val = if self.param(1) < self.param(2) { 1 } else { 0 };
                self.write(3, val);
                self.ip += 4;
            },
            8 => {
                let val = if self.param(1) == self.param(2) { 1 } else { 0 };
                self.write(3, val);
                self.ip += 4;
            },
            99 => stop = true,
            _ => panic!("Unknown op code: {}", self.opcode())
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
            Mode::Immediate => (self.ip + offset) as i64
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
            Mode::Immediate => (self.ip + offset) as i64
        };

        self.memory[address as usize] = value;
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
            _ => panic!("Unknown mode encountered: {}", mode)
        }
    }

    /// Returns the current opcode
    fn opcode(&self) -> Data {
        self.read(self.ip) % 100
    }
}

enum Mode {
    Immediate,
    Position
}