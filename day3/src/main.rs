use std::collections::HashSet;
use std::fs;
use std::ops::Add;
use std::process;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn visit_houses<'a, I>(instructions: I) -> HashSet<Coord>
where
    I: Iterator<Item = char>,
{
    // returns a set of houses that were visited based on instructions from the char iterator
    let mut visited_points: HashSet<Coord> = instructions
        .scan(Coord::new(0, 0), |current_coordinate, instruction| {
            match instruction {
                '^' => current_coordinate.y += 1,
                '<' => current_coordinate.x -= 1,
                '>' => current_coordinate.x += 1,
                'v' => current_coordinate.y -= 1,
                _ => (),
            };
            Some(*current_coordinate)
        })
        .collect();

    visited_points.insert(Coord::new(0, 0)); // put in the origin coordinate
    visited_points
}

fn part_one(elf_directions: &str) -> usize {
    // make a hashmap of the coordinates santa's visited
    let visited_points = visit_houses(elf_directions.chars());
    visited_points.len()
}

fn part_two(elf_directions: &str) -> usize {
    // make a hashmap of the coordinates santa's visited
    let santa_points = visit_houses(elf_directions.chars().step_by(2));
    let robo_points = visit_houses(elf_directions.chars().skip(1).step_by(2));
    santa_points.union(&robo_points).count()
}

fn main() {
    let input = fs::read_to_string("day3/input.txt").unwrap_or_else(|err| {
        eprintln!("File Load Error {}", err);
        process::exit(1);
    });

    println!("Part One Solution: {}", part_one(&input));
    println!("Part Two Solution: {}", part_two(&input));
    // println!("Part One Solution: {}", part_two(&presents));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let instructions = String::from("^v^v^v^v^v");
        assert_eq!(part_one(&instructions), 2);
        assert_eq!(part_one("^>v"), 4);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("^v^v^v^v^v"), 11);
    }
}
