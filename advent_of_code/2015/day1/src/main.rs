use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let mut floor: i32 = 0;
    let instructions = read_to_string("src/input")?;
    let mut basement: u32 = 0;
    let mut found: bool = false;
    for c in instructions.chars() {
        if c == ')' {
            floor = floor - 1;
        } else if c == '(' {
            floor = floor + 1;
        }
        if found == false {
            if floor == -1 {
                found = true;
            }
            basement = basement + 1;
        }
    }
    println!("floor: {}, basement {}", floor, basement);
    Ok(())
}
