use md5;

fn is_valid_checksum(base: &str, number: i32, n_zeros: usize) -> bool{
    let checksum =  format!("{:x}",md5::compute(format!("{}{}",base,number)));
    checksum
    .chars()
    .take(n_zeros)
    .fold(true, |same, x| same && (x=='0'))
}

fn part_one(puzzle_input: &str) -> i32{
    (0..)
    .find(|&x| is_valid_checksum(puzzle_input, x, 5)).unwrap()
}

fn part_two(puzzle_input: &str) -> i32{
    (0..)
    .find(|&x| is_valid_checksum(puzzle_input, x, 6)).unwrap()
}

fn main() {
    let input = "iwrupvqb";
    println!("Part One: {}", part_one(input));
    println!("Part One: {}", part_two(input));

}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_valid_checksum(){
        assert!(is_valid_checksum("abcdef",609043, 5));
        assert!(is_valid_checksum("pqrstuv",1048970, 5));
        assert!(is_valid_checksum("iwrupvqb",346386, 5));

    }

    // #[test]
    // fn test_part_one(){
    //     assert_eq!(part_one("abcdef"),609043);
    // }
}