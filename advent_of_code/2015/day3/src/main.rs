use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

fn update_coord(entry: char, x: i32, y: i32) -> (i32, i32) {
    if entry == '>' {
        (x + 1, y)
    } else if entry == '<' {
        (x - 1, y)
    } else if entry == '^' {
        (x, y + 1)
    } else {
        (x, y - 1)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let instructions = read_to_string("src/input")?;
    let mut santa_count = HashMap::from([([0, 0], 1)]);
    let mut robo_count = HashMap::from([([0, 0], 1)]);
    let mut santa_coord: Vec<i32> = vec![0, 0];
    let mut robo_coord: Vec<i32> = vec![0, 0];
    let mut total: u32 = 2;

    for entry in instructions.chars() {
        if entry != '>' && entry != '<' && entry != 'v' && entry != '^' {
            break;
        }
        total = total + 1;
        if total % 2 == 0 {
            (robo_coord[0], robo_coord[1]) = update_coord(entry, robo_coord[0], robo_coord[1]);
            let robo_house_count = robo_count
                .entry([robo_coord[0], robo_coord[1]])
                .or_insert(0);
            *robo_house_count += 1;
        } else {
            (santa_coord[0], santa_coord[1]) = update_coord(entry, santa_coord[0], santa_coord[1]);
            let santa_house_count = santa_count
                .entry([santa_coord[0], santa_coord[1]])
                .or_insert(0);
            *santa_house_count += 1;
        }
        //println!("santa_count: {:?}", santa_count);
        //println!("robo_count: {:?}", robo_count);
    }
    for k in robo_count.keys() {
        santa_count.insert(*k, robo_count[k]);
    }
    println!("count = {}", santa_count.len());
    Ok(())
}
