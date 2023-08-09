fn main() {
    let number = 3;
    // normal if-else
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // handling multiple cases using else-if
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // variable assignment using if-else
    let condition = true;
    let number = if condition { 5 } else { 6 };
    // but the condition is both the return values should have the same data type.
    println!("The value of number is: {number}");
/*
Rust needs to know at compile time what type the number variable is, definitively. Knowing the type of number lets the compiler verify the type is valid everywhere we use number. Rust wouldn’t be able to do that if the type of number was only determined at runtime; the compiler would be more complex and would make fewer guarantees about the code if it had to keep track of multiple hypothetical types for any variable.
*/

    //loops
    // infinite loop by default
    // loop {
    //     println!("again!");
    // }
    let mut count = 0;
    loop{
        if count < 5{
            println!("hello");
            count += 1;
        }
        else{
            break;
        }
    }

    // passing values to variables using loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // while loop
    let mut number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    // for loop
    let a = [10, 20, 30, 40, 50];
    // direct element access
    for element in a {
        println!("the value is: {element}");
    }
    // loop using range
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}