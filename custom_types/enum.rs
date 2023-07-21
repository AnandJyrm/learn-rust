use enum_iterator::{all};
extern crate enum_iterator;

#[allow(dead_code)]
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

#[derive(Debug)]
enum VerboseEnum {
    Add,
    Subtract,
}

impl VerboseEnum {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

type Operations = VerboseEnum;

fn main() {
    let pressed = WebEvent::KeyPress('x');
    inspect(pressed);

    println!("{:?}", all::<VerboseEnum>());

    let op_y = Operations::Subtract;
    println!("Subtract 3, 2: {}", op_y.run(3, 2));
}
