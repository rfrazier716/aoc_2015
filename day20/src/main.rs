/// This is a house
#[derive(Copy, Clone)]
struct House {
    presents: u32,
}

impl House {
    pub fn new() -> Self {
        Self { presents: 0 }
    }
}

struct HousingGrid {
    size: usize,
    houses: Vec<House>,
}

impl HousingGrid {
    pub fn infinite_elves(size: usize) -> Self {
        let mut grid = Self {
            size,
            houses: vec![House::new(); size + 1],
        };
        grid.count_presents_infinite(); // count the presents
        grid
    }

    pub fn lazy_elves(size: usize) -> Self {
        let mut grid = Self {
            size,
            houses: vec![House::new(); size + 1],
        };
        grid.count_presents_lazy(); // count the presents
        grid
    }

    pub fn presents_at(&self, house: usize) -> u32 {
        self.houses[house].presents
    }

    fn count_presents_infinite(&mut self) {
        for elf in 1..self.size {
            for house in (elf..self.size).step_by(elf) {
                self.houses[house].presents += 10 * elf as u32;
            }
        }
    }

    fn count_presents_lazy(&mut self) {
        for elf in 1..self.size {
            for (_, house) in (0..50).zip((elf..self.size).step_by(elf)) {
                self.houses[house].presents += 11 * elf as u32;
            }
        }
    }

    pub fn first_house_with_atleast(&self, n_presents: u32) -> Option<usize> {
        for n in 1..self.size {
            if self.houses[n].presents > n_presents {
                return Some(n);
            }
        }
        None
    }
}

fn part_one(desired_presents: u32) {
    let grid = HousingGrid::infinite_elves((desired_presents / 10) as usize);
    if let Some(house) = grid.first_house_with_atleast(desired_presents) {
        println!("Part 1 Solution: {}", house);
    } else {
        println!("No House has the desired number of Presents");
    }
}

fn part_two(desired_presents: u32) {
    let grid = HousingGrid::lazy_elves((desired_presents / 10) as usize);
    if let Some(house) = grid.first_house_with_atleast(desired_presents) {
        println!("Part 2 Solution: {}", house);
    } else {
        println!("No House has the desired number of Presents");
    }
}

fn main() {
    let puzzle_input = 36000000;
    part_one(puzzle_input);
    part_two(puzzle_input);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_cases() {
        let grid = HousingGrid::infinite_elves(10);
        let expected = [10, 30, 40, 70, 60, 120, 80, 150, 130];
        for (house, expected_presents) in (1..).zip(expected) {
            assert_eq!(grid.presents_at(house), expected_presents)
        }
    }

    #[test]
    fn test_finding_first_house() {
        let grid = HousingGrid::infinite_elves(10);
        assert_eq!(grid.first_house_with_atleast(100), Some(6))
    }
}
