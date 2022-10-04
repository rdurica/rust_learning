#![allow(unused)]

fn main() {
    print_labeled_measurement(5, 'h'); // fn call
}

// Variables, mutability, data types

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

// Functions

fn hello_world() {
    println!("Function called - Success");
}

fn another_function(x: i32) {
    // Function with parameter
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    // Separate the parameter declarations with commas
    println!("The measurement is: {value}{unit_label}");
}

fn fn_with_return_val() -> i32 {
    // You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn expression_example() {
    let y = {
        let x = 3;
        x + 1
    };

    // Expression
    // {
    //     let x = 3;
    //     x + 1 // do not end with semicolon
    // }

    println!("The value of y is: {y}");
}

// Control Flow

fn if_example() {
    let number = 3;

    //Simple
    if number == 3 {
        println!("Success!");
    }

    // ff/Else
    if number < 5 {
        println!("True");
    } else {
        println!("False");
    }

    //if/elseif/else = executed first block which match condition
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let Statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
}

// Loops

fn loop_example() {
    // Base loop
    loop {
        println!("again!");
    }

    let mut counter = 0;

    // Loop return value
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    let mut count = 0;

    // Labeled loop to Disambiguate Between Multiple Loops
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // Break outside "named" loop
            }
            remaining -= 1;
        }

        count += 1;
    }
}

fn while_example() {
    let mut number = 3;

    // While
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn for_example() {
    let source_array = [10, 20, 30, 40, 50];

    // Foreach element in array
    for element in source_array {
        println!("Value of element is: {element}");
    }

    // For number in range 1 - 4
    for number in (1..4).rev() {
        println!("{number}!");
    }
}
