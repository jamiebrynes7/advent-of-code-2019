use std::error::Error;
use std::str::FromStr;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const PATH: &str = "src/day8/input.txt";

fn main() -> Result<()> {
    let data = std::fs::read_to_string(PATH)?;
    part1(&data)?;

    Ok(())
}

fn part1(data: &str) -> Result<()> {
    let layers = Layer::from_data_stream(data.trim(), 25, 6);
    let layer = layers.iter().min_by_key(|l| l.count_of(0)).unwrap();

    let result = layer.count_of(1) * layer.count_of(2);

    println!("Result: {}", result);
    Ok(())
}

struct Layer {
    width: u32,
    height: u32,
    data: Vec<u32>,
}

impl Layer {
    pub fn from_data_stream(data: &str, width: u32, height: u32) -> Vec<Layer> {
        let mut data_stream = data.chars().into_iter().map(|c| u32::from_str(&c.to_string()).unwrap()).peekable();

        let mut layers = Vec::new();
        while data_stream.peek().is_some() {
            let data = data_stream
                .by_ref()
                .take((width * height) as usize)
                .collect();

            layers.push(Layer {
                width,
                height,
                data,
            })
        }

        layers
    }

    pub fn count_of(&self, val: u32) -> usize {
        self.data.iter().filter(|v| **v == val).count()
    }
}
