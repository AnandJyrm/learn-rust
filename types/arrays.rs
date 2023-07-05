use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("Number of elements in slice: {}", slice.len());
    println!("Slice occupies {} bytes", mem::size_of_val(slice));
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    println!("First element of the array: {}", xs[0]);
    println!("Number of elements in array: {}", xs.len());
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    println!("Borrow the whole array as a slice.\n\n");
    analyze_slice(&xs);


    println!("Borrow partial array as a slice.\n\n");
    analyze_slice(&xs[2..4]);

    println!("Borrow partial array as a slice.\n\n");
    analyze_slice(&ys[..]);

    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]);

    for i in 0..xs.len() + 1 { // Oops, one element too far!
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
        println!("Expect: {:?}", xs.get(i).expect("what"));
    }
}
