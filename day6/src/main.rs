use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fs,
    ops::{Index, IndexMut},
    process,
};

type GridIndex = (usize, usize);
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Light {
    On,
    Off,
}

impl Default for Light {
    fn default() -> Self {
        Self::Off
    }
}

struct Grid<T> {
    lights: Vec<T>,
    rows: usize,
    columns: usize,
}

impl<T> Grid<T>
where
    T: Default + Clone,
{
    fn new(rows: usize, columns: usize) -> Self {
        Self {
            lights: vec![T::default(); rows * columns],
            rows,
            columns,
        }
    }
}

type StaticGrid = Grid<Light>;
type DynamicGrid = Grid<u32>;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GridCommand {
    On,
    Off,
    Toggle,
}
pub struct GridInstruction {
    command: GridCommand,
    start_pos: GridIndex,
    end_pos: GridIndex,
}

impl GridInstruction {
    pub fn new(command: GridCommand, start_pos: GridIndex, end_pos: GridIndex) -> Self {
        Self {
            command,
            start_pos,
            end_pos,
        }
    }
}

pub trait LightGrid {
    fn count(&self) -> u32;
    fn execute(&mut self, instruction: &GridInstruction);
}

impl LightGrid for StaticGrid {
    fn count(&self) -> u32 {
        self.lights.iter().filter(|&x| *x == Light::On).count() as u32
    }

    fn execute(&mut self, instruction: &GridInstruction) {
        let (r0, c0) = instruction.start_pos;
        let (r1, c1) = instruction.end_pos;
        for row in r0..r1 + 1 {
            for col in c0..c1 + 1 {
                self[(row, col)] = match instruction.command {
                    GridCommand::Off => Light::Off,
                    GridCommand::On => Light::On,
                    GridCommand::Toggle => {
                        if self[(row, col)] == Light::On {
                            Light::Off
                        } else {
                            Light::On
                        }
                    }
                }
            }
        }
    }
}

impl LightGrid for DynamicGrid {
    fn count(&self) -> u32 {
        self.lights.iter().sum()
    }

    fn execute(&mut self, instruction: &GridInstruction) {
        let (r0, c0) = instruction.start_pos;
        let (r1, c1) = instruction.end_pos;
        for row in r0..r1 + 1 {
            for col in c0..c1 + 1 {
                match instruction.command {
                    GridCommand::Off => self[(row, col)] = self[(row, col)].saturating_sub(1),
                    GridCommand::On => self[(row, col)] += 1,
                    GridCommand::Toggle => self[(row, col)] += 2,
                }
            }
        }
    }
}

impl<T> IndexMut<GridIndex> for Grid<T> {
    fn index_mut(&mut self, (r, c): GridIndex) -> &mut Self::Output {
        &mut self.lights[r * self.columns + c]
    }
}

impl<T> Index<GridIndex> for Grid<T> {
    fn index(&self, (r, c): GridIndex) -> &Self::Output {
        if r < self.rows && c < self.columns {
            &self.lights[r * self.columns + c]
        } else {
            panic!("Index Out of Bounds Error");
        }
    }

    type Output = T;
}

fn parse(input: &str) -> Vec<GridInstruction> {
    lazy_static! {
        static ref COMMAND: Regex =
            Regex::new(r"(toggle|turn off|turn on) ([0-9]+),([0-9]+) through ([0-9]+),([0-9]+)")
                .unwrap();
    }

    //iterate over the lines and collect grid instructions
    input
        .lines()
        .map(|x| {
            if let Some(captures) = COMMAND.captures(x) {
                let command = match &captures[1] {
                    "toggle" => GridCommand::Toggle,
                    "turn on" => GridCommand::On,
                    "turn off" => GridCommand::Off,
                    &_ => panic!("Could not Parse Input"),
                };
                let start_pos: GridIndex =
                    (captures[2].parse().unwrap(), captures[3].parse().unwrap());
                let end_pos: GridIndex =
                    (captures[4].parse().unwrap(), captures[5].parse().unwrap());
                GridInstruction {
                    command,
                    start_pos,
                    end_pos,
                }
            } else {
                panic!("Could not Parse Input")
            }
        })
        .collect()
}

fn main() {
    let input = fs::read_to_string("day6/input.txt").unwrap_or_else(|err| {
        eprintln!("File Load Error {}", err);
        process::exit(1);
    });

    let commands = parse(&input);
    let mut static_grid = StaticGrid::new(1000, 1000);
    let mut dynamic_grid = DynamicGrid::new(1000, 1000);
    for command in commands {
        static_grid.execute(&command);
        dynamic_grid.execute(&command);
    }
    println!("Part One Solution: {}", static_grid.count());
    println!("Part Two Solution: {}", dynamic_grid.count());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_grid_creation() {
        let grid = StaticGrid::new(1000, 1000);
        assert_eq!(grid.lights.len(), 1000000);
        assert_eq!(grid[(0, 0)], Light::Off);
    }
    #[test]
    fn test_count() {
        let mut grid = StaticGrid::new(3, 3);
        assert_eq!(grid.count(), 0);
        grid[(0, 0)] = Light::On;
        assert_eq!(grid.count(), 1);
    }

    #[test]
    fn test_toggle() {
        let mut grid = StaticGrid::new(2, 2);
        grid[(0, 0)] = Light::On;
        let command = GridInstruction::new(GridCommand::Toggle, (0, 0), (1, 1));
        grid.execute(&command);

        let mut expected_on = vec![Light::On; 4];
        expected_on[0] = Light::Off;

        for (actual, expected) in grid.lights.iter().zip(expected_on.iter()) {
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_grid_modification() {
        let mut grid = StaticGrid::new(3, 3);
        grid[(0, 0)] = Light::On;
        assert_eq!(grid[(0, 0)], Light::On);

        grid[(1, 0)] = Light::On;
        assert_eq!(grid.lights[3], Light::On);
    }
    #[test]
    fn test_switch() {
        let mut grid = StaticGrid::new(3, 3);
        let command = GridInstruction::new(GridCommand::On, (0, 0), (1, 1));

        grid.execute(&command);
        let mut expected_on = vec![Light::Off; 9];
        expected_on[0] = Light::On;
        expected_on[1] = Light::On;
        expected_on[3] = Light::On;
        expected_on[4] = Light::On;

        for (actual, expected) in grid.lights.iter().zip(expected_on.iter()) {
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_parsing() {
        let results = parse("toggle 461,550 through 564,900\nturn off 370,39 through 425,839");
        assert_eq!(results.len(), 2)
    }
}
