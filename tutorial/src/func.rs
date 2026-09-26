fn five() -> i32 {
    5
}

pub fn func_example() {
    //  let x = five();
    //  println!("The value of x is: {x}");
    let add_one = |x: i32| x + 1;
    let result = add_one(5);
    println!("result: {}", result);
}

fn plus_one(x: i32) -> i32 {
    return x + 1;
}
