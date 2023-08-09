fn main() {
    // scalar type: A scalar type represents a single value.
    // Integer
    let x: u8 = 4;
    let y: i8 = -21;
    // u8, u16, u32, u64, u128, usize
    // i8, i16, i32, i64, i128, isize

    // floating point types
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // operations on integers and float-->

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // Boolean Type
    let t = true;
    let f: bool = false;

    // character type
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);


    // compound types: Compound types can group multiple values into one type

    // tuple
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    // accessing each index in a tuple
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // Array Type: every element of an array must have the same type.
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // static typing
    // accessing array elements
    let first = a[0];
    let second = a[1];
    // to initialise all values of the array with the same value u can use:
    let a = [0;10];
    println!("a after initialisation with 0 {:?}", a);

    // user input int rust
    use std::io;

    println!("Enter a number: ");

    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Error in reading input");

    let n:u32 = line.trim().parse().expect("Error in parsing line");
    println!("u have entered the number {}", n);
}
