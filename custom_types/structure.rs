use fmt::Formatter;
use std::fmt;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

#[derive(Debug)]
struct Unit;

struct Point {
    x: i32,
    y: i32,
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "(x,y) = ({},{})", self.x, self.y)
    }
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}
impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "corners are:\n{}\n{}", self.top_left, self.bottom_right)
    }
}

fn square(p: Point, d: i32) -> Rectangle {
    Rectangle {
        top_left: Point {
            x: (p.x + d),
            y: (p.y + d),
        },
        bottom_right: Point {
            x: (p.y + d),
            y: (p.x + d),
        },
    }
}
fn area(rect: Rectangle) -> i32 {
    let Rectangle {
        top_left: Point { x: a, y: b },
        bottom_right: Point { x: c, y: d },
    } = rect;
    a * b * c * d
}

fn main() {
    let unit: Unit = Unit;
    println!("{:?}", unit);

    let peters_name = String::from("Peter");
    let peters_age = 27;
    let peter = Person {
        name: peters_name,
        age: peters_age,
    };
    println!("{:?}", peter);
    println!("{}, {}", peter.name, peter.age);

    let point_a: Point = Point { x: 1, y: 4 };
    let point_b: Point = Point { x: 2, y: 6 };
    let point_c: Point = Point { x: 5, ..point_a };
    println!("{}", point_a);
    println!("{}", point_b);
    println!("{}", point_c);

    let Point {
        x: left_edge,
        y: top_edge,
    } = point_c;
    println!("{}, {}, {}", point_c, left_edge, top_edge);

    let rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: point_b,
    };

    println!("{}", rectangle);

    let square_a = square(Point { x: 1, y: 2 }, 32);
    println!("{}", square_a);

    println!("Area of square: {}", area(square_a));
}
