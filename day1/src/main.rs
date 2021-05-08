use std::fs;

fn part_one(input: &str) -> Result<i32, String> {
    // iterate over the input string, adding 1 for ( and -1 for )
    input.chars().try_fold(0, |acc, char| match char {
        '(' => Ok(acc + 1),
        ')' => Ok(acc - 1),
        x => Err(format!("Received Invalid Character {}", x)),
    })
}

fn part_two(input: &str) -> Result<usize, String> {
    // iterate over the input string, adding 1 for ( and -1 for )
    let mut it = input.chars().enumerate();
    it.try_fold(0, |acc: u32, (_, char)| match char {
        '(' => acc.checked_add(1),
        ')' => acc.checked_sub(1),
        _ => Some(acc),
    });

    // get the next iterator index, raise error if one never existed
    Ok(match it.next() {
        Some((n, _)) => n,
        None => input.len(),
    })
}

fn main() {
    let input = fs::read_to_string("day1/input.txt").expect("Error While Reading File");
    println!("Part 1: {}", part_one(&input).unwrap());
    println!("Part 1: {}", part_two(&input).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*; //import all parent scopes

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("(())").unwrap(), 0);
        assert_eq!(part_one("))(((((").unwrap(), 3);
        assert_eq!(part_one(")())())").unwrap(), -3);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("()())").unwrap(), 5);
        assert_eq!(part_two(")").unwrap(), 1);
    }
}
