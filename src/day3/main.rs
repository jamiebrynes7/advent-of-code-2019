use std::error::Error;
use std::collections::{HashSet, HashMap};
use std::convert::TryFrom;
use crate::Direction::{Right, Left, Up, Down};
use std::str::FromStr;
use std::fs::File;
use std::io::{BufReader, BufRead};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const PATH: &str = "src/day3/input.txt";

fn main() -> Result<()> {
    let f = File::open(PATH)?;
    let mut buf_read = BufReader::new(f);
    let mut data = String::new();

    buf_read.read_line(&mut data)?;
    let first = Line::from_string(data.trim());

    data.clear();

    buf_read.read_line(&mut data)?;
    let second = Line::from_string(data.trim());

    part1(&first, &second)?;
    part2(&first, &second)?;
    Ok(())
}

fn part1(first: &Line, second: &Line) -> Result<()> {
    let data = first
        .get_intersections(&second)
        .map(|point| point.distance())
        .min();

    println!("{}", data.unwrap());

    Ok(())
}

fn part2(first: &Line, second: &Line) -> Result<()> {
    let data = first
        .get_steps_at_intersection(second)
        .min();

    println!("{}", data.unwrap());

    Ok(())
}

struct Line {
    points: HashSet<Point>,
    steps: HashMap<Point, u32>
}

impl Line {
    pub fn from_string(line: &str) -> Self {
        let instructions: Vec<Instruction> = line.split(",").map(|instruction| {
            let mut chars = instruction.chars();
            let direction = Direction::try_from(chars.next().unwrap()).unwrap();
            let value = u32::from_str(chars.as_str()).unwrap();
            Instruction { direction, value }
        }).collect();

        let mut point = Point { x: 0, y: 0 };
        let mut step_count = 0;

        let mut points = HashSet::new();
        let mut steps = HashMap::new();

        for inst in instructions {
            for _ in 0..inst.value {
                step_count += 1;
                point = point.next(inst.direction);
                points.insert(point);
                steps.insert(point, step_count);
            }
        }

        Line { points, steps }
    }

    pub fn get_intersections<'a>(&'a self, other: &'a Line) -> impl Iterator<Item = &'a Point> {
        self.points.intersection(&other.points)
    }

    pub fn get_steps_at_intersection<'a>(&'a self, other: &'a Line) -> impl Iterator<Item = u32> + 'a {
        self.get_intersections(other)
            .map(move |point| self.steps.get(point).unwrap() + other.steps.get(point).unwrap())
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Right,
    Left,
    Up,
    Down
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> std::result::Result<Self, ()> {
        match value.to_ascii_lowercase() {
            'r' => Ok(Right),
            'l' => Ok(Left),
            'u' => Ok(Up),
            'd' => Ok(Down),
            _ => Err(())
        }
    }
}

struct Instruction {
    pub direction: Direction,
    pub value: u32
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn next(self, dir: Direction) -> Point {
        match dir {
            Right => Point { x: self.x + 1, y: self.y },
            Left => Point { x: self.x - 1, y: self.y },
            Up => Point { x: self.x, y: self.y + 1 },
            Down => Point { x: self.x, y: self.y - 1 }
        }
    }

    pub fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}