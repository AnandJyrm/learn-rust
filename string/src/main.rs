#[derive(Debug, Default)]
struct sensor {
    sensor_name: String,
    sensor_value: i32,
}


#[derive(Debug, Default)]
struct cardinfo {
    pid: String,
    location: String,
    num: [u32; 20],
    temp_sensors: [sensor; 20],
}

fn main() {
    println!("Hello, world!");
    let new_card = cardinfo { ..Default::default() };
    println!("card {:?}", new_card);
}
