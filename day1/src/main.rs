use std::fs;

fn part_one(input: &str) -> i32 {
    // iterate over the input string, adding 1 for ( and -1 for )
    input.chars().fold(0, |acc, char| match char {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    })
}

fn part_two(input: &str) -> usize {
    // iterate over the input string, adding 1 for ( and -1 for )
    let mut it = input.chars().enumerate();
    it.try_fold(0, |acc: u32, (_, char)| match char {
        '(' => acc.checked_add(1),
        ')' => acc.checked_sub(1),
        _ => Some(acc),
    });

    // get the next iterator index, raise error if one never existed
    match it.next() {
        Some((n, _)) => n,
        None => input.len(),
    }
}

fn main() {
    let input = fs::read_to_string("day1/input.txt").expect("Error While Reading File");
    println!("Part 1: {}", part_one(&input));
    println!("Part 1: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*; //import all parent scopes

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("(())"), 0);
        assert_eq!(part_one("))((((("), 3);
        assert_eq!(part_one(")())())"), -3);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("()())"), 5);
        assert_eq!(part_two(")"), 1);
    }
}
