use crypto::digest::Digest;
use crypto::md5::Md5;
use std::thread;

fn find_valid_checksum(base: &str, search_key: &str, starting_index: u64) -> u64{
    let mut sh = Md5::new(); //make a new md5 hash
    (starting_index..)
    .find(|&x| {
        sh.input_str(&format!("{}{}",base, x)); // make our hash
        let out_hash = sh.result_str();
        sh.reset(); // reset the hash
        out_hash.starts_with(search_key)
    }).unwrap()
}

fn main() {
    let input = "iwrupvqb";
    let part_one_soln = find_valid_checksum(input, "00000",0);
    println!("Part One: {}", part_one_soln);
    let part_two_soln = find_valid_checksum(input, "000000", part_one_soln);
    println!("Part One: {}", part_two_soln);

}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_valid_checksum(){
        assert_eq!(find_valid_checksum("abcdef","00000",609043),609043);
    }

    // #[test]
    // fn test_part_one(){
    //     assert_eq!(part_one("abcdef"),609043);
    // }
}