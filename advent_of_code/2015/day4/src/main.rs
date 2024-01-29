use md5::{compute, Digest};

const INPUT: &str = "iwrupvqb";

fn is_valid_hash(hash: Digest) -> bool {
    hash[0] == 0 && hash[1] == 0 && hash[2] == 0
}
fn main() {
    let mut i: u32 = 1;
    loop {
        let input = format!("{}{}", INPUT, i);
        let hash: Digest = compute(&input);
        if is_valid_hash(hash) {
            println!("input: {}, hash: {:?}", input, hash);
            break;
        } else {
            i += 1;
        }
    }
}
