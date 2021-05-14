use std::{fs, ops::{Index, IndexMut}, process};
use lazy_static::lazy_static;
use regex::Regex;

type GridIndex = (usize, usize);
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Light{
    On,
    Off
}

struct LightGrid {
    lights: Vec<Light>,
    rows: usize,
    columns: usize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum GridCommand{
    On,
    Off,
    Toggle
}
struct GridInstruction{
    command: GridCommand,
    start_pos: GridIndex,
    end_pos: GridIndex
}

impl LightGrid {
    pub fn new(rows: usize, columns: usize) -> Self {
        Self {
            lights: vec![Light::Off; rows * columns],
            rows,
            columns,
        }
    }

    pub fn switch(&mut self, (r0,c0): GridIndex, (r1,c1): GridIndex, state: Light){
        for row in r0..r1+1{
            for col in c0..c1+1{
                self[(row, col)] = state;
            }
        }
    }

    pub fn toggle(&mut self, (r0,c0): GridIndex, (r1,c1): GridIndex){
        for row in r0..r1+1{
            for col in c0..c1+1{
                self[(row, col)] = match self[(row, col)]{
                    Light::On => Light::Off,
                    Light::Off => Light::On
                }
            }
        }
    }

    pub fn count(&self) -> usize{
        self.lights
        .iter()
        .filter(|&x| *x == Light::On)
        .count()
    }

    pub fn execute(&mut self, instruction: GridInstruction){
        match instruction.command{
            GridCommand::On => self.switch(instruction.start_pos, instruction.end_pos, Light::On),
            GridCommand::Off => self.switch(instruction.start_pos, instruction.end_pos, Light::Off),
            GridCommand::Toggle => self.toggle(instruction.start_pos, instruction.end_pos),
        }
    }

}


impl IndexMut<GridIndex> for LightGrid{
    fn index_mut(&mut self, (r,c): GridIndex) -> &mut Self::Output{
        &mut self.lights[r*self.columns+c]
    }
}

impl Index<GridIndex> for LightGrid{
    fn index(&self, (r,c): GridIndex) -> &Self::Output {
        if r < self.rows && c < self.columns{
            &self.lights[r*self.columns+c]
        } else {
            panic!("Index Out of Bounds Error");
        }
    }

    type Output=Light;
}

fn parse(input: &str) -> Vec<GridInstruction>{
    lazy_static!{
        static ref COMMAND: Regex = Regex::new(r"(toggle|turn off|turn on) ([0-9]+),([0-9]+) through ([0-9]+),([0-9]+)").unwrap();
    }

    //iterate over the lines and collect grid instructions
    input
    .lines()
    .map(|x| {
        if let Some(captures) = COMMAND.captures(x){
            let command = match &captures[1]{
                "toggle" => GridCommand::Toggle,
                "turn on" => GridCommand::On,
                "turn off" => GridCommand::Off,
                &_ => panic!("Could not Parse Input")
            };
            let start_pos: GridIndex = (captures[2].parse().unwrap(), captures[3].parse().unwrap());
            let end_pos: GridIndex = (captures[4].parse().unwrap(), captures[5].parse().unwrap());
            GridInstruction{command, start_pos, end_pos}
        } else{panic!("Could not Parse Input")}
    }).collect()
}

fn main() {
    let input = fs::read_to_string("day6/input.txt").unwrap_or_else(|err| {
        eprintln!("File Load Error {}", err);
        process::exit(1);
    });
    
    let commands = parse(&input);
    let mut grid = LightGrid::new(1000, 1000);
    for command in commands{
        grid.execute(command);
    }

    println!("Part One Solution: {}", grid.count());
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_grid_creation(){
        let grid = LightGrid::new(1000, 1000);
        assert_eq!(grid.lights.len(), 1000000);
        assert_eq!(grid[(0,0)], Light::Off);
    }
    #[test]
    fn test_count(){
        let mut grid = LightGrid::new(3, 3);
        assert_eq!(grid.count(), 0);
        grid[(0,0)] = Light::On;
        assert_eq!(grid.count(), 1);
    }

    #[test]
    fn test_toggle(){
        let mut grid = LightGrid::new(2,2);
        grid[(0,0)] = Light::On;
        grid.toggle((0,0), (1,1));

        let mut expected_on = vec![Light::On; 4];
        expected_on[0] = Light::Off;

        for (actual, expected) in grid.lights.iter().zip(expected_on.iter()){
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_grid_modification(){
        let mut grid = LightGrid::new(3, 3);
        grid[(0,0)] = Light::On;
        assert_eq!(grid[(0,0)], Light::On);

        grid[(1,0)] = Light::On;
        assert_eq!(grid.lights[3], Light::On);
    }
    #[test]
    fn test_switch(){
        let mut grid = LightGrid::new(3, 3);
        grid.switch((0,0),(1,1), Light::On);
        let mut expected_on = vec![Light::Off; 9];
        expected_on[0] = Light::On;
        expected_on[1] = Light::On;
        expected_on[3] = Light::On;
        expected_on[4] = Light::On;

        for (actual, expected) in grid.lights.iter().zip(expected_on.iter()){
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_parsing(){
        let results = parse("toggle 461,550 through 564,900\nturn off 370,39 through 425,839");
        assert_eq!(results.len(), 2)
    }
}
