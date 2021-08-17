use itertools::Itertools;
use std::fmt::Display;

#[derive(Debug)]
struct NumCounter {
    num: u8,
    count: u8,
}

impl NumCounter {
    pub fn new(num: u8) -> Self {
        Self { num, count: 1 }
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }
}

impl Display for NumCounter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.count, self.num)
    }
}

fn look_and_say(input: &[u8]) -> Vec<u8> {
    input
        .iter()
        // map each char into a char counter struct
        .map(|x| NumCounter::new(*x))
        // Coalesce is really cool! collapses an iterator based on conditions
        .coalesce(|mut current_count, new_count| {
            if current_count.num == new_count.num {
                current_count.increment();
                Ok(current_count)
            } else {
                Err((current_count, new_count))
            }
        })
        .fold(Vec::new(), |mut sequence, x| {
            sequence.push(x.count);
            sequence.push(x.num);
            sequence
        })
}

fn solution(puzzle_input: &[u8]) {
    let mut output = puzzle_input.to_vec();
    for n in 1..=50 {
        output = look_and_say(&output);
        if n == 40 {
            println!("Part One Solution: {}", output.len())
        }
    }
    println!("Part Two Solution: {}", output.len())
}

fn main() {
    // get the input string into something useable
    let puzzle_input: Vec<u8> = "1321131112"
        .chars()
        .map(|x| {
            x.to_digit(10)
                .expect("Could Not Convert Puzzle input to array of u8") as u8
        })
        .collect();

    // Run the puzzle solution
    solution(&puzzle_input);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_look_and_say() {
        assert_eq!(look_and_say(&[1, 2, 1, 1]), [1, 1, 1, 2, 2, 1])
    }
}
