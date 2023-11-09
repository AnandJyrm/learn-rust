struct Point {
    x: u32,
    y: u32,
}

pub struct Square {
    side: u32,
    center: Point,
}

impl Area for Square {
    type Value = u32;
    fn area(&self) -> Self::Value {
        self.side * self.side
    }
}

pub struct Rectangle {
    length: u32,
    breadth: u32,
    center: Point,
}

impl Area for Rectangle {
    type Value = u32;
    fn area(&self) -> Self::Value {
        self.length * self.breadth
    }
}

pub struct Circle {
    radius: u32,
    center: Point,
}

impl Area for Circle {
    type Value = f32;
    fn area(&self) -> Self::Value {
        3.14 * self.radius as f32 * self.radius as f32
    }
}

pub fn print_area<T: Area>(shape: T)
where
    <T as Area>::Value: std::fmt::Display,
{
    println!("Area of shape is {:}", shape.area());
}

pub trait Area {
    type Value: std::fmt::Display;
    fn area(&self) -> Self::Value;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        let square = Square {
            side: 2,
            center: Point { x: 0, y: 0 },
        };
        assert_eq!(square.area(), 4);
    }
}
