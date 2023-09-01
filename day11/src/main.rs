use std::{cmp::max, ops::Add, str::FromStr};

const ASCII_LETTER_OFFSET: u32 = 97;
const PASSWORD_LENGTH: usize = 8;

#[derive(Copy, Clone, PartialEq, Debug)]
struct PasswordCharacter(u32);

impl Add for PasswordCharacter {
    type Output = (Self, Self);

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.0 + rhs.0;
        if sum < 26 {
            (Self(sum), Self(0))
        } else {
            (Self(sum % 26), Self(1))
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Password(Vec<PasswordCharacter>);

impl Password {
    pub fn new() -> Self {
        return Password(vec![PasswordCharacter(0); PASSWORD_LENGTH]);
    }

    pub fn is_valid(&self) -> bool {
        !self.contains_invalid_characters()
            && self.contains_two_doubles()
            && self.longest_straight() >= 3
    }

    fn contains_invalid_characters(&self) -> bool {
        self.0.iter().any(|x| match x {
            PasswordCharacter(9) | PasswordCharacter(12) | PasswordCharacter(15) => true,
            _ => false,
        })
    }

    fn contains_two_doubles(&self) -> bool {
        let mut idx = 1;
        let mut pairs_found = 0;

        while idx < self.0.len() {
            if self.0[idx - 1] == self.0[idx] {
                pairs_found += 1;
                idx += 1 // extra gap so that 3 characters don't trigger it
            }
            idx += 1
        }

        pairs_found >= 2
    }

    fn longest_straight(&self) -> usize {
        // since we store these in memory in reverse, we need to see if the next element is one less than the previous
        (0..self.0.len())
            .rev()
            .fold((1, self.0.len() - 1), |(longest_straight, tail), head| {
                if self.0[tail].0 + (tail - head) as u32 == self.0[head].0 {
                    (max(longest_straight, tail - head + 1), tail)
                } else {
                    (longest_straight, head)
                }
            })
            .0
    }
}

impl FromStr for Password {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != PASSWORD_LENGTH {
            return Err("Passwords must be eight letters".into());
        }
        let characters = s
            .to_lowercase()
            .chars()
            .rev()
            .map(char::into)
            .map(|x: u32| {
                if x >= ASCII_LETTER_OFFSET && x < (ASCII_LETTER_OFFSET + 26) {
                    Ok(PasswordCharacter(x - ASCII_LETTER_OFFSET))
                } else {
                    Err("invalid Item".into())
                }
            })
            .collect::<Result<Vec<_>, Self::Err>>()?;
        Ok(Password(characters))
    }
}

impl std::fmt::Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .iter()
            .rev()
            .map(|x| char::from_u32(x.0 + ASCII_LETTER_OFFSET).unwrap())
            .map(|char| write!(f, "{}", char))
            .collect::<Result<Vec<_>, std::fmt::Error>>()?;
        Ok(())
    }
}

pub fn next(current: &Password) -> Password {
    if current.0.len() == 0 {
        return Password(vec![PasswordCharacter(0)]);
    }

    let mut carry = PasswordCharacter(1);
    let next: Vec<PasswordCharacter> = current
        .0
        .iter()
        .map(|char| {
            let out = *char + carry;
            carry = out.1;
            out.0
        })
        .collect();
    Password(next)
}

fn next_valid(starting_password: &Password) -> Password {
    let mut pass = next(starting_password);
    while !pass.is_valid(){
        pass = next(&pass)
    }
    pass
}

fn main() {
    let input = Password::from_str("vzbxkghb").unwrap();
    let part_one = next_valid(&input);
    let part_two = next_valid(&part_one);
    println!("part1: {}\tpart2: {}",part_one,part_two);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_password_from_string() {
        let password = Password::from_str("deadbeef".into()).unwrap();
        assert_eq!(format!("{}", password), "deadbeef");
    }

    #[test]
    fn test_password_incrementing() {
        let password = Password::new();
        assert_eq!(format!("{}", password), "aaaaaaaa");

        // use let since we want to redeclare the variable, not mutate our existing one
        let password = next(&password);
        assert_eq!(format!("{}", password), "aaaaaaab");

        let password = next(&Password::from_str("aaaaaaaz").unwrap());
        assert_eq!(format!("{}", password), "aaaaaaba");

        let password = next(&Password::from_str("zzzzzzzz").unwrap());
        assert_eq!(format!("{}", password), "aaaaaaaa");
    }

    #[test]
    fn test_longest_straight() {
        struct TestCase {
            password: String,
            expected_straight: usize,
        }

        let cases = vec![
            TestCase {
                password: "abcdefgh".into(),
                expected_straight: 8,
            },
            TestCase {
                password: "hgfedcba".into(),
                expected_straight: 1,
            },
            TestCase {
                password: "ghcdefab".into(),
                expected_straight: 4,
            },
        ];

        for case in cases.iter() {
            let password = Password::from_str(&case.password).unwrap();
            assert_eq!(password.longest_straight(), case.expected_straight);
        }
    }

    #[test]
    fn test_password_validity() {
        struct TestCase {
            password: String,
            expected_valid: bool,
        }

        let cases = vec![
            TestCase {
                password: "hijklmmn".into(),
                expected_valid: false,
            },
            TestCase {
                password: "abbceffg".into(),
                expected_valid: false,
            },
            TestCase {
                password: "abbcegjk".into(),
                expected_valid: false,
            },
            TestCase {
                password: "abcdffaa".into(),
                expected_valid: true,
            },
        ];

        for case in cases.iter() {
            let password = Password::from_str(&case.password).unwrap();
            assert_eq!(password.is_valid(), case.expected_valid);
        }
    }
}
