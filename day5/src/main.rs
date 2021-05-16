use fancy_regex::Regex;
use lazy_static::lazy_static;
use std::fs;
use std::process;

fn is_nice(input: &str) -> bool {
    lazy_static! {
        static ref THREE_VOWELS: Regex = Regex::new(r"(?:[aeiou].*?){3,}").unwrap();
        static ref DOUBLE_LETTER: Regex = Regex::new(r"(.)\1").unwrap();
        static ref ILLEGAL_SUBSTRINGS: Regex = Regex::new(r"^(?!.*(ab|cd|pq|xy)).*$").unwrap();
    }

    THREE_VOWELS.is_match(input).unwrap()
        && DOUBLE_LETTER.is_match(input).unwrap()
        && ILLEGAL_SUBSTRINGS.is_match(input).unwrap()
}

fn part_one(input: &str) -> u32 {
    input
        .split("\r\n")
        .fold(0, |acc, item| match is_nice(item) {
            true => acc + 1,
            false => acc,
        })
}
fn main() {
    let input = fs::read_to_string("day5/input.txt").unwrap_or_else(|err| {
        eprintln!("File Load Error {}", err);
        process::exit(1);
    });

    println!("Part One Solution: {}", part_one(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nice_strings() {
        assert!(is_nice("ugknbfddgicrmopn"));
        assert!(is_nice("aaa"));
    }

    #[test]
    fn test_naughty_strings() {
        assert!(!is_nice("jchzalrnumimnmhp"));
        assert!(!is_nice("haegwjzuvuyypxyu"));
        assert!(!is_nice("dvszwmarrgswjxmb"));
    }
}
