use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const PATH: &str = "src/day8/input.txt";
const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn main() -> Result<()> {
    let data = std::fs::read_to_string(PATH)?;
    part1(&data);
    part2(&data);

    Ok(())
}

fn part1(data: &str) {
    let layers = Layer::from_data_stream(data.trim(), WIDTH, HEIGHT);
    let layer = layers.iter().min_by_key(|l| l.count_of('0')).unwrap();

    let result = layer.count_of('1') * layer.count_of('2');

    println!("Result: {}", result);
}

fn part2(data: &str) {
    let layers = Layer::from_data_stream(data.trim(), WIDTH, HEIGHT);
    let result = Layer::compose(&layers, WIDTH, HEIGHT);
    println!("{}", result);
}

struct Layer {
    width: usize,
    height: usize,
    data: Vec<char>,
}

impl Display for Layer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        const PIXELS: [&str; 3] = ["  ", "██", "  "];

        for i in 0..self.height * self.width {
            if i % self.width == 0 {
                f.write_str("\n")?;
            }

            match self.data[i] {
                '0' => f.write_str(PIXELS[0])?,
                '1' => f.write_str(PIXELS[1])?,
                '2' => f.write_str(PIXELS[2])?,
                _ => panic!("Oh no."),
            };
        }

        Ok(())
    }
}

impl Layer {
    pub fn from_data_stream(data: &str, width: usize, height: usize) -> Vec<Layer> {
        let mut data_stream = data
            .chars()
            .into_iter()
            .peekable();

        let mut layers = Vec::new();
        while data_stream.peek().is_some() {
            let data = data_stream
                .by_ref()
                .take(width * height)
                .collect();

            layers.push(Layer {
                width,
                height,
                data,
            })
        }

        layers
    }

    pub fn compose<T: AsRef<[Layer]>>(layers: &T, width: usize, height: usize) -> Layer {
        let mut data = Vec::with_capacity(width * height);

        fn pixel_at(layers: &[Layer], i: usize) -> char {
            for layer in layers.as_ref() {
                if layer.data[i] != '2' {
                    return layer.data[i];
                }
            }

            '2'
        }

        for i in 0..(width * height) {
            data.push(pixel_at(layers.as_ref(), i));
        }

        Layer {
            width,
            height,
            data,
        }
    }

    pub fn count_of(&self, val: char) -> usize {
        self.data.iter().filter(|v| **v == val).count()
    }
}
