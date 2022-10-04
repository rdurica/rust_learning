#![allow(unused)]
fn main() {
    tuple_type()
}

fn mutable_immutable() {
    // Mutable = can be changed
    // Immutable = no change allowed
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; //constant is immutable
}

fn shadowing() {
    //Same variable name for two different variables
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    } // Inner scope

    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len(); // Often used when need to change datatype
    println!("No of spaces: {spaces}")
}

fn floating_point() {
    let x = 2.0; // f64 - default
    let y: f32 = 3.0; // f32
}

fn numeric_operations() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;
}

fn bool_type() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}

fn char_type() {
    let c = 'z'; // For char is used single quotes
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}

fn tuple_type() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is {y}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
}

fn array_type() {
    // Must have fixed length and same type
    let a = [1, 2, 3, 4, 5];

    let b: [i32; 5] = [1, 2, 3, 4, 5];

    let c = [3; 5]; // Same as let c = [3, 3, 3, 3, 3];

    let first = a[0]; // get 1

    let second = a[1]; // get 2
}
