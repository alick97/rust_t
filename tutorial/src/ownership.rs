pub fn example_1() {
    let mut s = String::from("hello");
    // let mut s = "hello";
    s.push_str(", world!");
    println!("{s}");
}

pub fn example_variables_and_data_interacting_with_move() {
    let mut x = 5;
    let y = x;

    println!("{x} {y}");
    x = 6;
    println!("{x} {y}");
}

fn get_var_owner(s: String) {
    println!("get owner of str ({s})");
}

pub fn example_owner_err() {
    let s1 = String::from("hello");
    let s2 = s1;

    // err value borrowed here after move
    // println!("{s1}, world!");
    get_var_owner(s2);
    // get_var_owner(s2); // has err use of moved value `s2
}

pub fn example_data_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
}

pub fn example_stack_only_data_copy() {
    let x = 5;
    let y = x;
    // x = 44;
    println!("x = {x}, y = {y}");
}
