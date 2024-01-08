use std::error::Error;
use std::fs::read_to_string;

fn area_smallest_side(dimensions: Vec<u32>) -> u32 {
    if dimensions[0] >= dimensions[1] && dimensions[0] >= dimensions[2] {
        return dimensions[1] * dimensions[2];
    } else if dimensions[1] >= dimensions[0] && dimensions[1] >= dimensions[2] {
        return dimensions[0] * dimensions[2];
    } else {
        return dimensions[0] * dimensions[1];
    }
}

fn smallest_perimeter(dimensions: Vec<u32>) -> u32 {
    if dimensions[0] >= dimensions[1] && dimensions[0] >= dimensions[2] {
        return 2 * dimensions[1] + 2 * dimensions[2];
    } else if dimensions[1] >= dimensions[0] && dimensions[1] >= dimensions[2] {
        return 2 * dimensions[0] + 2 * dimensions[2];
    } else {
        return 2 * dimensions[0] + 2 * dimensions[1];
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let present_list = read_to_string("src/input")?;
    let mut area: u32 = 0;
    let mut ribbon: u32 = 0;
    for present in present_list.split("\n") {
        if present.len() == 0 {
            break;
        }
        let dimensions: Vec<u32> = present
            .split("x")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        area = area
            + 2 * dimensions[0] * dimensions[1]
            + 2 * dimensions[1] * dimensions[2]
            + 2 * dimensions[0] * dimensions[2]
            + area_smallest_side(dimensions.clone());
        ribbon =
            ribbon + dimensions[0] * dimensions[1] * dimensions[2] + smallest_perimeter(dimensions);
    }
    println!("paper: {}, ribbon: {}", area, ribbon);
    Ok(())
}
