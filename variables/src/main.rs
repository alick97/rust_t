const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn var_example_1() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("Three hours in sconds: {THREE_HOURS_IN_SECONDS}");
}

fn shadowing_example() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}

fn let_example() {
    let spaces = "  ";
    let spaces = spaces.len();

    println!("spaces is {spaces}");
}

fn main() {
    // var_example_1();
    // shadowing_example();
    let_example();
}
