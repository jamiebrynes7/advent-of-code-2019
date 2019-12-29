use std::error::Error;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, BinaryHeap};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::cmp::Ordering;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
type GraphObject = Rc<RefCell<Object>>;

const PATH: &str = "src/day6/input.txt";

fn main() -> Result<()> {
    let file = File::open(PATH)?;
    let mut graph = OrbitGraph::from_file(&file);

    println!("{}", graph.count_orbits());
    println!("{}", graph.dist("YOU", "SAN") - 2);
    Ok(())
}

struct Object {
    pub name: String,
    orbits: Vec<GraphObject>,
    is_orbited_by: Vec<GraphObject>,
    orbit_count: Option<u32>
}

impl Object {
    pub fn new(name: String) -> Self {
        Object {
            name,
            orbits: Vec::new(),
            is_orbited_by: Vec::new(),
            orbit_count: None
        }
    }

    pub fn orbits(&self) -> impl Iterator<Item=&GraphObject> {
        self.orbits.iter().chain(self.is_orbited_by.iter())
    }

    pub fn add_orbit(&mut self, other: &GraphObject) {
        self.orbits.push(other.clone())
    }

    pub fn add_parent(&mut self, other: &GraphObject) {
        self.is_orbited_by.push(other.clone())
    }

    pub fn orbit_count(&mut self) -> u32 {
        if let Some(val) = &self.orbit_count {
            return *val;
        }

        let count = self.orbits.iter().map(|obj| obj.borrow_mut().orbit_count() + 1 ).sum();
        self.orbit_count = Some(count);
        count
    }
}

struct OrbitGraph {
    objects: HashMap<String, GraphObject>,
}

impl OrbitGraph {
    pub fn from_file(file: &File) -> Self {
        let mut graph = OrbitGraph {
            objects: HashMap::new()
        };

        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.unwrap();
            let mut parts = line.split(")");
            let inner = parts.next().unwrap();
            let outer = parts.next().unwrap();

            let inner = graph.get_or_create(inner);
            let outer = graph.get_or_create(outer);

            outer.borrow_mut().add_orbit(&inner);
            inner.borrow_mut().add_parent(&outer);
        }

        graph
    }

    pub fn get_or_create<T: Into<String>>(&mut self, name: T) -> GraphObject {
        let name = name.into();
        self.objects.entry(name.clone()).or_insert_with(|| {
            let object = Object::new(name);
            Rc::new(RefCell::new(object))
        }).clone()
    }

    pub fn count_orbits(&mut self) -> u32 {
        self.objects
            .iter()
            .map(|(_, obj)| obj.borrow_mut().orbit_count())
            .sum()
    }

    pub fn dist<M : Into<String>, N: Into<String>>(&self, start: M, target: N) -> u32 {

        #[derive(PartialEq, Eq)]
        struct State {
            pub cost: u32,
            pub name: String
        }

        impl PartialOrd for State {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.cost.partial_cmp(&other.cost)
            }
        }

        impl Ord for State {
            fn cmp(&self, other: &Self) -> Ordering {
                self.cost.cmp(&other.cost)
            }
        }

        let start = start.into();
        let target = target.into();

        let mut dists: HashMap<String, u32> = HashMap::new();
        let mut heap = BinaryHeap::new();

        dists.entry(start.clone()).or_insert(0);
        heap.push(State {cost: 0, name: start});

        while let Some(State {cost, name}) = heap.pop() {
            // We've already found a better way.
            if cost > dists[&name] { continue; }

            for related in self.objects[&name].borrow_mut().orbits() {
                let next = State { cost: cost + 1, name: related.borrow().name.clone() };
                let distance = dists.entry(next.name.clone()).or_insert(std::u32::MAX);

                if next.cost < *distance {
                    *distance = next.cost;
                    heap.push(next);
                }
            }
        }

        dists[&target]
    }
}