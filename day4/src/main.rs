use crypto::digest::Digest;
use crypto::md5::Md5;
use std::thread;
use std::sync::mpsc;

fn find_valid_checksum(base: &str, search_key: &str, starting_index: u64, end_index: u64) -> Option<u64>{
    let mut sh = Md5::new(); //make a new md5 hash
    (starting_index..end_index)
    .find(|&x| {
        sh.input_str(&format!("{}{}",base, x)); // make our hash
        let out_hash = sh.result_str();
        sh.reset(); // reset the hash
        out_hash.starts_with(search_key)
    })
}

fn main() {
    let input = "iwrupvqb";
    let part_one_soln = find_valid_checksum(input, "00000",0, 1000000).unwrap();
    println!("Part One: {}", part_one_soln);
    let (tx, rx) = mpsc::channel();
    for val in 0..10{
        let tx1 = tx.clone();
        thread::spawn(move || {
            tx1.send(find_valid_checksum(input, "000000", part_one_soln + 1000000*val, part_one_soln+1000000*(val+1)-1)).unwrap();
        });
    }
    drop(tx);
    
    for received in rx{
        match received{
            Some(x) => println!("Part Two: {}", x),
            None => ()
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_valid_checksum(){
        assert_eq!(find_valid_checksum("abcdef","00000",609043, 1000000).unwrap(),609043);
    }

    // #[test]
    // fn test_part_one(){
    //     assert_eq!(part_one("abcdef"),609043);
    // }
}