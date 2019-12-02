use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const PATH: &str = "src/day2/input.txt";

fn main() -> Result<()> {

    let op_list = OpList::new(PATH)?;

    part_1(&op_list)?;
    part_2(&op_list)?;
    Ok(())
}

fn part_1(op_list: &OpList) -> Result<()> {
    let mut op_list = op_list.clone();
    op_list.set(1, 12)?;
    op_list.set(2, 2)?;
    println!("{}", execute_program(op_list)?);
    Ok(())
}

fn part_2(op_list: &OpList) -> Result<()>{
    const TARGET: usize = 19690720;

    for noun in 0..99 {
        for verb in 0..99 {
            let mut op_list = op_list.clone();
            op_list.set(1, noun)?;
            op_list.set(2, verb)?;
            if execute_program(op_list)? == TARGET {
                println!("{}", 100 * noun + verb);
                return Ok(());
            }
        }
    }

    Err("Could not find combo".into())
}

fn execute_program(mut op_list: OpList) -> Result<usize> {
    loop {
        match op_list.next_op()? {
            OpCode::Add(add) => {
                let res = add.arg1 + add.arg2;
                op_list.set(add.result, res)?;
            },
            OpCode::Multiply(mul) => {
                let res = mul.arg1 * mul.arg2;
                op_list.set(mul.result, res)?;
            },
            OpCode::Halt => {
                break
            }
        }
    }

    Ok(op_list.get(0).unwrap())
}

struct OpList {
    data: Vec<usize>,
    op_pointer: usize
}

impl Clone for OpList {
    fn clone(&self) -> Self {
        OpList {
            data: self.data.clone(),
            op_pointer: self.op_pointer
        }
    }
}

impl OpList {
    pub fn new(path: &str) -> Result<Self> {
        let mut data: String = String::new();
        let mut file = File::open(path)?;
        File::read_to_string(&mut file, &mut data)?;

        let data : std::result::Result<Vec<usize>, _> = data.split(",")
            .map(|s| usize::from_str(&s))
            .collect();

        Ok(OpList {
            data: data?,
            op_pointer: 0
        })
    }

    pub fn next_op(&mut self) -> Result<OpCode> {
        if self.op_pointer > self.data.len() {
            return Err("Op pointer exceeded data length".into());
        }

        let val = self.get(self.op_pointer).unwrap();

        match val {
            1 => Ok(OpCode::Add(OpInfo::new(self))),
            2 => Ok(OpCode::Multiply(OpInfo::new(self))),
            99 => Ok(OpCode::Halt),
            _ => Err(format!("Unknown op code: {}", val).into())
        }
    }

    pub fn get(&self, index: usize) -> Option<usize> {
        if index > self.data.len() {
            return None;
        }

        Some(self.data[index])
    }

    pub fn deref_(&self, index: usize) -> Option<usize> {
        match self.get(index) {
            Some(v) => self.get(v),
            None => None
        }
    }

    pub fn set(&mut self, index: usize, val: usize) -> Result<()>{
        if index > self.data.len() {
            return Err("Index out of bounds".into());
        }

        self.data[index] = val;
        Ok(())
    }
}

enum OpCode {
    Add(OpInfo),
    Multiply(OpInfo),
    Halt
}

struct OpInfo {
    pub arg1: usize,
    pub arg2: usize,
    pub result: usize
}

impl OpInfo {
    pub fn new(op_list: &mut OpList) -> Self {
        let op_info = OpInfo {
            arg1: op_list.deref_(op_list.op_pointer + 1).unwrap(),
            arg2: op_list.deref_(op_list.op_pointer + 2).unwrap(),
            result: op_list.get(op_list.op_pointer + 3).unwrap()
        };

        op_list.op_pointer += 4;

        op_info
    }
}

