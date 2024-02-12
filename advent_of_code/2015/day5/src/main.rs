use std::error::Error;
use std::fs::read_to_string;

#[allow(dead_code)]
fn old_valid_string(input: String) -> bool {
    let mut vowel_count = 0;
    let mut twice = false;
    let mut disallowed = false;
    for (enumerator, character) in input.char_indices() {
        if character == 'a'
            || character == 'e'
            || character == 'i'
            || character == 'o'
            || character == 'u'
        {
            vowel_count += 1;
        }
        if enumerator > 0 {
            if character == input.chars().nth(enumerator - 1).unwrap() {
                twice = true;
            }
            if (character, input.chars().nth(enumerator - 1).unwrap()) == ('b', 'a')
                || (character, input.chars().nth(enumerator - 1).unwrap()) == ('d', 'c')
                || (character, input.chars().nth(enumerator - 1).unwrap()) == ('q', 'p')
                || (character, input.chars().nth(enumerator - 1).unwrap()) == ('y', 'x')
            {
                disallowed = true;
                break;
            }
        }
    }
    return vowel_count >= 3 && twice && !disallowed;
}

fn valid_string(input: String) -> bool {
    let input_2 = input.clone();
    let mut repeat_once = false;
    let mut pair_twice = false;
    for (enumerator, character) in input.char_indices() {
        if enumerator > 1 {
            if character == input.chars().nth(enumerator - 2).unwrap() {
                repeat_once = true;
            }
            if pair_twice == false {
                let current_pair = (
                    input.chars().nth(enumerator - 2).unwrap(),
                    input.chars().nth(enumerator - 1).unwrap(),
                );
                for (second_enum, second_char) in input_2.char_indices() {
                    if second_enum <= enumerator {
                        continue;
                    } else {
                        let new_pair = (input_2.chars().nth(second_enum - 1).unwrap(), second_char);
                        if current_pair == new_pair {
                            pair_twice = true;
                            break;
                        }
                    }
                }
            }
        }
        if repeat_once && pair_twice {
            break;
        }
    }
    return repeat_once && pair_twice;
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("src/input")?;
    let mut count: u32 = 0;
    for entry in input.split("\n") {
        if valid_string(entry.to_string()) {
            count += 1;
        }
    }
    println!("Valid string count: {}", count);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_old() {
        assert_eq!(old_valid_string("ugknbfddgicrmopn".to_string()), true);
        assert_eq!(old_valid_string("aaa".to_string()), true);
        assert_eq!(old_valid_string("jchzalrnumimnmhp".to_string()), false);
        assert_eq!(old_valid_string("haegwjzuvuyypxyu".to_string()), false);
        assert_eq!(old_valid_string("dvszwmarrgswjxmb".to_string()), false);
    }

    #[test]
    fn test_input() {
        assert_eq!(valid_string("qjhvhtzxzqqjkmpb".to_string()), true);
        assert_eq!(valid_string("xyxy".to_string()), true);
        assert_eq!(valid_string("uurcxstgmygtbstg".to_string()), false);
        assert_eq!(valid_string("ieodomkazucvgmuy".to_string()), false);
    }
}
