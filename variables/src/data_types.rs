pub fn data_type_example() {
    // println!("data type example")
    let guess: u32 = "42".parse().expect("Not a number!");
    // let c: u8 = 256;
    let tup = (500, 6.4);
    let (x1, y1) = tup;

    println!("tup: {x1} {y1}");

    let mut a = [1, 2, 5];

    let a_first = a[0];
    println!("a a[0]: {a_first}");
}
