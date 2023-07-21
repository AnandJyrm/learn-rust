use enum_iterator::{all, Sequence};

fn main() {
    #[derive(Debug, Sequence)]
    enum VerboseEnum {
        Add,
        Subtract,
    }
    let var = all::<VerboseEnum>().collect::<Vec<_>>();

    println!("{:?}", stringify!(var[0]));
}
