use std::fs;
use std::process;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub struct Present {
    x: i32,
    y: i32,
    z: i32,
}

impl Present {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        let mut sides = vec![x, y, z];
        sides.sort();
        Self {
            x: sides[0],
            y: sides[1],
            z: sides[2],
        }
    }

    pub fn surface_area(&self) -> i32 {
        2 * (self.x * (self.y + self.z) + self.y * self.z)
    }

    pub fn wrapping_area(&self) -> i32 {
        self.surface_area() + self.x * self.y
    }

    pub fn volume(&self) -> i32 {
        self.x * self.y * self.z
    }

    pub fn ribbon_length(&self) -> i32 {
        2 * (self.x + self.y) + self.volume()
    }
}

impl FromStr for Present {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elements = s
            .split("x")
            .map(|x| x.parse::<i32>())
            .collect::<Result<Vec<_>, _>>();

        match elements {
            Ok(x) => match x {
                x if x.len() == 3 => Ok(Self::new(x[0], x[1], x[2])),
                x => Err(format!("incorrect elements in {}, found {}", s, x.len())),
            },
            Err(_) => Err(format!("Could not parse Dimensions from {}", s)),
        }
    }
}

fn load_presents(input: &str) -> Result<Vec<Present>, String> {
    input
        .split_terminator("\r\n")
        .map(|s| Present::from_str(s))
        .collect::<Result<Vec<_>, _>>()
}

fn part_one(presents: &[Present]) -> i32 {
    presents.iter().map(|x| x.wrapping_area()).sum::<i32>()
}

fn part_two(presents: &[Present]) -> i32 {
    presents.iter().map(|x| x.ribbon_length()).sum::<i32>()
}

fn main() {
    let input = fs::read_to_string("day2/input.txt").unwrap_or_else(|err| {
        eprintln!("File Load Error {}", err);
        process::exit(1);
    });

    let presents = load_presents(&input).unwrap_or_else(|err| {
        eprintln!("Error Loading Presents: {}", err);
        process::exit(1);
    });

    println!("Part One Solution: {}", part_one(&presents));
    println!("Part One Solution: {}", part_two(&presents));
}

#[cfg(test)]
mod tests {
    use super::*; //import all parent scopes

    #[test]
    fn test_present() {
        let present = Present::new(2, 3, 4);
        assert_eq!(present.wrapping_area(), 58);

        let present = Present::new(1, 1, 10);
        assert_eq!(present.wrapping_area(), 43);
    }

    #[test]
    fn test_present_parsing() {
        let present_string = "29x13x26";
        let present = Present::from_str(present_string).unwrap();
        assert_eq!(present, Present::new(29, 13, 26));
    }

    #[test]
    fn test_ribbon_length() {
        let present = Present::new(2, 3, 4);
        assert_eq!(present.ribbon_length(), 34);

        let present = Present::new(1, 1, 10);
        assert_eq!(present.ribbon_length(), 14);
    }
}
